use glam::Vec2;
use neo_ray::d2::intersection::RayRay2DIntersection;

use crate::d2::def::LineSegment2D;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineLine2DIntersection {
    Parallel,
    CollinearNoOverlap,
    CollinearOverlap(Line2DOverlap),
    /// This is a real intersection where the intersection point is located in both lines that were
    /// intersected. If you're interested in the general intersection point which may be located
    /// outside the line, consider using [`LineSegment2D::ray_intersection`]
    Intersection(Vec2),
    None,
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

impl LineSegment2D {
    pub fn intersection(&self, other: &Self) -> LineLine2DIntersection {
        if self.aabb().intersects(&other.aabb()) {
            self.classify_aabbs_intersected(other)
        } else {
            self.classify_aabbs_didnt_intersect(other)
        }
    }

    pub fn ray_intersection(&self, other: &Self) -> RayRay2DIntersection {
        self.ray().intersection(&other.ray())
    }

    fn classify_aabbs_intersected(&self, other: &Self) -> LineLine2DIntersection {
        use neo_ray::d2::intersection::RayRay2DIntersection::*;
        match self.ray_intersection(other) {
            Parallel => LineLine2DIntersection::Parallel,
            Collinear => self.classify_collinear_overlap(other),
            Intersection(intersection_point) => {
                self.classify_intersecting_relation_to(other, intersection_point)
            }
        }
    }

    fn classify_aabbs_didnt_intersect(&self, other: &Self) -> LineLine2DIntersection {
        if self.ray().is_point_on_ray(other.src) && self.ray().is_point_on_ray(other.dst) {
            LineLine2DIntersection::CollinearNoOverlap
        } else if self.is_parallel_to(other) {
            LineLine2DIntersection::Parallel
        } else {
            LineLine2DIntersection::None
        }
    }

    fn classify_collinear_overlap(&self, other: &Self) -> LineLine2DIntersection {
        let other_scalars = other.array().map(|v| self.scalar_of(v));
        if other_scalars.iter().any(|s| (0.0..=1.0).contains(s)) {
            self.calculate_collinear_overlap(other_scalars)
        } else {
            LineLine2DIntersection::CollinearNoOverlap
        }
    }

    fn calculate_collinear_overlap(
        &self,
        [other_scalar_a, other_scalar_b]: [f32; 2],
    ) -> LineLine2DIntersection {
        let mut all_scalars = [other_scalar_a, other_scalar_b, 0.0, 1.0];
        all_scalars.sort_by(f32::total_cmp);
        let all_points = all_scalars.map(|s| self.inject_scalar(s));
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

    fn classify_intersecting_relation_to(
        &self,
        other: &Self,
        intersection_point: Vec2,
    ) -> LineLine2DIntersection {
        let in_first = self.is_point_on_line(intersection_point);
        let in_second = other.is_point_on_line(intersection_point);
        match (in_first, in_second) {
            (true, true) => LineLine2DIntersection::Intersection(intersection_point),
            _ => LineLine2DIntersection::None,
        }
    }
}

#[test]
fn intersection_both_works() {
    let l1 = LineSegment2D::new(Vec2::ZERO, Vec2::ONE);
    let l2 = LineSegment2D::new(Vec2::X, Vec2::Y);
    assert_eq!(
        l1.intersection(&l2),
        LineLine2DIntersection::Intersection(Vec2::ONE * 0.5)
    )
}

#[test]
fn intersection_first_works() {
    let l1 = LineSegment2D::new(Vec2::ONE, Vec2::ONE - Vec2::Y * 0.5);
    let l2 = LineSegment2D::new(Vec2::ZERO, Vec2::X * 2.0);
    assert_eq!(l2.intersection(&l1), LineLine2DIntersection::None)
}

#[test]
fn intersection_second_works() {
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
