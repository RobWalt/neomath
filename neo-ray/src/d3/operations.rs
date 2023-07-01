use glam::Vec3;

use crate::d3::def::NeoLineRay3D;

impl NeoLineRay3D {
    pub fn offset_origin_by(&self, offset: Vec3) -> Self {
        Self::from((self.origin + offset, self.direction))
    }

    pub fn offset_direction_by(&self, offset: Vec3) -> Self {
        Self::from((self.origin, self.direction + offset))
    }
}
