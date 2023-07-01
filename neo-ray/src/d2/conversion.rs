use geo_glam_interop::to_geo::ConvertToGeo;
use glam::Vec2;

use crate::d2::def::NeoLineRay2D;

impl NeoLineRay2D {
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
