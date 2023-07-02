use glam::Vec3;
use neo_line_segment::d3::def::LineSegment3D;

use crate::results::RayRay3DIntersection;
use crate::trait_def::NeoIntersectable;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineLine3DIntersection {
    None,
    Parallel,
    CollinearNoOverlap,
    CollinearOverlap(Line3DOverlap),
    Intersection(Vec3),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line3DOverlap {
    before: LineSegment3D,
    overlap: LineSegment3D,
    after: LineSegment3D,
}

impl Line3DOverlap {
    pub fn overlap(&self) -> LineSegment3D {
        self.overlap
    }
    pub fn non_overlap(&self) -> [LineSegment3D; 2] {
        [self.before, self.after]
    }
}

impl NeoIntersectable for LineSegment3D {
    type Output = LineLine3DIntersection;
    fn intersection(&self, rhs: &Self) -> Self::Output {
        if self.aabb().intersects(&rhs.aabb()) {
            classify_aabbs_intersecting(self, rhs)
        } else {
            classify_aabbs_not_intersecting(self, rhs)
        }
    }
}

pub(crate) fn classify_aabbs_intersecting(
    l1: &LineSegment3D,
    l2: &LineSegment3D,
) -> LineLine3DIntersection {
    match l1.ray().intersection(&l2.ray()) {
        RayRay3DIntersection::Parallel => LineLine3DIntersection::Parallel,
        RayRay3DIntersection::Collinear => classify_collinear_overlap(l1, l2),
        RayRay3DIntersection::Intersection(intersection_point) => {
            classify_intersection_point(l1, l2, intersection_point)
        }
        RayRay3DIntersection::Skewed => LineLine3DIntersection::None,
    }
}

pub(crate) fn classify_aabbs_not_intersecting(
    l1: &LineSegment3D,
    l2: &LineSegment3D,
) -> LineLine3DIntersection {
    if l1.ray().is_point_on_ray(l2.src) && l1.ray().is_point_on_ray(l2.dst) {
        LineLine3DIntersection::CollinearNoOverlap
    } else if l1.is_parallel_to(l2) {
        LineLine3DIntersection::Parallel
    } else {
        LineLine3DIntersection::None
    }
}

pub(crate) fn classify_collinear_overlap(
    l1: &LineSegment3D,
    l2: &LineSegment3D,
) -> LineLine3DIntersection {
    let other_scalars = l2.array().map(|v| l1.scalar_of(v));
    if other_scalars.iter().any(|s| (0.0..=1.0).contains(s)) {
        calculate_collinear_overlap(l1, other_scalars)
    } else {
        LineLine3DIntersection::CollinearNoOverlap
    }
}

pub(crate) fn calculate_collinear_overlap(
    l: &LineSegment3D,
    [other_scalar_a, other_scalar_b]: [f32; 2],
) -> LineLine3DIntersection {
    let mut all_scalars = [other_scalar_a, other_scalar_b, 0.0, 1.0];
    all_scalars.sort_by(f32::total_cmp);
    let all_points = all_scalars.map(|s| l.inject_scalar(s));
    let [before, overlap, after] = [
        LineSegment3D::new(all_points[0], all_points[1]),
        LineSegment3D::new(all_points[1], all_points[2]),
        LineSegment3D::new(all_points[2], all_points[3]),
    ];
    LineLine3DIntersection::CollinearOverlap(Line3DOverlap {
        before,
        overlap,
        after,
    })
}

pub(crate) fn classify_intersection_point(
    l1: &LineSegment3D,
    l2: &LineSegment3D,
    intersection_point: Vec3,
) -> LineLine3DIntersection {
    if l1.is_point_on_line(intersection_point) && l2.is_point_on_line(intersection_point) {
        LineLine3DIntersection::Intersection(intersection_point)
    } else {
        LineLine3DIntersection::None
    }
}

#[test]
fn intersection_in_both_works() {
    let l1 = LineSegment3D::new(Vec3::ZERO, Vec3::ONE);
    let l2 = LineSegment3D::new(Vec3::X, Vec3::Y + Vec3::Z);
    assert_eq!(
        l1.intersection(&l2),
        LineLine3DIntersection::Intersection(Vec3::ONE * 0.5)
    )
}

#[test]
fn intersection_in_first_works() {
    let l1 = LineSegment3D::new(
        Vec3::X + Vec3::Z + Vec3::Y,
        Vec3::X + (Vec3::Z + Vec3::Y) * 0.5,
    );
    let l2 = LineSegment3D::new(Vec3::ZERO, Vec3::X * 2.0);
    assert_eq!(l2.intersection(&l1), LineLine3DIntersection::None)
}

#[test]
fn intersection_in_second_works() {
    let l1 = LineSegment3D::new(
        Vec3::X + Vec3::Z + Vec3::Y,
        Vec3::X + (Vec3::Z + Vec3::Y) * 0.5,
    );
    let l2 = LineSegment3D::new(Vec3::ZERO, Vec3::X * 2.0);
    assert_eq!(l1.intersection(&l2), LineLine3DIntersection::None)
}

#[test]
fn intersection_outside_works() {
    let d1 = Vec3::Y + Vec3::Z;
    let l1 = LineSegment3D::new(Vec3::X + 0.5 * d1, Vec3::X + d1);
    let d2 = Vec3::Y - Vec3::Z;
    let l2 = LineSegment3D::new(Vec3::X + 0.5 * d2, Vec3::X + d2);
    assert_eq!(l1.intersection(&l2), LineLine3DIntersection::None);
}

#[test]
fn parallel_works() {
    let l1 = LineSegment3D::UNIT_X;
    let l2 = l1.offset_line_by(Vec3::Y);
    assert_eq!(l1.intersection(&l2), LineLine3DIntersection::Parallel);
}

#[test]
fn collinear_no_overlap_works() {
    let l1 = LineSegment3D::UNIT_X;
    let l2 = l1.offset_line_by(Vec3::X * 2.0);
    assert_eq!(
        l1.intersection(&l2),
        LineLine3DIntersection::CollinearNoOverlap
    );
}

#[test]
fn collinear_overlap_works() {
    let l1 = LineSegment3D::UNIT_X;
    let l2 = l1.offset_line_by(Vec3::X * 0.5);
    assert_eq!(
        l1.intersection(&l2),
        LineLine3DIntersection::CollinearOverlap(Line3DOverlap {
            before: LineSegment3D::UNIT_X.scale_dst_by(0.5),
            overlap: LineSegment3D::UNIT_X
                .scale_dst_by(0.5)
                .offset_line_by(Vec3::X * 0.5),
            after: LineSegment3D::UNIT_X
                .scale_dst_by(0.5)
                .offset_line_by(Vec3::X),
        })
    );
}
