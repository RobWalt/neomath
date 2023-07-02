use glam::Vec3;

use crate::d3::def::LineSegment3D;

pub const NEO_LINE_SEGMENT_3D_EPS: f32 = 0.000_1;

impl LineSegment3D {
    pub const UNIT_X: LineSegment3D = LineSegment3D {
        src: Vec3::ZERO,
        dst: Vec3::X,
    };

    pub const UNIT_Y: LineSegment3D = LineSegment3D {
        src: Vec3::ZERO,
        dst: Vec3::Y,
    };

    pub const UNIT_Z: LineSegment3D = LineSegment3D {
        src: Vec3::ZERO,
        dst: Vec3::Z,
    };

    pub const UNIT_ONE: LineSegment3D = LineSegment3D {
        src: Vec3::ZERO,
        dst: Vec3::ONE,
    };
}
