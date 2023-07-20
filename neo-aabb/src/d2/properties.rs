use glam::Vec2;

use crate::d2::def::AABB2D;

impl AABB2D {
    pub fn points(&self) -> [Vec2; 4] {
        [
            self.min,
            Vec2::new(self.min.x, self.max.y),
            self.max,
            Vec2::new(self.max.x, self.min.y),
        ]
    }

    pub fn lines(&self) -> [[Vec2; 2]; 4] {
        let [a, b, c, d] = self.points();
        [[a, b], [b, c], [c, d], [d, a]]
    }

    pub fn hausdorff_size(&self) -> f32 {
        self.min.distance(self.max)
    }
}
