use glam::Vec3;

use crate::d3::constants::NEO_LINE_SEGMENT_3D_EPS;
use crate::d3::def::LineSegment3D;

impl LineSegment3D {
    pub fn is_degenerated(&self) -> bool {
        self.src == self.dst
    }

    pub fn is_point_on_line(&self, point: Vec3) -> bool {
        self.distance_to_point(point) < NEO_LINE_SEGMENT_3D_EPS
    }

    pub fn is_parallel_to(&self, other: &Self) -> bool {
        self.ray().is_parallel_to(&other.ray())
    }

    pub fn is_endpoint(&self, point: Vec3) -> bool {
        self.src == point || self.dst == point
    }
}

#[test]
fn is_point_on_line_endpoint_works() {
    let p = Vec3::X;
    let l = LineSegment3D::UNIT_X;
    assert!(
        l.is_point_on_line(p),
        "{l:?}, {p:?}, {}",
        l.distance_to_point(p)
    );
}

#[test]
fn is_point_on_line_epsilon_works() {
    let p = Vec3::X * (1.0 + f32::EPSILON);
    let l = LineSegment3D::UNIT_X;
    assert!(
        l.is_point_on_line(p),
        "{l:?}, {p:?}, {}",
        l.distance_to_point(p)
    );
}

#[test]
fn is_point_on_line_tilted_works() {
    let l = LineSegment3D::new(Vec3::ZERO, Vec3::new(1.0, 0.33, 0.33));
    let p = l.src + l.direction() * 0.33;
    assert!(
        l.is_point_on_line(p),
        "{l:?}, {p:?}, {}",
        l.distance_to_point(p)
    );
}
