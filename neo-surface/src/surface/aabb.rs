use neo_aabb::d3::def::AABB3D;

use crate::surface::def::NeoSurface;

impl NeoSurface {
    pub fn aabb(&self) -> AABB3D {
        self.as_polygon_3d().aabb()
    }
}
