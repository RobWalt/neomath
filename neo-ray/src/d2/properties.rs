use glam::Vec2;

use crate::d2::def::Ray2D;

impl Ray2D {
    pub fn direction(&self) -> Vec2 {
        self.direction
    }

    pub fn normal(&self) -> Vec2 {
        self.direction().perp().normalize_or_zero()
    }

    pub fn direction_normalized(&self) -> Vec2 {
        self.direction().normalize()
    }
}
