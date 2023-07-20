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
