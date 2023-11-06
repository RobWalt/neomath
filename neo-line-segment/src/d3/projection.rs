use glam::Vec3;

use crate::d3::def::LineSegment3D;

impl LineSegment3D {
    // NOTE: Does it make sense to clip the projection to the line?
    pub fn project_point(&self, point: Vec3) -> Vec3 {
        point.project_onto(self.direction())
    }

    pub fn scalar_of(&self, point: Vec3) -> f32 {
        self.project_point(point - self.src).length() / self.direction().length()
    }

    /// Inspired by the SDF formula of a line
    /// https://www.youtube.com/watch?v=PMltMdi1Wzg
    pub fn distance_to_point(&self, point: Vec3) -> f32 {
        let pa = point - self.src;
        let ba = self.direction();
        let prod = pa.dot(ba) / ba.length_squared();
        let h = prod.clamp(0.0, 1.0);
        pa.distance(h * ba)
    }

    pub fn inject_scalar(&self, scalar: f32) -> Vec3 {
        self.src + scalar * self.direction()
    }
}

#[test]
fn projection_works() {
    let p = Vec3::Y;
    let l = LineSegment3D::UNIT_ONE;
    assert_eq!(l.project_point(p), Vec3::ONE / 3.0);
}

#[test]
fn scalar_of_works_half() {
    let p = Vec3::ONE - Vec3::Z;
    let l = LineSegment3D::UNIT_X.scale_dst_by(2.0);
    assert_eq!(l.scalar_of(p), 0.5);
}

#[test]
fn scalar_of_works_two_thirds() {
    let p = (Vec3::ONE - Vec3::Z) * 1.5;
    let l = LineSegment3D::UNIT_X.scale_dst_by(2.0);
    assert_eq!(l.scalar_of(p), 0.75);
}

#[test]
fn scalar_of_works_negative_part() {
    let l = LineSegment3D::new(Vec3::NEG_ONE, Vec3::ONE);
    let p = Vec3::ONE;
    assert_eq!(l.scalar_of(p), 1.0);
}
