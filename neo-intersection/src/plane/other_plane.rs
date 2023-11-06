use glam::Vec3;
use neo_plane::Plane;
use neo_ray::d3::def::Ray3D;

use crate::trait_def::NeoIntersectable;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlanePlaneIntersection {
    Same,
    Ray(Ray3D),
}

impl NeoIntersectable for Plane {
    type Output = PlanePlaneIntersection;

    fn intersection(&self, rhs: &Self) -> Self::Output {
        if self == rhs {
            PlanePlaneIntersection::Same
        } else {
            PlanePlaneIntersection::Ray(Ray3D::new(Vec3::ZERO, self.normal.cross(rhs.normal)))
        }
    }
}

#[test]
fn same_works() {
    let p = Plane::from_normal(Vec3::ONE);
    assert_eq!(p.intersection(&p), PlanePlaneIntersection::Same);
}

#[test]
fn same_eps_diff_works() {
    use glam::Quat;
    let p1 = Plane::from_normal(Vec3::ONE);
    let p2 = Plane::from_normal(Quat::from_axis_angle(Vec3::X, f32::EPSILON) * Vec3::ONE);
    assert_eq!(p1.intersection(&p2), PlanePlaneIntersection::Same);
}

#[test]
fn ray_intersection_works() {
    let p1 = Plane::from_normal(Vec3::X);
    let p2 = Plane::from_normal(Vec3::Y);
    assert_eq!(p1.intersection(&p2), PlanePlaneIntersection::Ray(Ray3D::Z));
}
