use glam::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Ray2D {
    pub origin: Vec2,
    pub direction: Vec2,
}
