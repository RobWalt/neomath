use glam::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoPolygon3D {
    pub normal: Vec3,
    pub exterior: Vec<Vec3>,
    pub interiors: Vec<Vec<Vec3>>,
}
