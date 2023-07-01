use glam::Vec2;

use crate::d2::def::NeoLineRay2D;

impl NeoLineRay2D {
    pub const X: NeoLineRay2D = NeoLineRay2D {
        origin: Vec2::ZERO,
        direction: Vec2::X,
    };

    pub const Y: NeoLineRay2D = NeoLineRay2D {
        origin: Vec2::ZERO,
        direction: Vec2::Y,
    };

    pub const ONE: NeoLineRay2D = NeoLineRay2D {
        origin: Vec2::ZERO,
        direction: Vec2::ONE,
    };
}

pub const NEO_LINE_RAY_2D_EPS: f32 = 0.000_1;
