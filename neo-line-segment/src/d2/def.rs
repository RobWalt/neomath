use glam::Vec2;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct LineSegment2D {
    pub src: Vec2,
    pub dst: Vec2,
}
