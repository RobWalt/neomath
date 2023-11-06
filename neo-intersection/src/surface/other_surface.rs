use geo::{MapCoords, SpadeBoolops};
use glam::Vec3;
use neo_surface::surface::def::NeoSurface;

use crate::line_intersection_parts::Line3DIntersectionParts;
use crate::surface::coord_sys::SurfaceCoordSys3DIntersection;
use crate::trait_def::NeoIntersectable;

#[derive(Debug, Clone, PartialEq)]
pub enum SurfaceSurface3DIntersection {
    None,
    Point(Vec3),
    Parts(Vec<Line3DIntersectionParts>),
    Surface(NeoSurface),
    MultiSurface(Vec<NeoSurface>),
}

impl NeoIntersectable for NeoSurface {
    type Output = SurfaceSurface3DIntersection;

    fn intersection(&self, rhs: &Self) -> Self::Output {
        let inter = self.intersection(&rhs.coordinate_system);
        match inter {
            SurfaceCoordSys3DIntersection::None => SurfaceSurface3DIntersection::None,
            // the next two imply that the intersection is happening because of coordinate systems
            // which are not parallel so we can just pass them through
            SurfaceCoordSys3DIntersection::Point(p) => SurfaceSurface3DIntersection::Point(p),
            SurfaceCoordSys3DIntersection::Parts(ps) => SurfaceSurface3DIntersection::Parts(ps),
            SurfaceCoordSys3DIntersection::Surface(_) => {
                let intersection = surface_intersection_case_analysis(self, rhs);
                match intersection.len() {
                    0 => SurfaceSurface3DIntersection::None,
                    1 => SurfaceSurface3DIntersection::Surface(intersection[0].clone()),
                    _ => SurfaceSurface3DIntersection::MultiSurface(intersection),
                }
            }
        }
    }
}

fn surface_intersection_case_analysis(surface: &NeoSurface, rhs: &NeoSurface) -> Vec<NeoSurface> {
    // we are working in the coordinate system of the `surface` argument
    let origin_2d_diff = surface.shape_origin - rhs.shape_origin;
    let rhs_translated_shape = rhs.shape.map_coords(|c| c + origin_2d_diff);
    let intersection = geo::Polygon::intersection(&surface.shape, &rhs_translated_shape);

    // if it fails, we just return no intersection at all which might be wrong
    intersection
        .into_iter()
        .flat_map(|mp| mp.into_iter())
        .map(|p| NeoSurface {
            shape: p,
            ..surface.clone()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod surface_surface_intersection {
    use neo_coordinate_system::CoordinateSystem;

    use super::*;

    #[test]
    fn simple_intersection() {
        let coord_sys =
            CoordinateSystem::from_origin_and_normal(Vec3::ZERO, Vec3::X + Vec3::Y + Vec3::Z);
        let r1 = geo::Rect::new(geo::Coord::zero(), geo::Coord { x: 2.0, y: 2.0 });
        let r2 = geo::Rect::new(geo::Coord { x: 1.0, y: 1.0 }, geo::Coord { x: 3.0, y: 3.0 });
        let s1 = NeoSurface::new_origin_at_zero(coord_sys, r1.to_polygon());
        let s2 = NeoSurface::new_origin_at_zero(coord_sys, r2.to_polygon());

        let intersection = s1.intersection(&s2);

        assert!(matches!(
            intersection,
            SurfaceSurface3DIntersection::Surface(_)
        ));
    }

    #[test]
    fn multi_intersection() {
        let coord_sys =
            CoordinateSystem::from_origin_and_normal(Vec3::ZERO, Vec3::X + Vec3::Y + Vec3::Z);
        let r1 = geo::Polygon::new(
            geo::LineString::new(
                [
                    (0.0, 0.0),
                    (1.0, 0.0),
                    (1.0, 1.0),
                    (2.0, 1.0),
                    (2.0, 0.0),
                    (3.0, 0.0),
                    (3.0, 3.0),
                    (0.0, 3.0),
                ]
                .map(geo::Coord::from)
                .to_vec(),
            ),
            vec![],
        );
        let r2 = geo::Polygon::new(
            geo::LineString::new(
                [
                    (0.0, 1.0),
                    (1.0, 1.0),
                    (1.0, 0.0),
                    (2.0, 0.0),
                    (2.0, 1.0),
                    (3.0, 1.0),
                    (3.0, -2.0),
                    (0.0, -2.0),
                ]
                .map(geo::Coord::from)
                .to_vec(),
            ),
            vec![],
        );
        let s1 = NeoSurface::new_origin_at_zero(coord_sys, r1);
        let s2 = NeoSurface::new_origin_at_zero(coord_sys, r2);

        let intersection = s1.intersection(&s2);

        assert!(matches!(
            intersection,
            SurfaceSurface3DIntersection::MultiSurface(_)
        ));
    }
}
