use glam::Vec2;

use crate::d2::def::Ray2D;

impl Ray2D {
    pub fn offset_origin_by(&self, offset: Vec2) -> Self {
        Self::from((self.origin + offset, self.direction))
    }

    pub fn offset_direction_by(&self, offset: Vec2) -> Self {
        Self::from((self.origin, self.direction + offset))
    }
}
