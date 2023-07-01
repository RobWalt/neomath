use glam::Vec3;

use crate::d3::def::NeoLineRay3D;

impl NeoLineRay3D {
    pub fn tuple(&self) -> (Vec3, Vec3) {
        (self.origin, self.direction)
    }

    pub fn array(&self) -> [Vec3; 2] {
        [self.origin, self.direction]
    }
}
