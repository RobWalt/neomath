use glam::Vec2;

use crate::d2::def::Ray2D;

impl Ray2D {
    pub const X: Ray2D = Ray2D {
        origin: Vec2::ZERO,
        direction: Vec2::X,
    };

    pub const Y: Ray2D = Ray2D {
        origin: Vec2::ZERO,
        direction: Vec2::Y,
    };

    pub const ONE: Ray2D = Ray2D {
        origin: Vec2::ZERO,
        direction: Vec2::ONE,
    };
}

pub const NEO_LINE_RAY_2D_EPS: f32 = 0.000_1;
