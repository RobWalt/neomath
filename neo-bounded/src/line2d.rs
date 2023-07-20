use neo_line_segment::d2::def::LineSegment2D;

use crate::traits::NeoBounded2D;

impl NeoBounded2D for LineSegment2D {
    fn min_x(&self) -> f32 {
        self.src.x.min(self.dst.x)
    }

    fn max_x(&self) -> f32 {
        self.src.x.max(self.dst.x)
    }

    fn min_y(&self) -> f32 {
        self.src.y.min(self.dst.y)
    }

    fn max_y(&self) -> f32 {
        self.src.y.max(self.dst.y)
    }
}
