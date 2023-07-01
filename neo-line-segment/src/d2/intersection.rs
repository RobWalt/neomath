use glam::Vec2;

use crate::d2::def::LineSegment2D;

#[derive(Debug, PartialEq)]
pub enum Line2DIntersection {
    Parallel,
    CollinearNoOverlap,
    CollinearOverlap(Line2DOverlap),
    IntersectionInBoth(Vec2),
    IntersectionInFirst(Vec2),
    IntersectionInSecond(Vec2),
    IntersectionOutside(Vec2),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line2DOverlap(pub [LineSegment2D; 3]);

impl Line2DOverlap {
    pub fn overlap(&self) -> LineSegment2D {
        self.0[1]
    }
    pub fn non_overlap(&self) -> [LineSegment2D; 2] {
        [self.0[0], self.0[2]]
    }
}

impl LineSegment2D {
    pub fn intersection(&self, other: &Self) -> Line2DIntersection {
        use neo_ray::d2::intersection::RayRay2DIntersection::*;
        match self.ray().intersection(&other.ray()) {
            Parallel => Line2DIntersection::Parallel,
            Collinear => self.classify_collinear_overlap(other),
            Intersection(intersection_point) => {
                self.classify_intersecting_relation_to(other, intersection_point)
            }
        }
    }

    fn classify_collinear_overlap(&self, other: &Self) -> Line2DIntersection {
        let other_scalars = other.array().map(|v| self.scalar_of(v));
        if other_scalars.iter().any(|s| (0.0..=1.0).contains(s)) {
            self.calculate_collinear_overlap(other_scalars)
        } else {
            Line2DIntersection::CollinearNoOverlap
        }
    }

    fn calculate_collinear_overlap(
        &self,
        [other_scalar_a, other_scalar_b]: [f32; 2],
    ) -> Line2DIntersection {
        let mut all_scalars = [other_scalar_a, other_scalar_b, 0.0, 1.0];
        all_scalars.sort_by(f32::total_cmp);
        let all_points = all_scalars.map(|s| self.inject_scalar(s));
        let split_lines = [
            LineSegment2D::new(all_points[0], all_points[1]),
            LineSegment2D::new(all_points[1], all_points[2]),
            LineSegment2D::new(all_points[2], all_points[3]),
        ];
        Line2DIntersection::CollinearOverlap(Line2DOverlap(split_lines))
    }

    fn classify_intersecting_relation_to(
        &self,
        other: &Self,
        intersection_point: Vec2,
    ) -> Line2DIntersection {
        let in_first = self.is_point_on_line(intersection_point);
        let in_second = other.is_point_on_line(intersection_point);
        match (in_first, in_second) {
            (true, true) => Line2DIntersection::IntersectionInBoth(intersection_point),
            (true, false) => Line2DIntersection::IntersectionInFirst(intersection_point),
            (false, true) => Line2DIntersection::IntersectionInSecond(intersection_point),
            (false, false) => Line2DIntersection::IntersectionOutside(intersection_point),
        }
    }
}

#[test]
fn intersection_both_works() {
    let l1 = LineSegment2D::new(Vec2::ZERO, Vec2::ONE);
    let l2 = LineSegment2D::new(Vec2::X, Vec2::Y);
    assert_eq!(
        l1.intersection(&l2),
        Line2DIntersection::IntersectionInBoth(Vec2::ONE * 0.5)
    )
}

#[test]
fn intersection_first_works() {
    let l1 = LineSegment2D::new(Vec2::ONE, Vec2::ONE - Vec2::Y * 0.5);
    let l2 = LineSegment2D::new(Vec2::ZERO, Vec2::X * 2.0);
    assert_eq!(
        l2.intersection(&l1),
        Line2DIntersection::IntersectionInFirst(Vec2::X)
    )
}

#[test]
fn intersection_second_works() {
    let l1 = LineSegment2D::new(Vec2::ONE, Vec2::ONE - Vec2::Y * 0.5);
    let l2 = LineSegment2D::new(Vec2::ZERO, Vec2::X * 2.0);
    assert_eq!(
        l1.intersection(&l2),
        Line2DIntersection::IntersectionInSecond(Vec2::X)
    );
}

#[test]
fn intersection_outside_works() {
    let l1 = LineSegment2D::new(Vec2::X, Vec2::ONE - Vec2::Y * 0.5);
    let l2 = LineSegment2D::new(Vec2::Y, Vec2::ONE - Vec2::X * 0.5);
    assert_eq!(
        l1.intersection(&l2),
        Line2DIntersection::IntersectionOutside(Vec2::ONE)
    );
}

#[test]
fn parallel_works() {
    let l1 = LineSegment2D::UNIT_X;
    let l2 = l1.offset_line_by(Vec2::Y);
    assert_eq!(l1.intersection(&l2), Line2DIntersection::Parallel);
}

#[test]
fn collinear_no_overlap_works() {
    let l1 = LineSegment2D::UNIT_X;
    let l2 = l1.offset_line_by(Vec2::X * 2.0);
    assert_eq!(l1.intersection(&l2), Line2DIntersection::CollinearNoOverlap);
}

#[test]
fn collinear_overlap_works() {
    let l1 = LineSegment2D::UNIT_X;
    let l2 = l1.offset_line_by(Vec2::X * 0.5);
    assert_eq!(
        l1.intersection(&l2),
        Line2DIntersection::CollinearOverlap(Line2DOverlap([
            LineSegment2D::UNIT_X.scale_dst_by(0.5),
            LineSegment2D::UNIT_X
                .scale_dst_by(0.5)
                .offset_line_by(Vec2::X * 0.5),
            LineSegment2D::UNIT_X
                .scale_dst_by(0.5)
                .offset_line_by(Vec2::X),
        ]))
    );
}
