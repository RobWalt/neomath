use glam::Vec2;
use neo_geo_glam_interop::to_glam::ConvertToGlam;

use crate::d2::def::LineSegment2D;

impl LineSegment2D {
    pub const fn new(src: Vec2, dst: Vec2) -> Self {
        Self { src, dst }
    }

    pub const fn flip(&self) -> Self {
        Self::new(self.dst, self.src)
    }
}

impl From<(Vec2, Vec2)> for LineSegment2D {
    fn from((src, dst): (Vec2, Vec2)) -> Self {
        Self::new(src, dst)
    }
}

impl From<[Vec2; 2]> for LineSegment2D {
    fn from([src, dst]: [Vec2; 2]) -> Self {
        Self::new(src, dst)
    }
}

impl From<geo::Line<f32>> for LineSegment2D {
    fn from(value: geo::Line<f32>) -> Self {
        Self::from(value.to_glam())
    }
}
