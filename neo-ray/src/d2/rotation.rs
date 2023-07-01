use glam::Vec2;

use crate::d2::def::Ray2D;

impl Ray2D {
    pub fn angle_xaxis(&self) -> f32 {
        self.direction().angle_between(Vec2::X)
    }

    pub fn angle_yaxis(&self) -> f32 {
        self.direction().angle_between(Vec2::Y)
    }

    pub fn ccw_rotate(&self, angle: f32) -> Self {
        Self::from((
            self.origin,
            (self.direction - self.origin).rotate(Vec2::from_angle(angle)) + self.origin,
        ))
    }

    pub fn cw_rotate(&self, angle: f32) -> Self {
        self.ccw_rotate(-angle)
    }

    pub fn angle_between(&self, other: &Self) -> f32 {
        self.direction().angle_between(other.direction())
    }
}

#[test]
fn rotation_works() {
    use crate::d2::constants::NEO_LINE_RAY_2D_EPS;
    let l = Ray2D::X;
    let angle = 90.0_f32.to_radians();
    let rotated = l.ccw_rotate(angle);
    assert!(rotated.origin.abs_diff_eq(Vec2::ZERO, NEO_LINE_RAY_2D_EPS));
    assert!(rotated.direction.abs_diff_eq(Vec2::Y, NEO_LINE_RAY_2D_EPS));
}
