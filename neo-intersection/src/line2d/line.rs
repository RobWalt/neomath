use glam::Vec2;
use neo_bounded::traits::NeoBounded2D;
use neo_line_segment::d2::def::LineSegment2D;

use crate::ray2d::ray::RayRay2DIntersection;
use crate::trait_def::NeoIntersectable;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineLine2DIntersection {
    None,
    Parallel,
    CollinearNoOverlap,
    CollinearOverlap(Line2DOverlap),
    /// This is a real intersection where the intersection point is located in both lines that were
    /// intersected. If you're interested in the general intersection point which may be located
    /// outside the line, consider using [`LineSegment2D::ray_intersection`]
    Intersection(Vec2),
}

impl LineLine2DIntersection {
    pub fn intersection_point(&self) -> Option<Vec2> {
        match self {
            LineLine2DIntersection::Intersection(p) => Some(*p),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line2DOverlap {
    before: LineSegment2D,
    overlap: LineSegment2D,
    after: LineSegment2D,
}

impl Line2DOverlap {
    pub fn overlap(&self) -> LineSegment2D {
        self.overlap
    }
    pub fn non_overlap(&self) -> [LineSegment2D; 2] {
        [self.before, self.after]
    }
}

impl NeoIntersectable for LineSegment2D {
    type Output = LineLine2DIntersection;

    fn intersection(&self, rhs: &Self) -> Self::Output {
        let self_aabb = self.aabb();
        let rhs_aabb = rhs.aabb();
        if self_aabb.intersects(&rhs_aabb) {
            classify_aabbs_intersecting(self, rhs)
        } else {
            classify_aabbs_not_intersecting(self, rhs)
        }
    }
}

pub(crate) fn classify_aabbs_intersecting(
    l1: &LineSegment2D,
    l2: &LineSegment2D,
) -> LineLine2DIntersection {
    let inter = l1.ray().intersection(&l2.ray());
    match inter {
        RayRay2DIntersection::Parallel => LineLine2DIntersection::Parallel,
        RayRay2DIntersection::Collinear => classify_collinear_overlap(l1, l2),
        RayRay2DIntersection::Intersection(intersection_point) => {
            classify_intersecting_relation_to(l1, l2, intersection_point)
        }
    }
}

pub(crate) fn classify_aabbs_not_intersecting(
    l1: &LineSegment2D,
    l2: &LineSegment2D,
) -> LineLine2DIntersection {
    if l1.ray().is_point_on_ray(l2.src) && l1.ray().is_point_on_ray(l2.dst) {
        LineLine2DIntersection::CollinearNoOverlap
    } else if l1.is_parallel_to(l2) {
        LineLine2DIntersection::Parallel
    } else {
        LineLine2DIntersection::None
    }
}

pub(crate) fn classify_collinear_overlap(
    l1: &LineSegment2D,
    l2: &LineSegment2D,
) -> LineLine2DIntersection {
    let other_scalars = l2.array().map(|v| l1.scalar_of(v));
    if other_scalars.iter().any(|s| (0.0..=1.0).contains(s)) {
        calculate_collinear_overlap(l1, other_scalars)
    } else {
        LineLine2DIntersection::CollinearNoOverlap
    }
}

pub(crate) fn calculate_collinear_overlap(
    l: &LineSegment2D,
    [other_scalar_a, other_scalar_b]: [f32; 2],
) -> LineLine2DIntersection {
    let mut all_scalars = [other_scalar_a, other_scalar_b, 0.0, 1.0];
    all_scalars.sort_by(f32::total_cmp);
    let all_points = all_scalars.map(|s| l.inject_scalar(s));
    let [before, overlap, after] = [
        LineSegment2D::new(all_points[0], all_points[1]),
        LineSegment2D::new(all_points[1], all_points[2]),
        LineSegment2D::new(all_points[2], all_points[3]),
    ];
    LineLine2DIntersection::CollinearOverlap(Line2DOverlap {
        before,
        overlap,
        after,
    })
}

pub(crate) fn classify_intersecting_relation_to(
    l1: &LineSegment2D,
    l2: &LineSegment2D,
    intersection_point: Vec2,
) -> LineLine2DIntersection {
    let in_first = l1.is_point_on_line(intersection_point);
    let in_second = l2.is_point_on_line(intersection_point);
    match (in_first, in_second) {
        (true, true) => LineLine2DIntersection::Intersection(intersection_point),
        _ => LineLine2DIntersection::None,
    }
}

#[test]
fn intersection_in_both_works() {
    let l1 = LineSegment2D::new(Vec2::ZERO, Vec2::ONE);
    let l2 = LineSegment2D::new(Vec2::X, Vec2::Y);
    assert_eq!(
        l1.intersection(&l2),
        LineLine2DIntersection::Intersection(Vec2::ONE * 0.5)
    )
}

#[test]
fn intersection_in_first_works() {
    let l1 = LineSegment2D::new(Vec2::ONE, Vec2::ONE - Vec2::Y * 0.5);
    let l2 = LineSegment2D::new(Vec2::ZERO, Vec2::X * 2.0);
    assert_eq!(l2.intersection(&l1), LineLine2DIntersection::None)
}

#[test]
fn intersection_in_second_works() {
    let l1 = LineSegment2D::new(Vec2::ONE, Vec2::ONE - Vec2::Y * 0.5);
    let l2 = LineSegment2D::new(Vec2::ZERO, Vec2::X * 2.0);
    assert_eq!(l1.intersection(&l2), LineLine2DIntersection::None);
}

#[test]
fn intersection_outside_works() {
    let l1 = LineSegment2D::new(Vec2::X, Vec2::ONE - Vec2::Y * 0.5);
    let l2 = LineSegment2D::new(Vec2::Y, Vec2::ONE - Vec2::X * 0.5);
    assert_eq!(l1.intersection(&l2), LineLine2DIntersection::None);
}

#[test]
fn parallel_works() {
    let l1 = LineSegment2D::UNIT_X;
    let l2 = l1.offset_line_by(Vec2::Y);
    assert_eq!(l1.intersection(&l2), LineLine2DIntersection::Parallel);
}

#[test]
fn collinear_no_overlap_works() {
    let l1 = LineSegment2D::UNIT_X;
    let l2 = l1.offset_line_by(Vec2::X * 2.0);
    assert_eq!(
        l1.intersection(&l2),
        LineLine2DIntersection::CollinearNoOverlap
    );
}

#[test]
fn collinear_overlap_works() {
    let l1 = LineSegment2D::UNIT_X;
    let l2 = l1.offset_line_by(Vec2::X * 0.5);
    assert_eq!(
        l1.intersection(&l2),
        LineLine2DIntersection::CollinearOverlap(Line2DOverlap {
            before: LineSegment2D::UNIT_X.scale_dst_by(0.5),
            overlap: LineSegment2D::UNIT_X
                .scale_dst_by(0.5)
                .offset_line_by(Vec2::X * 0.5),
            after: LineSegment2D::UNIT_X
                .scale_dst_by(0.5)
                .offset_line_by(Vec2::X),
        })
    );
}
