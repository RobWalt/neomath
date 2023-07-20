use glam::Vec2;
use neo_bounded::traits::NeoBounded2D;
use neo_line_segment::d2::def::LineSegment2D;
use neo_ray::d2::def::Ray2D;

use crate::ray2d::aabb::RayAABB2DIntersection;
use crate::results::LineLine2DIntersection;
use crate::trait_def::NeoIntersectable;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RayLine2DIntersection {
    None,
    Parallel,
    CollinearOverlap(LineSegment2D),
    /// This is a real intersection where the intersection point is located in both lines that were
    /// intersected. If you're interested in the general intersection point which may be located
    /// outside the line, consider using [`LineSegment2D::ray_intersection`]
    Intersection(Vec2),
}

impl NeoIntersectable<LineSegment2D> for Ray2D {
    type Output = RayLine2DIntersection;

    fn intersection(&self, rhs: &LineSegment2D) -> Self::Output {
        let rhs_aabb = rhs.aabb();
        let inter = self.intersection(&rhs_aabb);
        match inter {
            RayAABB2DIntersection::None => parallel_case_analysis(self, rhs),
            RayAABB2DIntersection::Point(p) => aabb_point_case_analysis(self, p, rhs),
            RayAABB2DIntersection::Line(l) => aabb_line_case_analysis(l, rhs),
        }
    }
}

fn parallel_case_analysis(ray: &Ray2D, rhs: &LineSegment2D) -> RayLine2DIntersection {
    if ray.is_parallel_to(&rhs.ray()) {
        RayLine2DIntersection::Parallel
    } else {
        RayLine2DIntersection::None
    }
}

fn aabb_point_case_analysis(
    ray: &Ray2D,
    aabb_intersection_point: Vec2,
    rhs: &LineSegment2D,
) -> RayLine2DIntersection {
    if rhs.is_point_on_line(aabb_intersection_point) {
        RayLine2DIntersection::Intersection(aabb_intersection_point)
    } else {
        parallel_case_analysis(ray, rhs)
    }
}

fn aabb_line_case_analysis(
    aabb_intersection_line: LineSegment2D,
    rhs: &LineSegment2D,
) -> RayLine2DIntersection {
    let inter = aabb_intersection_line.intersection(rhs);
    match inter {
        LineLine2DIntersection::None => RayLine2DIntersection::None,
        LineLine2DIntersection::Parallel => RayLine2DIntersection::Parallel,
        LineLine2DIntersection::CollinearNoOverlap => {
            let text = [
                "shouldn't be possible",
                "since the ray is infinite",
                "and since if there is overlap, it is always the whole line",
            ]
            .join(" ");
            unreachable!("{text}")
        }
        LineLine2DIntersection::CollinearOverlap(_) => {
            RayLine2DIntersection::CollinearOverlap(rhs.clone())
        }
        LineLine2DIntersection::Intersection(i) => RayLine2DIntersection::Intersection(i),
    }
}

#[cfg(test)]
mod ray_line {
    use glam::Vec2;
    use neo_line_segment::d2::def::LineSegment2D;
    use neo_ray::d2::def::Ray2D;

    use crate::ray2d::line::RayLine2DIntersection;
    use crate::trait_def::NeoIntersectable;

    #[test]
    fn parallel_ray_line_with_aabb_works() {
        let line = LineSegment2D::new(Vec2::ZERO, Vec2::ONE);
        let offsetted_line = line.offset_line_by((Vec2::Y - Vec2::X) * 0.2);

        assert_eq!(
            offsetted_line.ray().intersection(&line),
            RayLine2DIntersection::Parallel
        );
    }

    #[test]
    fn parallel_ray_line_without_aabb_works() {
        let line = LineSegment2D::new(Vec2::ZERO, Vec2::ONE);
        let offsetted_line = line.offset_line_by((Vec2::Y - Vec2::X) * 2.0);

        assert_eq!(
            offsetted_line.ray().intersection(&line),
            RayLine2DIntersection::Parallel
        );
    }

    #[test]
    fn skewed_ray_line_with_aabb_works() {
        let line = LineSegment2D::new(Vec2::ZERO, Vec2::ONE);
        let offsetted_line = line
            .offset_line_by((Vec2::Y - Vec2::X) * 0.2)
            .offset_src_by(Vec2::Y * 0.2);

        assert_eq!(
            offsetted_line.ray().intersection(&line),
            RayLine2DIntersection::None
        );
    }

    #[test]
    fn skewed_ray_line_without_aabb_works() {
        let line = LineSegment2D::new(Vec2::ZERO, Vec2::ONE);
        let offsetted_line = line
            .offset_line_by((Vec2::Y - Vec2::X) * 2.0)
            .offset_src_by(Vec2::Y * 0.2);

        assert_eq!(
            offsetted_line.ray().intersection(&line),
            RayLine2DIntersection::None
        );
    }

    #[test]
    fn intersection_point_works() {
        let line = LineSegment2D::new(Vec2::ZERO, Vec2::ONE);
        let ray = Ray2D::new(Vec2::Y - Vec2::X, (Vec2::X - Vec2::Y) * 2.0);

        assert_eq!(
            ray.intersection(&line),
            RayLine2DIntersection::Intersection(Vec2::ZERO)
        );
    }

    #[test]
    fn intersection_point_other_side_works() {
        let line = LineSegment2D::new(Vec2::ZERO, Vec2::ONE);
        let ray =
            Ray2D::new(Vec2::Y - Vec2::X, (Vec2::X - Vec2::Y) * 2.0).offset_origin_by(Vec2::ONE);

        assert_eq!(
            ray.intersection(&line),
            RayLine2DIntersection::Intersection(Vec2::ONE)
        );
    }

    #[test]
    fn overlap_works() {
        let line = LineSegment2D::new(Vec2::ZERO, Vec2::ONE);
        let ray = line.ray();
        assert_eq!(
            ray.intersection(&line),
            RayLine2DIntersection::CollinearOverlap(line)
        );
    }

    #[test]
    fn overlap_offsetted_line_works() {
        let line = LineSegment2D::new(Vec2::ZERO, Vec2::ONE);
        let ray = line.ray().offset_origin_by(line.direction() * -10.0);
        assert_eq!(
            ray.intersection(&line),
            RayLine2DIntersection::CollinearOverlap(line)
        );
    }

    #[test]
    fn normal_intersection_works() {
        let line = LineSegment2D::new(Vec2::ZERO, Vec2::ONE);
        let ray = Ray2D::new(Vec2::X, Vec2::Y - Vec2::X);

        assert_eq!(
            ray.intersection(&line),
            RayLine2DIntersection::Intersection(Vec2::ONE * 0.5)
        );
    }
}
