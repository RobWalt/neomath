use geo_glam_interop::to_glam::ConvertToGlam;
use glam::Vec2;

use crate::d2::def::Ray2D;

impl Ray2D {
    pub const fn new(origin: Vec2, direction: Vec2) -> Self {
        Self { origin, direction }
    }
}

impl From<(Vec2, Vec2)> for Ray2D {
    fn from((origin, direction): (Vec2, Vec2)) -> Self {
        Self::new(origin, direction)
    }
}

impl From<[Vec2; 2]> for Ray2D {
    fn from([origin, direction]: [Vec2; 2]) -> Self {
        Self::new(origin, direction)
    }
}

impl From<geo::Line<f32>> for Ray2D {
    fn from(value: geo::Line<f32>) -> Self {
        Self::from(value.to_glam())
    }
}
