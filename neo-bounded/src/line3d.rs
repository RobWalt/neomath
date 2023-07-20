use neo_line_segment::d3::def::LineSegment3D;

use crate::traits::NeoBounded3D;

impl NeoBounded3D for LineSegment3D {
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

    fn min_z(&self) -> f32 {
        self.src.z.min(self.dst.z)
    }

    fn max_z(&self) -> f32 {
        self.src.z.max(self.dst.z)
    }
}
