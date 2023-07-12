use glam::Quat;

use crate::surface::def::NeoSurface;

impl NeoSurface {
    pub fn rotation_between(&self, other: &Self) -> Quat {
        Quat::from_rotation_arc(
            self.coordinate_system.plane.normal,
            other.coordinate_system.plane.normal,
        )
    }
}
