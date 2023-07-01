use geo::Contains;
use geo_glam_interop::to_geo::ConvertToGeo;
use glam::Vec2;

use crate::d2::def::AABB2D;

impl AABB2D {
    pub fn contains(&self, point: Vec2) -> bool {
        self.as_rect().contains(&point.to_geo())
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.contains(other.min) || self.contains(other.max)
    }

    pub fn as_rect(&self) -> geo::Rect<f32> {
        geo::Rect::new(self.min.to_geo(), self.max.to_geo())
    }
}
