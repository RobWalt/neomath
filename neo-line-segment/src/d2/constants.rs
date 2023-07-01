use glam::Vec2;

use crate::d2::def::NeoLineSegment2D;

pub const NEO_LINE_SEGMENT_2D_EPS: f32 = 0.000_1;

impl NeoLineSegment2D {
    pub const UNIT_X: NeoLineSegment2D = NeoLineSegment2D {
        src: Vec2::ZERO,
        dst: Vec2::X,
    };

    pub const UNIT_Y: NeoLineSegment2D = NeoLineSegment2D {
        src: Vec2::ZERO,
        dst: Vec2::Y,
    };

    pub const UNIT_ONE: NeoLineSegment2D = NeoLineSegment2D {
        src: Vec2::ZERO,
        dst: Vec2::ONE,
    };
}
