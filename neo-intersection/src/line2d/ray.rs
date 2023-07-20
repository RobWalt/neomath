use neo_line_segment::d2::def::LineSegment2D;
use neo_ray::d2::def::Ray2D;

use crate::ray2d::line::RayLine2DIntersection;
use crate::trait_def::NeoIntersectable;

impl NeoIntersectable<Ray2D> for LineSegment2D {
    type Output = RayLine2DIntersection;

    fn intersection(&self, rhs: &Ray2D) -> Self::Output {
        rhs.intersection(self)
    }
}
