use glam::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB2D {
    pub(crate) min: Vec2,
    pub(crate) max: Vec2,
}

impl AABB2D {
    pub fn new(a: Vec2, b: Vec2) -> Self {
        Self {
            min: Vec2::new(a.x.min(b.x), a.y.min(b.y)),
            max: Vec2::new(a.x.max(b.x), a.y.max(b.y)),
        }
    }
}
