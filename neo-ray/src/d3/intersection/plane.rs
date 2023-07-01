use glam::Vec3;
use neo_coordinate_system::CoordinateSystem;

use crate::d3::constants::NEO_LINE_RAY_3D_EPS;
use crate::d3::def::Ray3D;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RayCoordSysIntersection {
    Parallel,
    Point(Vec3),
    Ray(Ray3D),
}

impl Ray3D {
    pub fn intersection_plane(
        &self,
        coordinate_system: &CoordinateSystem,
    ) -> RayCoordSysIntersection {
        let second_point = self.origin + self.direction;
        if coordinate_system.is_point_in_coordinate_system(self.origin)
            && coordinate_system.is_point_in_coordinate_system(second_point)
        {
            RayCoordSysIntersection::Ray(*self)
        } else {
            self.classify_intersection_cases(coordinate_system)
        }
    }

    fn classify_intersection_cases(
        &self,
        coordinate_system: &CoordinateSystem,
    ) -> RayCoordSysIntersection {
        if self.direction.dot(coordinate_system.normal()) < NEO_LINE_RAY_3D_EPS {
            RayCoordSysIntersection::Parallel
        } else {
            self.calculate_intersection_point_with_plane(coordinate_system)
        }
    }

    // https://stackoverflow.com/questions/5666222/3d-line-plane-intersection
    fn calculate_intersection_point_with_plane(
        &self,
        coordinate_system: &CoordinateSystem,
    ) -> RayCoordSysIntersection {
        // won't be 0 because of the previous branches catch that case
        let dot = self.direction.dot(coordinate_system.normal());
        let w = self.origin - coordinate_system.origin();
        let fac = -coordinate_system.normal().dot(w) / dot;
        RayCoordSysIntersection::Point(self.origin + self.direction * fac)
    }
}

#[test]
fn parallel_to_coordinate_system_works() {
    let c = CoordinateSystem::from_origin_and_normal(Vec3::ZERO, Vec3::Y);
    let ray = Ray3D::new(Vec3::Y, Vec3::ONE - Vec3::Y);
    assert_eq!(
        ray.intersection_plane(&c),
        RayCoordSysIntersection::Parallel
    );
}

#[test]
fn ray_in_coordinate_system_works() {
    let c = CoordinateSystem::from_origin_and_normal(Vec3::ZERO, Vec3::Y);
    let ray = Ray3D::new(Vec3::ONE - Vec3::Y, Vec3::ONE - Vec3::Y);
    assert_eq!(
        ray.intersection_plane(&c),
        RayCoordSysIntersection::Ray(ray)
    );
}

#[test]
fn intersection_in_coordinate_system_works() {
    let c = CoordinateSystem::from_origin_and_normal(Vec3::ZERO, Vec3::Y);
    let ray = Ray3D::new(Vec3::ONE - Vec3::Y, Vec3::ONE);
    assert_eq!(
        ray.intersection_plane(&c),
        RayCoordSysIntersection::Point(Vec3::ONE - Vec3::Y)
    );
}
