use glam::Vec2;

use crate::d2::def::NeoLineSegment2D;

#[derive(Debug, PartialEq)]
pub enum NeoLine2DIntersection {
    Parallel,
    CollinearNoOverlap,
    CollinearOverlap(Overlap),
    IntersectionInBoth(Vec2),
    IntersectionInFirst(Vec2),
    IntersectionInSecond(Vec2),
    IntersectionOutside(Vec2),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Overlap(pub [NeoLineSegment2D; 3]);

impl Overlap {
    pub fn overlap(&self) -> NeoLineSegment2D {
        self.0[1]
    }
    pub fn non_overlap(&self) -> [NeoLineSegment2D; 2] {
        [self.0[0], self.0[2]]
    }
}

impl NeoLineSegment2D {
    pub fn intersection(&self, other: &Self) -> NeoLine2DIntersection {
        use neo_ray::d2::intersection::NeoRay2DIntersection::*;
        match self.ray().intersection(&other.ray()) {
            Parallel => NeoLine2DIntersection::Parallel,
            Collinear => self.classify_collinear_overlap(other),
            Intersection(intersection_point) => {
                self.classify_intersecting_relation_to(other, intersection_point)
            }
        }
    }

    fn classify_collinear_overlap(&self, other: &Self) -> NeoLine2DIntersection {
        let other_scalars = other.array().map(|v| self.scalar_of(v));
        if other_scalars.iter().any(|s| (0.0..=1.0).contains(s)) {
            self.calculate_collinear_overlap(other_scalars)
        } else {
            NeoLine2DIntersection::CollinearNoOverlap
        }
    }

    fn calculate_collinear_overlap(
        &self,
        [other_scalar_a, other_scalar_b]: [f32; 2],
    ) -> NeoLine2DIntersection {
        let mut all_scalars = [other_scalar_a, other_scalar_b, 0.0, 1.0];
        all_scalars.sort_by(f32::total_cmp);
        let all_points = all_scalars.map(|s| self.inject_scalar(s));
        let split_lines = [
            NeoLineSegment2D::new(all_points[0], all_points[1]),
            NeoLineSegment2D::new(all_points[1], all_points[2]),
            NeoLineSegment2D::new(all_points[2], all_points[3]),
        ];
        NeoLine2DIntersection::CollinearOverlap(Overlap(split_lines))
    }

    fn classify_intersecting_relation_to(
        &self,
        other: &Self,
        intersection_point: Vec2,
    ) -> NeoLine2DIntersection {
        let in_first = self.is_point_on_line(intersection_point);
        let in_second = other.is_point_on_line(intersection_point);
        match (in_first, in_second) {
            (true, true) => NeoLine2DIntersection::IntersectionInBoth(intersection_point),
            (true, false) => NeoLine2DIntersection::IntersectionInFirst(intersection_point),
            (false, true) => NeoLine2DIntersection::IntersectionInSecond(intersection_point),
            (false, false) => NeoLine2DIntersection::IntersectionOutside(intersection_point),
        }
    }
}

#[test]
fn intersection_both_works() {
    let l1 = NeoLineSegment2D::new(Vec2::ZERO, Vec2::ONE);
    let l2 = NeoLineSegment2D::new(Vec2::X, Vec2::Y);
    assert_eq!(
        l1.intersection(&l2),
        NeoLine2DIntersection::IntersectionInBoth(Vec2::ONE * 0.5)
    )
}

#[test]
fn intersection_first_works() {
    let l1 = NeoLineSegment2D::new(Vec2::ONE, Vec2::ONE - Vec2::Y * 0.5);
    let l2 = NeoLineSegment2D::new(Vec2::ZERO, Vec2::X * 2.0);
    assert_eq!(
        l2.intersection(&l1),
        NeoLine2DIntersection::IntersectionInFirst(Vec2::X)
    )
}

#[test]
fn intersection_second_works() {
    let l1 = NeoLineSegment2D::new(Vec2::ONE, Vec2::ONE - Vec2::Y * 0.5);
    let l2 = NeoLineSegment2D::new(Vec2::ZERO, Vec2::X * 2.0);
    assert_eq!(
        l1.intersection(&l2),
        NeoLine2DIntersection::IntersectionInSecond(Vec2::X)
    );
}

#[test]
fn intersection_outside_works() {
    let l1 = NeoLineSegment2D::new(Vec2::X, Vec2::ONE - Vec2::Y * 0.5);
    let l2 = NeoLineSegment2D::new(Vec2::Y, Vec2::ONE - Vec2::X * 0.5);
    assert_eq!(
        l1.intersection(&l2),
        NeoLine2DIntersection::IntersectionOutside(Vec2::ONE)
    );
}

#[test]
fn parallel_works() {
    let l1 = NeoLineSegment2D::UNIT_X;
    let l2 = l1.offset_line_by(Vec2::Y);
    assert_eq!(l1.intersection(&l2), NeoLine2DIntersection::Parallel);
}

#[test]
fn collinear_no_overlap_works() {
    let l1 = NeoLineSegment2D::UNIT_X;
    let l2 = l1.offset_line_by(Vec2::X * 2.0);
    assert_eq!(
        l1.intersection(&l2),
        NeoLine2DIntersection::CollinearNoOverlap
    );
}

#[test]
fn collinear_overlap_works() {
    let l1 = NeoLineSegment2D::UNIT_X;
    let l2 = l1.offset_line_by(Vec2::X * 0.5);
    assert_eq!(
        l1.intersection(&l2),
        NeoLine2DIntersection::CollinearOverlap(Overlap([
            NeoLineSegment2D::UNIT_X.scale_dst_by(0.5),
            NeoLineSegment2D::UNIT_X
                .scale_dst_by(0.5)
                .offset_line_by(Vec2::X * 0.5),
            NeoLineSegment2D::UNIT_X
                .scale_dst_by(0.5)
                .offset_line_by(Vec2::X),
        ]))
    );
}
