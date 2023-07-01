use glam::Vec3;
use neo_coordinate_system::CoordinateSystem;

use crate::d3::constants::NEO_LINE_RAY_3D_EPS;
use crate::d3::def::NeoLineRay3D;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NeoRayCoordinateIntersection {
    Parallel,
    Point(Vec3),
    Ray(NeoLineRay3D),
}

impl NeoLineRay3D {
    pub fn intersection_plane(
        &self,
        coordinate_system: &CoordinateSystem,
    ) -> NeoRayCoordinateIntersection {
        let second_point = self.origin + self.direction;
        if coordinate_system.is_point_in_coordinate_system(self.origin)
            && coordinate_system.is_point_in_coordinate_system(second_point)
        {
            NeoRayCoordinateIntersection::Ray(*self)
        } else {
            self.classify_intersection_cases(coordinate_system)
        }
    }

    fn classify_intersection_cases(
        &self,
        coordinate_system: &CoordinateSystem,
    ) -> NeoRayCoordinateIntersection {
        if self.direction.dot(coordinate_system.normal()) < NEO_LINE_RAY_3D_EPS {
            NeoRayCoordinateIntersection::Parallel
        } else {
            self.calculate_intersection_point_with_plane(coordinate_system)
        }
    }

    // https://stackoverflow.com/questions/5666222/3d-line-plane-intersection
    fn calculate_intersection_point_with_plane(
        &self,
        coordinate_system: &CoordinateSystem,
    ) -> NeoRayCoordinateIntersection {
        // won't be 0 because of the previous branches catch that case
        let dot = self.direction.dot(coordinate_system.normal());
        let w = self.origin - coordinate_system.origin();
        let fac = -coordinate_system.normal().dot(w) / dot;
        NeoRayCoordinateIntersection::Point(self.origin + self.direction * fac)
    }
}

#[test]
fn parallel_to_coordinate_system_works() {
    let c = CoordinateSystem::from_origin_and_normal(Vec3::ZERO, Vec3::Y);
    let ray = NeoLineRay3D::new(Vec3::Y, Vec3::ONE - Vec3::Y);
    assert_eq!(
        ray.intersection_plane(&c),
        NeoRayCoordinateIntersection::Parallel
    );
}

#[test]
fn ray_in_coordinate_system_works() {
    let c = CoordinateSystem::from_origin_and_normal(Vec3::ZERO, Vec3::Y);
    let ray = NeoLineRay3D::new(Vec3::ONE - Vec3::Y, Vec3::ONE - Vec3::Y);
    assert_eq!(
        ray.intersection_plane(&c),
        NeoRayCoordinateIntersection::Ray(ray)
    );
}

#[test]
fn intersection_in_coordinate_system_works() {
    let c = CoordinateSystem::from_origin_and_normal(Vec3::ZERO, Vec3::Y);
    let ray = NeoLineRay3D::new(Vec3::ONE - Vec3::Y, Vec3::ONE);
    assert_eq!(
        ray.intersection_plane(&c),
        NeoRayCoordinateIntersection::Point(Vec3::ONE - Vec3::Y)
    );
}
