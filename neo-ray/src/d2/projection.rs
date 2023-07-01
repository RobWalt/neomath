use glam::Vec2;

use crate::d2::def::NeoLineRay2D;

impl NeoLineRay2D {
    pub fn project_point(&self, point: Vec2) -> Vec2 {
        point.project_onto(self.direction())
    }

    pub fn distance_to_point(&self, point: Vec2) -> f32 {
        self.project_point(point).distance(point)
    }
}

#[test]
fn projection_works() {
    let p = Vec2::Y;
    let l = NeoLineRay2D::ONE;
    assert_eq!(l.project_point(p), Vec2::ONE * 0.5);
}
