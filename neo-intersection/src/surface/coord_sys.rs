use glam::Vec3;
use neo_coordinate_system::CoordinateSystem;
use neo_ray::d3::def::Ray3D;
use neo_surface::surface::def::NeoSurface;

use crate::coord_sys::other_coord_sys::CoordSysCoordSysIntersection;
use crate::line_intersection_parts::Line3DIntersectionParts;
use crate::surface::ray::SurfaceRay3DIntersection;
use crate::trait_def::NeoIntersectable;

#[derive(Debug, Clone, PartialEq)]
pub enum SurfaceCoordSys3DIntersection {
    None,
    Point(Vec3),
    Parts(Vec<Line3DIntersectionParts>),
    Surface(NeoSurface),
}

impl NeoIntersectable<CoordinateSystem> for NeoSurface {
    type Output = SurfaceCoordSys3DIntersection;

    fn intersection(&self, rhs: &CoordinateSystem) -> Self::Output {
        let inter = self.coordinate_system.intersection(rhs);
        match inter {
            CoordSysCoordSysIntersection::Same => {
                SurfaceCoordSys3DIntersection::Surface(self.clone())
            }
            CoordSysCoordSysIntersection::Parallel { .. } => SurfaceCoordSys3DIntersection::None,
            CoordSysCoordSysIntersection::Ray(ray) => ray_intersection_case_analysis(self, ray),
        }
    }
}

fn ray_intersection_case_analysis(
    surface: &NeoSurface,
    ray: Ray3D,
) -> SurfaceCoordSys3DIntersection {
    let inter = surface.intersection(&ray);
    match inter {
        SurfaceRay3DIntersection::Skewed => SurfaceCoordSys3DIntersection::None,
        SurfaceRay3DIntersection::Parallel => SurfaceCoordSys3DIntersection::None,
        SurfaceRay3DIntersection::Point(p) => SurfaceCoordSys3DIntersection::Point(p),
        SurfaceRay3DIntersection::Parts(ps) => SurfaceCoordSys3DIntersection::Parts(ps),
    }
}

#[cfg(test)]
mod surface_coord_sys {
    use glam::{Vec2, Vec3};
    use neo_coordinate_system::CoordinateSystem;
    use neo_geo_glam_interop::to_geo::ConvertToGeo;
    use neo_plane::Plane;
    use neo_surface::surface::def::NeoSurface;

    use crate::line_intersection_parts::Line3DIntersectionParts;
    use crate::surface::coord_sys::SurfaceCoordSys3DIntersection;
    use crate::trait_def::NeoIntersectable;

    fn standard_surface(shape: geo::Polygon<f32>) -> NeoSurface {
        let local_x = Vec3::X + Vec3::Z;
        let local_y = -Vec3::X * 0.5 + Vec3::Y + Vec3::Z * 0.5;

        let p = Plane::from_local_axis(local_x, local_y);
        let c = CoordinateSystem::from_origin_and_plane(Vec3::ONE, p);
        let o = geo::Coord::<f32>::zero();

        NeoSurface::new(c, o, shape)
    }

    fn standard_surface_rect() -> NeoSurface {
        let shape = geo::Rect::<f32>::new(Vec2::NEG_ONE.to_geo(), Vec2::ONE.to_geo()).to_polygon();
        standard_surface(shape)
    }

    #[test]
    fn no_intersection_parallel_works() {
        let surface = standard_surface_rect();
        let local_x = Vec3::X + Vec3::Z;
        let local_y = -Vec3::X * 0.5 + Vec3::Y + Vec3::Z * 0.5;

        let p = Plane::from_local_axis(local_x, local_y);
        let normal = p.normal;
        let c = CoordinateSystem::from_origin_and_plane(Vec3::ONE + normal, p);

        let inter = surface.intersection(&c);
        assert_eq!(inter, SurfaceCoordSys3DIntersection::None);
    }

    #[test]
    fn no_intersection_coord_sys_translated_away_works() {
        let surface = standard_surface_rect();
        let local_x = Vec3::X + Vec3::Z;
        let local_y = -Vec3::X * 0.5 + Vec3::Y + Vec3::Z * 0.5;
        let normal = surface.coordinate_system.plane.normal;

        let p = Plane::from_local_axis(local_x, normal);
        let c = CoordinateSystem::from_origin_and_plane(Vec3::ONE + local_y * 10.0, p);

        let inter = surface.intersection(&c);
        assert_eq!(inter, SurfaceCoordSys3DIntersection::None);
    }

    #[test]
    fn line_intersection_works() {
        let surface = standard_surface_rect();
        let local_x = Vec3::X + Vec3::Z;
        let normal = surface.coordinate_system.plane.normal;

        let p = Plane::from_local_axis(local_x, normal);
        let c = CoordinateSystem::from_origin_and_plane(Vec3::ONE, p);

        let inter = surface.intersection(&c);
        match inter {
            SurfaceCoordSys3DIntersection::Parts(ps) => {
                assert_eq!(ps.len(), 1);
                assert!(matches!(ps[0], Line3DIntersectionParts::Line(_)));
            }
            _ => panic!("expected {inter:?} to be a single line"),
        }
    }

    #[test]
    fn surface_intersection_works() {
        let surface = standard_surface_rect();
        let c = surface.coordinate_system;

        let inter = surface.intersection(&c);
        assert_eq!(inter, SurfaceCoordSys3DIntersection::Surface(surface));
    }
}
