use glam::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB2D {
    pub min: Vec2,
    pub max: Vec2,
}

fn min_max(x: f32, y: f32) -> [f32; 2] {
    [f32::min, f32::max].map(|f| f(x, y))
}

impl AABB2D {
    pub fn new(a: Vec2, b: Vec2) -> Self {
        let [min_x, max_x] = min_max(a.x, b.x);
        let [min_y, max_y] = min_max(a.y, b.y);
        Self {
            min: Vec2::new(min_x, min_y),
            max: Vec2::new(max_x, max_y),
        }
    }
}
