use neo_aabb::d2::def::AABB2D;

use crate::traits::NeoBounded2D;

impl NeoBounded2D for AABB2D {
    fn min_x(&self) -> f32 {
        self.min.x
    }
    fn min_y(&self) -> f32 {
        self.min.y
    }
    fn max_x(&self) -> f32 {
        self.max.x
    }
    fn max_y(&self) -> f32 {
        self.max.y
    }
}
