use glam::Vec2;

use crate::d2::def::LineSegment2D;

pub const NEO_LINE_SEGMENT_2D_EPS: f32 = 0.000_1;

impl LineSegment2D {
    pub const UNIT_X: LineSegment2D = LineSegment2D {
        src: Vec2::ZERO,
        dst: Vec2::X,
    };

    pub const UNIT_Y: LineSegment2D = LineSegment2D {
        src: Vec2::ZERO,
        dst: Vec2::Y,
    };

    pub const UNIT_ONE: LineSegment2D = LineSegment2D {
        src: Vec2::ZERO,
        dst: Vec2::ONE,
    };
}
