use neo_aabb::d3::def::AABB3D;

use crate::traits::NeoBounded3D;

impl NeoBounded3D for AABB3D {
    fn min_x(&self) -> f32 {
        self.min.x
    }

    fn min_y(&self) -> f32 {
        self.min.y
    }

    fn min_z(&self) -> f32 {
        self.min.z
    }

    fn max_x(&self) -> f32 {
        self.max.x
    }

    fn max_y(&self) -> f32 {
        self.max.y
    }

    fn max_z(&self) -> f32 {
        self.max.z
    }
}
