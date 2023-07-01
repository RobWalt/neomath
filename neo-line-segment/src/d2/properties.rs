use glam::Vec2;

use crate::d2::def::NeoLineSegment2D;

impl NeoLineSegment2D {
    pub fn direction(&self) -> Vec2 {
        self.dst - self.src
    }

    pub fn normal(&self) -> Vec2 {
        self.direction().perp().normalize_or_zero()
    }

    pub fn direction_normalized(&self) -> Vec2 {
        self.direction().normalize()
    }

    pub fn length(&self) -> f32 {
        self.direction().length()
    }

    pub fn length_squared(&self) -> f32 {
        self.direction().length_squared()
    }

    pub fn center(&self) -> Vec2 {
        self.src + self.direction() * 0.5
    }
}
