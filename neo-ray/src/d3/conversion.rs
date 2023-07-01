use glam::Vec3;

use crate::d3::def::Ray3D;

impl Ray3D {
    pub fn tuple(&self) -> (Vec3, Vec3) {
        (self.origin, self.direction)
    }

    pub fn array(&self) -> [Vec3; 2] {
        [self.origin, self.direction]
    }
}
