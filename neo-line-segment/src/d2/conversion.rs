use glam::Vec2;
use neo_geo_glam_interop::to_geo::ConvertToGeo;
use neo_ray::d2::def::Ray2D;

use crate::d2::def::LineSegment2D;

impl LineSegment2D {
    pub fn tuple(&self) -> (Vec2, Vec2) {
        (self.src, self.dst)
    }

    pub fn geo(&self) -> geo::Line<f32> {
        self.tuple().to_geo()
    }

    pub fn array(&self) -> [Vec2; 2] {
        [self.src, self.dst]
    }

    pub fn ray(&self) -> Ray2D {
        Ray2D::new(self.src, self.direction())
    }
}
