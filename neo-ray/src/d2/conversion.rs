use glam::Vec2;
use neo_geo_glam_interop::to_geo::ConvertToGeo;

use crate::d2::def::Ray2D;

impl Ray2D {
    pub fn tuple(&self) -> (Vec2, Vec2) {
        (self.origin, self.direction)
    }

    pub fn geo(&self) -> geo::Line<f32> {
        self.tuple().to_geo()
    }

    pub fn array(&self) -> [Vec2; 2] {
        [self.origin, self.direction]
    }
}
