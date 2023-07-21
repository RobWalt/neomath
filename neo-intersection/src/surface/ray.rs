use glam::Vec3;
use neo_geo_glam_interop::to_glam::ConvertToGlam;
use neo_line_segment::d3::def::LineSegment3D;
use neo_ray::d2::def::Ray2D;
use neo_ray::d3::def::Ray3D;
use neo_surface::surface::def::NeoSurface;

use crate::ray2d::polygon::RayPolygon2DIntersectionPart;
use crate::results::RayCoordSys3DIntersection;
use crate::trait_def::NeoIntersectable;

#[derive(Debug, Clone, PartialEq)]
pub enum SurfaceRay3DIntersection {
    Skewed,
    Parallel,
    Point(Vec3),
    Parts(Vec<SurfaceRay3DIntersectionParts>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SurfaceRay3DIntersectionParts {
    Point(Vec3),
    Line(LineSegment3D),
}

impl NeoIntersectable<Ray3D> for NeoSurface {
    type Output = SurfaceRay3DIntersection;
    fn intersection(&self, rhs: &Ray3D) -> Self::Output {
        match self.coordinate_system.intersection(rhs) {
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
    let ray2d = Ray2D::new(offset_ray_origin, offset_ray_direction);
    ray2d
}

fn contained_ray_case_analysis(surface: &NeoSurface, ray: Ray3D) -> SurfaceRay3DIntersection {
    let ray2d = project_ray(surface, &ray);
    let inter_parts = ray2d.intersection(&surface.shape).list_parts();
    if inter_parts.is_empty() {
        SurfaceRay3DIntersection::Skewed
    } else {
        let injection_func = surface.injection_function();

        let inject_part_func = move |part| match part {
            RayPolygon2DIntersectionPart::Point(p) => {
                SurfaceRay3DIntersectionParts::Point(injection_func(p))
            }
            RayPolygon2DIntersectionPart::Line(l) => {
                let src = injection_func(l.src);
                let dst = injection_func(l.dst);
                let line3d = LineSegment3D::new(src, dst);
                SurfaceRay3DIntersectionParts::Line(line3d)
            }
        };

        let injected_parts = inter_parts
            .into_iter()
            .map(inject_part_func)
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

    use crate::surface::ray::{SurfaceRay3DIntersection, SurfaceRay3DIntersectionParts};
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
                assert!(matches!(parts[0], SurfaceRay3DIntersectionParts::Line(_)));
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
                assert!(matches!(parts[0], SurfaceRay3DIntersectionParts::Point(_)));
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
}
