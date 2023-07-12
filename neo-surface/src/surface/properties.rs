use geo::Area;
use glam::Vec3;

use crate::surface::def::NeoSurface;

impl NeoSurface {
    pub fn facing_direction(&self) -> Vec3 {
        self.coordinate_system.plane.normal
    }

    pub fn area(&self) -> f32 {
        self.shape.unsigned_area()
    }
}
