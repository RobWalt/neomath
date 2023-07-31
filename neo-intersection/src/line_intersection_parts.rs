use glam::{Vec2, Vec3};
use neo_line_segment::d2::def::LineSegment2D;
use neo_line_segment::d3::def::LineSegment3D;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Line3DIntersectionParts {
    Point(Vec3),
    Line(LineSegment3D),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Line2DIntersectionParts {
    Point(Vec2),
    Line(LineSegment2D),
}

impl Line2DIntersectionParts {
    pub fn inject_with(&self, f: &impl Fn(Vec2) -> Vec3) -> Line3DIntersectionParts {
        match self {
            Line2DIntersectionParts::Point(p) => Line3DIntersectionParts::Point(f(*p)),
            Line2DIntersectionParts::Line(l) => {
                let src = f(l.src);
                let dst = f(l.dst);
                let line = LineSegment3D::new(src, dst);
                Line3DIntersectionParts::Line(line)
            }
        }
    }
}
