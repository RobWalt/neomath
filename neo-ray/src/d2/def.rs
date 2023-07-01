use glam::Vec2;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Ray2D {
    pub origin: Vec2,
    pub direction: Vec2,
}
