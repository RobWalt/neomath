use glam::Vec2;

use crate::d2::constants::NEO_LINE_RAY_2D_EPS;
use crate::d2::def::Ray2D;

impl Ray2D {
    pub fn is_degenerated(&self) -> bool {
        self.direction == Vec2::ZERO
    }

    pub fn is_point_on_ray(&self, point: Vec2) -> bool {
        self.distance_to_point(point) < NEO_LINE_RAY_2D_EPS
    }

    pub fn is_parallel_to(&self, other: &Self) -> bool {
        self.direction().perp_dot(other.direction()).abs() < NEO_LINE_RAY_2D_EPS
    }
}

#[test]
fn is_point_on_line_works_endpoint() {
    let p = Vec2::X;
    let l = Ray2D::X;
    assert_eq!(
        l.is_point_on_ray(p),
        true,
        "{l:?}, {p:?}, {}",
        l.distance_to_point(p)
    );
}

#[test]
fn is_point_on_line_works_epsilon() {
    let p = Vec2::X * (1.0 + f32::EPSILON);
    let l = Ray2D::X;
    assert_eq!(
        l.is_point_on_ray(p),
        true,
        "{l:?}, {p:?}, {}",
        l.distance_to_point(p)
    );
}

#[test]
fn is_point_on_line_works_tilted() {
    let l = Ray2D::new(Vec2::ZERO, Vec2::new(1.0, 0.33));
    let p = l.origin + l.direction() * 0.33;
    assert_eq!(
        l.is_point_on_ray(p),
        true,
        "{l:?}, {p:?}, {}",
        l.distance_to_point(p)
    );
}
