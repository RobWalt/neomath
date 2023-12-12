use glam::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Ray3D {
    pub origin: Vec3,
    pub direction: Vec3,
}
