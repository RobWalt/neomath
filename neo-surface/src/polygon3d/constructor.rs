use glam::Vec3;

use crate::polygon3d::def::NeoPolygon3D;

impl NeoPolygon3D {
    pub fn from_outline_and_normal(outline: Vec<Vec3>, normal: Vec3) -> Self {
        Self {
            normal,
            exterior: outline,
            interiors: vec![],
        }
    }
}
