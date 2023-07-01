use glam::Vec2;

use crate::d2::def::LineSegment2D;

impl LineSegment2D {
    pub fn split_at_percent(&self, percentage: f32) -> Option<(Self, Self)> {
        (0.0..=1.0).contains(&percentage).then(|| {
            let split_point = self.inject_scalar(percentage);
            self.insert_midpoint(split_point)
        })
    }

    pub fn insert_midpoint(&self, midpoint: Vec2) -> (Self, Self) {
        (Self::new(self.src, midpoint), Self::new(midpoint, self.dst))
    }

    pub fn cut_in_n_segments(&self, n: usize) -> Vec<Self> {
        let d = self.length() / n as f32;
        let dir = self.direction();
        (0..n)
            .map(|n| n as f32)
            .map(|offset| Self::from(self.array().map(|v| v + offset * d * dir)))
            .collect::<Vec<_>>()
    }
}
