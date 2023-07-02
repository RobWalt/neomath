use glam::Vec3;
use neo_aabb::d3::def::AABB3D;

use crate::d3::def::LineSegment3D;

impl LineSegment3D {
    pub fn min_x(&self) -> f32 {
        self.src.x.min(self.dst.x)
    }

    pub fn max_x(&self) -> f32 {
        self.src.x.max(self.dst.x)
    }

    pub fn min_y(&self) -> f32 {
        self.src.y.min(self.dst.y)
    }

    pub fn max_y(&self) -> f32 {
        self.src.y.max(self.dst.y)
    }

    pub fn min_z(&self) -> f32 {
        self.src.z.min(self.dst.z)
    }

    pub fn max_z(&self) -> f32 {
        self.src.z.max(self.dst.z)
    }
}

impl LineSegment3D {
    pub fn aabb(&self) -> AABB3D {
        AABB3D::new(
            Vec3::new(self.min_x(), self.min_y(), self.min_z()),
            Vec3::new(self.max_x(), self.max_y(), self.max_z()),
        )
    }
}
