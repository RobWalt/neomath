use geo::BoundingRect;
use glam::Vec2;
use neo_geo_glam_interop::to_glam::ConvertToGlam;

use crate::d2::def::AABB2D;

impl From<(geo::Coord<f32>, geo::Coord<f32>)> for AABB2D {
    fn from((a, b): (geo::Coord<f32>, geo::Coord<f32>)) -> Self {
        Self::new(a.to_glam(), b.to_glam())
    }
}

impl From<([f32; 2], [f32; 2])> for AABB2D {
    fn from((a, b): ([f32; 2], [f32; 2])) -> Self {
        Self::new(Vec2::from_array(a), Vec2::from_array(b))
    }
}

impl From<geo::Rect<f32>> for AABB2D {
    fn from(value: geo::Rect<f32>) -> Self {
        Self::from((value.min(), value.max()))
    }
}

impl From<geo::LineString<f32>> for AABB2D {
    fn from(value: geo::LineString<f32>) -> Self {
        Self::from(
            value
                .bounding_rect()
                .unwrap_or(geo::Rect::new(geo::Coord::zero(), geo::Coord::zero())),
        )
    }
}

impl From<geo::Triangle<f32>> for AABB2D {
    fn from(value: geo::Triangle<f32>) -> Self {
        Self::from(value.bounding_rect())
    }
}

impl From<geo::Polygon<f32>> for AABB2D {
    fn from(value: geo::Polygon<f32>) -> Self {
        Self::from(
            value
                .bounding_rect()
                .unwrap_or(geo::Rect::new(geo::Coord::zero(), geo::Coord::zero())),
        )
    }
}

impl From<geo::MultiPolygon<f32>> for AABB2D {
    fn from(value: geo::MultiPolygon<f32>) -> Self {
        Self::from(
            value
                .bounding_rect()
                .unwrap_or(geo::Rect::new(geo::Coord::zero(), geo::Coord::zero())),
        )
    }
}
