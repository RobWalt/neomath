use glam::Vec2;

use crate::d2::def::LineSegment2D;

impl LineSegment2D {
    pub fn offset_line_by(&self, offset: Vec2) -> Self {
        Self::from(self.array().map(|v| v + offset))
    }

    pub fn offset_src_by(&self, offset: Vec2) -> Self {
        Self::from((self.src + offset, self.dst))
    }

    pub fn offset_dst_by(&self, offset: Vec2) -> Self {
        Self::from((self.src, self.dst + offset))
    }

    pub fn scale_line_by(&self, factor: f32) -> Self {
        let center = self.center();
        Self::from(self.array().map(move |v| center + factor * (v - center)))
    }

    pub fn scale_dst_by(&self, factor: f32) -> Self {
        Self::from((self.src, self.src + factor * self.direction()))
    }

    pub fn scale_src_by(&self, factor: f32) -> Self {
        Self::from((self.dst - factor * self.direction(), self.dst))
    }
}
