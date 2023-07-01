use glam::Vec3;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Ray3D {
    pub origin: Vec3,
    pub direction: Vec3,
}
