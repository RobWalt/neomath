use glam::Vec3;
use neo_coordinate_system::CoordinateSystem;
use neo_ray::d3::constants::NEO_LINE_RAY_3D_EPS;
use neo_ray::d3::def::Ray3D;

use crate::trait_def::NeoIntersectable;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RayCoordSys3DIntersection {
    Parallel,
    Point(Vec3),
    Ray(Ray3D),
}

impl NeoIntersectable<CoordinateSystem> for Ray3D {
    type Output = RayCoordSys3DIntersection;
    fn intersection(&self, rhs: &CoordinateSystem) -> Self::Output {
        if is_ray_on_coord_sys(self, rhs) {
            RayCoordSys3DIntersection::Ray(*self)
        } else {
            classify_intersection_cases(self, rhs)
        }
    }
}

pub(crate) fn is_ray_on_coord_sys(ray: &Ray3D, coordinate_system: &CoordinateSystem) -> bool {
    coordinate_system.is_point_in_coordinate_system(ray.origin)
        && coordinate_system.is_point_in_coordinate_system(ray.origin + ray.direction)
}

pub(crate) fn classify_intersection_cases(
    ray: &Ray3D,
    coordinate_system: &CoordinateSystem,
) -> RayCoordSys3DIntersection {
    if ray.direction.dot(coordinate_system.plane.normal) < NEO_LINE_RAY_3D_EPS {
        RayCoordSys3DIntersection::Parallel
    } else {
        calculate_intersection_point_with_plane(ray, coordinate_system)
    }
}

// https://stackoverflow.com/questions/5666222/3d-line-plane-intersection
pub(crate) fn calculate_intersection_point_with_plane(
    ray: &Ray3D,
    coordinate_system: &CoordinateSystem,
) -> RayCoordSys3DIntersection {
    // won't be 0 because of the previous branches catch that case
    let dot = ray.direction.dot(coordinate_system.plane.normal);
    let w = ray.origin - coordinate_system.origin;
    let fac = -coordinate_system.plane.normal.dot(w) / dot;
    RayCoordSys3DIntersection::Point(ray.origin + ray.direction * fac)
}

#[test]
fn parallel_to_coordinate_system_works() {
    let c = CoordinateSystem::from_origin_and_normal(Vec3::ZERO, Vec3::Y);
    let ray = Ray3D::new(Vec3::Y, Vec3::ONE - Vec3::Y);
    assert_eq!(ray.intersection(&c), RayCoordSys3DIntersection::Parallel);
}

#[test]
fn ray_in_coordinate_system_works() {
    let c = CoordinateSystem::from_origin_and_normal(Vec3::ZERO, Vec3::Y);
    let ray = Ray3D::new(Vec3::ONE - Vec3::Y, Vec3::ONE - Vec3::Y);
    assert_eq!(ray.intersection(&c), RayCoordSys3DIntersection::Ray(ray));
}

#[test]
fn intersection_in_coordinate_system_works() {
    let c = CoordinateSystem::from_origin_and_normal(Vec3::ZERO, Vec3::Y);
    let ray = Ray3D::new(Vec3::ONE - Vec3::Y, Vec3::ONE);
    assert_eq!(
        ray.intersection(&c),
        RayCoordSys3DIntersection::Point(Vec3::ONE - Vec3::Y)
    );
}

#[test]
fn ray_not_on_coordinate_system_works() {
    let dir = Vec3::Y * 0.43 + Vec3::X * 34.0;
    let c = CoordinateSystem::from_origin_and_normal(Vec3::ONE, dir);
    let r = Ray3D::new(Vec3::ONE, dir);
    assert!(!is_ray_on_coord_sys(&r, &c))
}

#[test]
fn ray_dst_not_on_coordinate_system_works() {
    let dir = Vec3::Y * 0.43 + Vec3::X * 34.0;
    let r = Ray3D::new(Vec3::ONE, dir);
    let c = CoordinateSystem::from_origin_and_normal(Vec3::ONE, dir).offset_origin_by(dir);
    assert!(!is_ray_on_coord_sys(&r, &c))
}
