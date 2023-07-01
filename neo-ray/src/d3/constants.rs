use glam::Vec3;

use crate::d3::def::NeoLineRay3D;

impl NeoLineRay3D {
    pub const X: NeoLineRay3D = NeoLineRay3D {
        origin: Vec3::ZERO,
        direction: Vec3::X,
    };

    pub const Y: NeoLineRay3D = NeoLineRay3D {
        origin: Vec3::ZERO,
        direction: Vec3::Y,
    };

    pub const Z: NeoLineRay3D = NeoLineRay3D {
        origin: Vec3::ZERO,
        direction: Vec3::Z,
    };

    pub const ONE: NeoLineRay3D = NeoLineRay3D {
        origin: Vec3::ZERO,
        direction: Vec3::ONE,
    };
}

pub const NEO_LINE_RAY_3D_EPS: f32 = 0.000_1;
