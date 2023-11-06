use glam::Vec3;

use crate::d3::constants::NEO_LINE_RAY_3D_EPS;
use crate::d3::def::Ray3D;

impl Ray3D {
    pub fn is_degenerated(&self) -> bool {
        self.direction == Vec3::ZERO
    }

    pub fn is_point_on_ray(&self, point: Vec3) -> bool {
        self.distance_to_point(point) < NEO_LINE_RAY_3D_EPS
    }

    pub fn is_parallel_to(&self, other: &Self) -> bool {
        self.direction()
            .cross(other.direction())
            .abs_diff_eq(Vec3::ZERO, NEO_LINE_RAY_3D_EPS)
    }
}

#[test]
fn is_point_on_line_works_endpoint() {
    let p = Vec3::X;
    let l = Ray3D::X;
    assert!(
        l.is_point_on_ray(p),
        "{l:?}, {p:?}, {}",
        l.distance_to_point(p)
    );
}

#[test]
fn is_point_on_line_works_epsilon() {
    let p = Vec3::X * (1.0 + f32::EPSILON);
    let l = Ray3D::X;
    assert!(
        l.is_point_on_ray(p),
        "{l:?}, {p:?}, {}",
        l.distance_to_point(p)
    );
}

#[test]
fn is_point_on_line_works_tilted() {
    let l = Ray3D::new(Vec3::ZERO, Vec3::new(1.0, 0.33, 1.0));
    let p = l.origin + l.direction() * 0.33;
    assert!(
        l.is_point_on_ray(p),
        "{l:?}, {p:?}, {}",
        l.distance_to_point(p)
    );
}
