use glam::Vec3;

use crate::d3::def::Ray3D;

impl Ray3D {
    pub fn project_point(&self, point: Vec3) -> Vec3 {
        (point - self.origin).project_onto(self.direction()) + self.origin
    }

    pub fn distance_to_point(&self, point: Vec3) -> f32 {
        self.project_point(point).distance(point)
    }
}

#[test]
fn projection_works() {
    let p = Vec3::Y;
    let l = Ray3D::ONE;
    assert_eq!(l.project_point(p), Vec3::ONE / 3.0);
}

#[test]
fn projection_of_line_not_through_zero() {
    let p = Vec3::Y * 2.0;
    let l = Ray3D::ONE.offset_origin_by(Vec3::Y);
    assert_eq!(l.project_point(p), Vec3::ONE / 3.0 + Vec3::Y);
}
