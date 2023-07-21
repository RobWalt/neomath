use glam::Vec2;

use crate::d2::def::LineSegment2D;

impl LineSegment2D {
    pub fn project_point(&self, point: Vec2) -> Vec2 {
        point.project_onto(self.direction())
    }

    pub fn scalar_of(&self, point: Vec2) -> f32 {
        self.project_point(point - self.src).length() / self.direction().length()
    }

    /// Inspired by the SDF formula of a line
    /// https://www.youtube.com/watch?v=PMltMdi1Wzg
    pub fn distance_to_point(&self, point: Vec2) -> f32 {
        let pa = point - self.src;
        let ba = self.direction();
        let prod = pa.dot(ba) / ba.length_squared();
        let h = prod.clamp(0.0, 1.0);
        let dist = pa.distance(h * ba);
        dist
    }

    pub fn inject_scalar(&self, scalar: f32) -> Vec2 {
        self.src + scalar * self.direction()
    }
}

#[test]
fn projection_works() {
    let p = Vec2::Y;
    let l = LineSegment2D::UNIT_ONE;
    assert_eq!(l.project_point(p), Vec2::ONE * 0.5);
}

#[test]
fn scalar_of_works_half() {
    let p = Vec2::ONE;
    let l = LineSegment2D::UNIT_X.scale_dst_by(2.0);
    assert_eq!(l.scalar_of(p), 0.5);
}

#[test]
fn scalar_of_works_two_thirds() {
    let p = Vec2::ONE * 1.5;
    let l = LineSegment2D::UNIT_X.scale_dst_by(2.0);
    assert_eq!(l.scalar_of(p), 0.75);
}

#[test]
fn scalar_of_works_negative_part() {
    let l = LineSegment2D::new(Vec2::NEG_ONE, Vec2::ONE);
    let p = Vec2::ONE;

    assert_eq!(l.scalar_of(p), 1.0);
}
