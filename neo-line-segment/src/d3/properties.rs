use glam::Vec3;

use crate::d3::def::LineSegment3D;

impl LineSegment3D {
    pub fn direction(&self) -> Vec3 {
        self.dst - self.src
    }

    pub fn orthogonal_dir(&self, other: &Self) -> Option<Vec3> {
        self.ray().orthogonal_dir(&other.ray())
    }

    pub fn direction_normalized(&self) -> Vec3 {
        self.direction().normalize()
    }

    pub fn length(&self) -> f32 {
        self.direction().length()
    }

    pub fn length_squared(&self) -> f32 {
        self.direction().length_squared()
    }

    pub fn center(&self) -> Vec3 {
        self.src + self.direction() * 0.5
    }
}
