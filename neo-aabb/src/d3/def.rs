use glam::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB3D {
    pub(crate) min: Vec3,
    pub(crate) max: Vec3,
}

impl AABB3D {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            min: Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z)),
            max: Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z)),
        }
    }
}
