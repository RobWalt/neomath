use glam::Vec3;

use crate::d3::def::LineSegment3D;

impl LineSegment3D {
    pub const fn new(src: Vec3, dst: Vec3) -> Self {
        Self { src, dst }
    }

    pub const fn flip(&self) -> Self {
        Self::new(self.dst, self.src)
    }
}

impl From<(Vec3, Vec3)> for LineSegment3D {
    fn from((src, dst): (Vec3, Vec3)) -> Self {
        Self::new(src, dst)
    }
}

impl From<[Vec3; 2]> for LineSegment3D {
    fn from([src, dst]: [Vec3; 2]) -> Self {
        Self::new(src, dst)
    }
}
