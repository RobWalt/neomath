use glam::Vec3;

use crate::d3::def::Ray3D;

impl Ray3D {
    pub const X: Ray3D = Ray3D {
        origin: Vec3::ZERO,
        direction: Vec3::X,
    };

    pub const Y: Ray3D = Ray3D {
        origin: Vec3::ZERO,
        direction: Vec3::Y,
    };

    pub const Z: Ray3D = Ray3D {
        origin: Vec3::ZERO,
        direction: Vec3::Z,
    };

    pub const ONE: Ray3D = Ray3D {
        origin: Vec3::ZERO,
        direction: Vec3::ONE,
    };
}

pub const NEO_LINE_RAY_3D_EPS: f32 = 0.000_1;
