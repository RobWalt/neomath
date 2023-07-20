use neo_aabb::d3::def::AABB3D;
use neo_surface::surface::def::NeoSurface;

use crate::traits::NeoBounded3D;

impl NeoBounded3D for NeoSurface {
    fn aabb(&self) -> AABB3D {
        self.as_polygon_3d().aabb()
    }

    fn min_x(&self) -> f32 {
        self.aabb().min_x()
    }

    fn min_y(&self) -> f32 {
        self.aabb().min_y()
    }

    fn min_z(&self) -> f32 {
        self.aabb().min_z()
    }

    fn max_x(&self) -> f32 {
        self.aabb().max_x()
    }

    fn max_y(&self) -> f32 {
        self.aabb().max_y()
    }

    fn max_z(&self) -> f32 {
        self.aabb().max_z()
    }
}
