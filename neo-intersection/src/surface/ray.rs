use glam::Vec3;
use neo_geo_glam_interop::to_glam::ConvertToGlam;
use neo_ray::d2::def::Ray2D;
use neo_ray::d3::def::Ray3D;
use neo_surface::surface::def::NeoSurface;

use crate::line_intersection_parts::Line3DIntersectionParts;
use crate::results::RayCoordSys3DIntersection;
use crate::trait_def::NeoIntersectable;

#[derive(Debug, Clone, PartialEq)]
pub enum SurfaceRay3DIntersection {
    Skewed,
    Parallel,
    Point(Vec3),
    Parts(Vec<Line3DIntersectionParts>),
}

impl NeoIntersectable<Ray3D> for NeoSurface {
    type Output = SurfaceRay3DIntersection;
    fn intersection(&self, rhs: &Ray3D) -> Self::Output {
        let inter = self.coordinate_system.intersection(rhs);
        match inter {
            RayCoordSys3DIntersection::Parallel => SurfaceRay3DIntersection::Parallel,
            RayCoordSys3DIntersection::Point(p) => point_case_analysis(self, p),
            RayCoordSys3DIntersection::Ray(ray) => contained_ray_case_analysis(self, ray),
        }
    }
}

fn point_case_analysis(surface: &NeoSurface, point: Vec3) -> SurfaceRay3DIntersection {
    if surface.is_point_in_surface(point) {
        SurfaceRay3DIntersection::Point(point)
    } else {
        SurfaceRay3DIntersection::Skewed
    }
}

fn project_ray(surface: &NeoSurface, ray: &Ray3D) -> Ray2D {
    let offset_ray_origin = surface.project_point_xy(ray.origin).to_glam();
    let offset_ray_direction = surface.project_point_xy(ray.direction).to_glam();
    Ray2D::new(offset_ray_origin, offset_ray_direction)
}

fn contained_ray_case_analysis(surface: &NeoSurface, ray: Ray3D) -> SurfaceRay3DIntersection {
    let ray2d = project_ray(surface, &ray);
    let inter_parts = ray2d.intersection(&surface.shape).list_parts();
    if inter_parts.is_empty() {
        SurfaceRay3DIntersection::Skewed
    } else {
        let injection_func = surface.injection_function();

        let injected_parts = inter_parts
            .into_iter()
            .map(|part| part.inject_with(&injection_func))
            .collect::<Vec<_>>();

        SurfaceRay3DIntersection::Parts(injected_parts)
    }
}

#[cfg(test)]
mod surface_ray {
    use glam::{Vec2, Vec3};
    use neo_coordinate_system::CoordinateSystem;
    use neo_geo_glam_interop::to_geo::ConvertToGeo;
    use neo_plane::Plane;
    use neo_ray::d3::def::Ray3D;
    use neo_surface::surface::def::NeoSurface;

    use crate::surface::ray::{Line3DIntersectionParts, SurfaceRay3DIntersection};
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

    fn standard_surface_rect_with_hole() -> NeoSurface {
        let ext = geo::Rect::<f32>::new((2.0 * Vec2::NEG_ONE).to_geo(), (2.0 * Vec2::ONE).to_geo())
            .to_polygon()
            .exterior()
            .clone();
        let int = geo::Rect::<f32>::new(Vec2::NEG_ONE.to_geo(), Vec2::ONE.to_geo())
            .to_polygon()
            .exterior()
            .clone();
        standard_surface(geo::Polygon::<f32>::new(ext, vec![int]))
    }

    #[test]
    fn point_intersection_works() {
        let surface = standard_surface_rect();
        let ray = Ray3D::Z.offset_origin_by(Vec3::ONE);

        assert_eq!(
            surface.intersection(&ray),
            SurfaceRay3DIntersection::Point(Vec3::ONE)
        )
    }

    #[test]
    fn point_intersection_border_works() {
        let surface = {
            let p = Plane::from_local_axis(Vec3::X, Vec3::Y);
            let c = CoordinateSystem::from_origin_and_plane(Vec3::ONE, p);
            let o = geo::Coord::<f32>::zero();
            let shape = geo::Rect::<f32>::new(
                geo::Coord { x: -1.0, y: -1.0 },
                geo::Coord { x: 1.0, y: 1.0 },
            )
            .to_polygon();

            NeoSurface::new(c, o, shape)
        };
        let ray = Ray3D::Z.offset_origin_by(Vec3::ONE);

        // on border
        assert_eq!(
            surface.intersection(&ray.offset_origin_by(Vec3::X)),
            SurfaceRay3DIntersection::Point(Vec3::ONE + Vec3::X)
        );

        // just outside
        assert_eq!(
            surface.intersection(&ray.offset_origin_by(Vec3::X * 1.00001)),
            SurfaceRay3DIntersection::Skewed
        );
    }

    #[test]
    fn skewed_works() {
        let surface = standard_surface_rect();
        let ray = Ray3D::Z.offset_origin_by(Vec3::ONE * 3.0);

        assert_eq!(surface.intersection(&ray), SurfaceRay3DIntersection::Skewed)
    }

    #[test]
    fn ray_in_surface_one_intersection_line_works() {
        let surface = standard_surface_rect();
        let local_x = surface.coordinate_system.plane.local_x;
        let local_y = surface.coordinate_system.plane.local_y;
        let ray = Ray3D::new(Vec3::ONE, local_x + local_y);

        let inter = surface.intersection(&ray);
        match inter {
            SurfaceRay3DIntersection::Parts(parts) => {
                assert_eq!(parts.len(), 1);
                assert!(matches!(parts[0], Line3DIntersectionParts::Line(_)));
            }
            _ => panic!("expected {inter:?} to be intersection parts containing one line"),
        }
    }

    #[test]
    fn ray_in_surface_one_intersection_point_works() {
        let surface = standard_surface_rect();
        let local_x = surface.coordinate_system.plane.local_x;
        let local_y = surface.coordinate_system.plane.local_y;
        let ray = Ray3D::new(Vec3::ONE + local_x + local_y, local_x + local_y);

        let inter = surface.intersection(&ray);
        match inter {
            SurfaceRay3DIntersection::Parts(parts) => {
                assert_eq!(parts.len(), 1);
                assert!(matches!(parts[0], Line3DIntersectionParts::Point(_)));
            }
            _ => panic!("expected {inter:?} to be intersection parts containing one point"),
        }
    }

    #[test]
    fn ray_in_surface_no_intersection_works() {
        let surface = standard_surface_rect();
        let local_x = surface.coordinate_system.plane.local_x;
        let local_y = surface.coordinate_system.plane.local_y;
        let ray = Ray3D::new(Vec3::ONE + local_x * 3.0, local_x + local_y);

        let inter = surface.intersection(&ray);
        assert_eq!(inter, SurfaceRay3DIntersection::Skewed);
    }

    #[test]
    fn ray_in_surface_two_intersection_lines_works() {
        let surface = standard_surface_rect_with_hole();
        let local_x = surface.coordinate_system.plane.local_x;
        let local_y = surface.coordinate_system.plane.local_y;
        let ray = Ray3D::new(Vec3::ONE, local_x + local_y);

        let inter = surface.intersection(&ray);
        match inter {
            SurfaceRay3DIntersection::Parts(parts) => {
                assert_eq!(parts.len(), 2);
                assert!(matches!(parts[0], Line3DIntersectionParts::Line(_)));
                assert!(matches!(parts[1], Line3DIntersectionParts::Line(_)));
            }
            _ => panic!("expected {inter:?} to be intersection parts containing two lines"),
        }
    }
}
