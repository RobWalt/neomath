use glam::Vec3;
use neo_plane::Plane;

use crate::d3::def::Ray3D;

impl Ray3D {
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn orthogonal_plane(&self) -> Plane {
        Plane::from_normal(self.direction)
    }

    pub fn any_orthogonal_ray(&self) -> Self {
        Self::from((self.origin, self.direction.any_orthogonal_vector()))
    }

    pub fn orthogonal_dir(&self, other: &Self) -> Option<Vec3> {
        (!self.is_parallel_to(other)).then(|| self.direction.cross(other.direction))
    }

    pub fn direction_normalized(&self) -> Vec3 {
        self.direction().normalize()
    }
}
