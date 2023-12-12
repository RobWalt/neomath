use geo::{BoundingRect, CoordsIter};
use neo_aabb::d2::def::AABB2D;
use neo_utils::float_ord::FloatOrd;

use crate::traits::NeoBounded2D;

impl NeoBounded2D for geo::Polygon<f32> {
    fn aabb(&self) -> AABB2D {
        AABB2D::from(
            self.bounding_rect()
                .unwrap_or_else(|| geo::Rect::new(geo::Coord::zero(), geo::Coord::zero())),
        )
    }

    fn min_x(&self) -> f32 {
        self.coords_iter()
            .map(|c| c.x)
            .min_by_key(|&x| FloatOrd(x))
            .unwrap_or_default()
    }

    fn min_y(&self) -> f32 {
        self.coords_iter()
            .map(|c| c.y)
            .min_by_key(|&x| FloatOrd(x))
            .unwrap_or_default()
    }

    fn max_x(&self) -> f32 {
        self.coords_iter()
            .map(|c| c.x)
            .max_by_key(|&x| FloatOrd(x))
            .unwrap_or_default()
    }

    fn max_y(&self) -> f32 {
        self.coords_iter()
            .map(|c| c.y)
            .max_by_key(|&x| FloatOrd(x))
            .unwrap_or_default()
    }
}
