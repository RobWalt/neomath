use neo_coordinate_system::{CoordinateSystem, COORDINATE_SYSTEM_EPS};
use neo_ray::d3::def::Ray3D;

use crate::results::{PlanePlaneIntersection, RayCoordSys3DIntersection};
use crate::trait_def::NeoIntersectable;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoordSysCoordSysIntersection {
    Same,
    Parallel { distance: f32 },
    Ray(Ray3D),
}

impl NeoIntersectable for CoordinateSystem {
    type Output = CoordSysCoordSysIntersection;
    fn intersection(&self, rhs: &Self) -> Self::Output {
        let inter = self.plane.intersection(&rhs.plane);
        match inter {
            PlanePlaneIntersection::Same => classify_parallel_coordinate_systems(self, rhs),
            PlanePlaneIntersection::Ray(plane_intersection_ray) => {
                classify_intersection_ray(self, rhs, plane_intersection_ray)
            }
        }
    }
}

pub(crate) fn classify_parallel_coordinate_systems(
    c1: &CoordinateSystem,
    c2: &CoordinateSystem,
) -> CoordSysCoordSysIntersection {
    let normal_ray = Ray3D::new(c1.origin, c1.plane.normal);
    let intersection = normal_ray.intersection(c2);
    let RayCoordSys3DIntersection::Point(point_in_other) = intersection else {
        unreachable!("the normal ray of parallel coords intersect the other coordinate system\n\n{normal_ray:?}\n\n{c1:?}\n{c2:?}\n\n{intersection:?}");
    };
    let distance = c1.origin.distance(point_in_other);
    if distance < COORDINATE_SYSTEM_EPS {
        CoordSysCoordSysIntersection::Same
    } else {
        CoordSysCoordSysIntersection::Parallel { distance }
    }
}

pub(crate) fn classify_intersection_ray(
    c1: &CoordinateSystem,
    c2: &CoordinateSystem,
    plane_intersection_ray: Ray3D,
) -> CoordSysCoordSysIntersection {
    let orthogonal_dir_in_c1 = plane_intersection_ray.direction.cross(c1.plane.normal);
    let non_parallel_ray_in_plane = Ray3D::new(c1.origin, orthogonal_dir_in_c1);
    let intersection = non_parallel_ray_in_plane.intersection(c2);
    let RayCoordSys3DIntersection::Point(point_in_other) = intersection else {
        unreachable!("the normal ray of parallel coords intersect the other coordinate system\n\n{non_parallel_ray_in_plane:?}\n\n{c2:?}\n\n{intersection:?}");
    };
    CoordSysCoordSysIntersection::Ray(Ray3D::new(point_in_other, plane_intersection_ray.direction))
}

#[test]
fn same_works() {
    use glam::Vec3;
    let c = CoordinateSystem::from_origin_and_normal(Vec3::ONE, Vec3::Y * 0.43 + Vec3::X * 34.0);
    assert_eq!(c.intersection(&c), CoordSysCoordSysIntersection::Same);
}

#[test]
fn same_eps_diff_works() {
    use glam::Vec3;
    let c1 = CoordinateSystem::from_origin_and_normal(Vec3::ONE, Vec3::Y * 0.43 + Vec3::X * 34.0);
    let c2 = c1.offset_origin_by(Vec3::Z * f32::EPSILON);
    assert_eq!(c1.intersection(&c2), CoordSysCoordSysIntersection::Same);
}

#[test]
fn parallel_works() {
    use glam::Vec3;
    let c1 = CoordinateSystem::from_origin_and_normal(Vec3::ONE, Vec3::Y * 0.43 + Vec3::X * 34.0);
    let c2 = c1.offset_origin_by(c1.plane.normal);
    assert_eq!(
        c1.intersection(&c2),
        CoordSysCoordSysIntersection::Parallel {
            distance: c1.plane.normal.length()
        }
    );
}

#[test]
fn ray_intersection_works() {
    use glam::Vec3;
    let c1 = CoordinateSystem::from_origin_and_normal(Vec3::Y, Vec3::Y);
    let c2 = CoordinateSystem::from_origin_and_normal(Vec3::X, Vec3::X);
    let intersection_ray = c1.intersection(&c2);
    let CoordSysCoordSysIntersection::Ray(ray) = intersection_ray else {
        panic!("expected ray");
    };
    assert!(c2.is_point_in_coordinate_system(ray.origin));
    assert!(c2.is_point_in_coordinate_system(ray.origin + ray.direction));
}
