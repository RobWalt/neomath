use glam::Vec2;

use crate::d2::constants::NEO_LINE_SEGMENT_2D_EPS;
use crate::d2::def::LineSegment2D;

impl LineSegment2D {
    pub fn is_degenerated(&self) -> bool {
        self.src == self.dst
    }

    pub fn is_point_on_line(&self, point: Vec2) -> bool {
        self.distance_to_point(point) < NEO_LINE_SEGMENT_2D_EPS
    }

    pub fn is_parallel_to(&self, other: &Self) -> bool {
        self.ray().is_parallel_to(&other.ray())
    }

    pub fn is_endpoint(&self, point: Vec2) -> bool {
        self.src == point || self.dst == point
    }
}

#[test]
fn is_point_on_line_works() {
    let p = Vec2::X;
    let l = LineSegment2D::UNIT_X.scale_dst_by(2.0);
    assert_eq!(
        l.is_point_on_line(p),
        true,
        "{l:?}, {p:?}, {}",
        l.distance_to_point(p)
    );
}

#[test]
fn is_point_on_line_endpoint_works() {
    let p = Vec2::X;
    let l = LineSegment2D::UNIT_X;
    assert_eq!(
        l.is_point_on_line(p),
        true,
        "{l:?}, {p:?}, {}",
        l.distance_to_point(p)
    );
}

#[test]
fn is_point_on_line_epsilon_works() {
    let p = Vec2::X * (1.0 + f32::EPSILON);
    let l = LineSegment2D::UNIT_X;
    assert_eq!(
        l.is_point_on_line(p),
        true,
        "{l:?}, {p:?}, {}",
        l.distance_to_point(p)
    );
}

#[test]
fn is_point_on_line_tilted_works() {
    let l = LineSegment2D::new(Vec2::ZERO, Vec2::new(1.0, 0.33));
    let p = l.src + l.direction() * 0.33;
    assert_eq!(
        l.is_point_on_line(p),
        true,
        "{l:?}, {p:?}, {}",
        l.distance_to_point(p)
    );
}
