use glam::Vec3;

use crate::def::NeoSurface;

impl NeoSurface {
    pub fn facing_direction(&self) -> Vec3 {
        self.coordinate_system.plane.normal
    }
}
