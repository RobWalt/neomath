use glam::Vec3;

use crate::d3::def::NeoLineRay3D;

impl NeoLineRay3D {
    pub const fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
}

impl From<(Vec3, Vec3)> for NeoLineRay3D {
    fn from((origin, direction): (Vec3, Vec3)) -> Self {
        Self::new(origin, direction)
    }
}

impl From<[Vec3; 2]> for NeoLineRay3D {
    fn from([origin, direction]: [Vec3; 2]) -> Self {
        Self::new(origin, direction)
    }
}
