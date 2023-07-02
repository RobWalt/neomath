use glam::Vec3;
use neo_ray::d3::def::Ray3D;

use crate::d3::def::LineSegment3D;

impl LineSegment3D {
    pub fn tuple(&self) -> (Vec3, Vec3) {
        (self.src, self.dst)
    }

    pub fn array(&self) -> [Vec3; 2] {
        [self.src, self.dst]
    }

    pub fn ray(&self) -> Ray3D {
        Ray3D::new(self.src, self.direction())
    }
}
