use glam::Vec2;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct NeoLineSegment2D {
    pub src: Vec2,
    pub dst: Vec2,
}
