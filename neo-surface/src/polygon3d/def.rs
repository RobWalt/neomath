use glam::Vec3;

#[derive(Debug, Clone)]
pub struct NeoPolygon3D {
    pub normal: Vec3,
    pub exterior: Vec<Vec3>,
    pub interiors: Vec<Vec<Vec3>>,
}
