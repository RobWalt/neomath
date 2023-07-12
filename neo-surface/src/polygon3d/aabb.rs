use std::cmp::Ordering;

use glam::Vec3;
use neo_aabb::d3::def::AABB3D;

use crate::polygon3d::def::NeoPolygon3D;

fn float_ord(f1: &f32, f2: &f32) -> Ordering {
    f1.partial_cmp(f2).unwrap_or(Ordering::Less)
}

impl NeoPolygon3D {
    pub fn min_x(&self) -> f32 {
        self.iter_all_points()
            .map(|p| p.x)
            .min_by(float_ord)
            .unwrap_or_default()
    }

    pub fn min_y(&self) -> f32 {
        self.iter_all_points()
            .map(|p| p.y)
            .min_by(float_ord)
            .unwrap_or_default()
    }

    pub fn min_z(&self) -> f32 {
        self.iter_all_points()
            .map(|p| p.y)
            .min_by(float_ord)
            .unwrap_or_default()
    }

    pub fn max_x(&self) -> f32 {
        self.iter_all_points()
            .map(|p| p.x)
            .max_by(float_ord)
            .unwrap_or_default()
    }

    pub fn max_y(&self) -> f32 {
        self.iter_all_points()
            .map(|p| p.y)
            .max_by(float_ord)
            .unwrap_or_default()
    }

    pub fn max_z(&self) -> f32 {
        self.iter_all_points()
            .map(|p| p.y)
            .max_by(float_ord)
            .unwrap_or_default()
    }

    pub fn aabb(&self) -> AABB3D {
        AABB3D::new(
            Vec3::new(self.min_x(), self.min_y(), self.min_z()),
            Vec3::new(self.max_x(), self.max_y(), self.max_z()),
        )
    }
}
