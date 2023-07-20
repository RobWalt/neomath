use glam::Vec2;

use crate::d2::def::Ray2D;

impl Ray2D {
    pub fn project_point(&self, point: Vec2) -> Vec2 {
        (point - self.origin).project_onto(self.direction()) + self.origin
    }

    pub fn distance_to_point(&self, point: Vec2) -> f32 {
        let proj = self.project_point(point);
        proj.distance(point)
    }
}

#[test]
fn projection_works() {
    let p = Vec2::Y;
    let l = Ray2D::ONE;
    assert_eq!(l.project_point(p), Vec2::ONE * 0.5);
}

#[test]
fn projection_of_line_not_through_zero() {
    let p = Vec2::Y * 2.0;
    let l = Ray2D::ONE.offset_origin_by(Vec2::Y);
    assert_eq!(l.project_point(p), Vec2::ONE * 0.5 + Vec2::Y);
}
