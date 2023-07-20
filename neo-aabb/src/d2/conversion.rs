use geo::BoundingRect;
use glam::Vec2;
use neo_float::NeoFloat;
use neo_geo_glam_interop::neo_float::NeoFloatConversions;
use neo_geo_glam_interop::to_glam::ConvertToGlam;

use crate::d2::def::AABB2D;

impl<F: NeoFloat> From<(geo::Coord<F>, geo::Coord<F>)> for AABB2D {
    fn from((a, b): (geo::Coord<F>, geo::Coord<F>)) -> Self {
        Self::new(
            a.to_f64_version().to_glam().as_vec2(),
            b.to_f64_version().to_glam().as_vec2(),
        )
    }
}

impl From<([f32; 2], [f32; 2])> for AABB2D {
    fn from((a, b): ([f32; 2], [f32; 2])) -> Self {
        Self::new(Vec2::from_array(a), Vec2::from_array(b))
    }
}

impl<F: NeoFloat> From<geo::Rect<F>> for AABB2D {
    fn from(value: geo::Rect<F>) -> Self {
        Self::from((value.min(), value.max()))
    }
}

impl<F: NeoFloat> From<geo::LineString<F>> for AABB2D {
    fn from(value: geo::LineString<F>) -> Self {
        Self::from(
            value
                .bounding_rect()
                .unwrap_or(geo::Rect::new(geo::Coord::zero(), geo::Coord::zero())),
        )
    }
}

impl<F: NeoFloat> From<geo::Triangle<F>> for AABB2D {
    fn from(value: geo::Triangle<F>) -> Self {
        Self::from(value.bounding_rect())
    }
}

impl<F: NeoFloat> From<geo::Polygon<F>> for AABB2D {
    fn from(value: geo::Polygon<F>) -> Self {
        Self::from(
            value
                .bounding_rect()
                .unwrap_or(geo::Rect::new(geo::Coord::zero(), geo::Coord::zero())),
        )
    }
}

impl<F: NeoFloat> From<geo::MultiPolygon<F>> for AABB2D {
    fn from(value: geo::MultiPolygon<F>) -> Self {
        Self::from(
            value
                .bounding_rect()
                .unwrap_or(geo::Rect::new(geo::Coord::zero(), geo::Coord::zero())),
        )
    }
}
