use glam::Vec2;
use neo_aabb::d2::def::AABB2D;

use crate::d2::def::NeoLineSegment2D;

impl NeoLineSegment2D {
    pub fn min_x(&self) -> f32 {
        self.src.x.min(self.dst.x)
    }

    pub fn max_x(&self) -> f32 {
        self.src.x.max(self.dst.x)
    }

    pub fn min_y(&self) -> f32 {
        self.src.y.min(self.dst.y)
    }

    pub fn max_y(&self) -> f32 {
        self.src.y.max(self.dst.y)
    }
}

impl NeoLineSegment2D {
    pub fn aabb(&self) -> AABB2D {
        AABB2D::new(
            Vec2::new(self.min_x(), self.min_y()),
            Vec2::new(self.max_x(), self.max_y()),
        )
    }
}
