use glam::Vec2;

use crate::d2::def::LineSegment2D;

impl LineSegment2D {
    pub fn angle_xaxis(&self) -> f32 {
        self.direction().angle_between(Vec2::X)
    }

    pub fn angle_yaxis(&self) -> f32 {
        self.direction().angle_between(Vec2::Y)
    }

    pub fn ccw_rotate_around(&self, p: Vec2, angle: f32) -> Self {
        Self::from(
            self.array()
                .map(|v| (v - p).rotate(Vec2::from_angle(angle)) + p),
        )
    }

    pub fn cw_rotate_around(&self, p: Vec2, angle: f32) -> Self {
        self.ccw_rotate_around(p, -angle)
    }

    pub fn angle_between(&self, other: &Self) -> f32 {
        self.direction().angle_between(other.direction())
    }
}

#[test]
fn rotation_around_works() {
    let l = LineSegment2D::UNIT_ONE.offset_line_by(Vec2::ONE);
    let p = Vec2::ONE;
    let angle = 90.0_f32.to_radians();
    assert_eq!(
        l.ccw_rotate_around(p, angle),
        LineSegment2D::new(Vec2::ONE, Vec2::Y * 2.0)
    );
}
