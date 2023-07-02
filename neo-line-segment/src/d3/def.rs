use glam::Vec3;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct LineSegment3D {
    pub src: Vec3,
    pub dst: Vec3,
}
