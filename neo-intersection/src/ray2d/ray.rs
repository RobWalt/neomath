use glam::Vec2;
use neo_ray::d2::def::Ray2D;

use crate::trait_def::NeoIntersectable;

#[derive(Debug, PartialEq)]
pub enum RayRay2DIntersection {
    Parallel,
    Collinear,
    Intersection(Vec2),
}

impl RayRay2DIntersection {
    pub fn intersection_point(&self) -> Option<Vec2> {
        match self {
            RayRay2DIntersection::Intersection(p) => Some(*p),
            _ => None,
        }
    }
}

impl NeoIntersectable for Ray2D {
    type Output = RayRay2DIntersection;

    fn intersection(&self, rhs: &Self) -> Self::Output {
        if self.is_parallel_to(rhs) {
            classify_parallel_relation_to(self, rhs)
        } else {
            classify_intersecting_relation_to(self, rhs)
        }
    }
}

pub(crate) fn classify_parallel_relation_to(r1: &Ray2D, r2: &Ray2D) -> RayRay2DIntersection {
    if r1.is_point_on_ray(r2.origin) {
        RayRay2DIntersection::Collinear
    } else {
        RayRay2DIntersection::Parallel
    }
}

pub(crate) fn classify_intersecting_relation_to(r1: &Ray2D, r2: &Ray2D) -> RayRay2DIntersection {
    let intersection_point = calculate_intersection_point(r1, r2);
    RayRay2DIntersection::Intersection(intersection_point)
}

pub(crate) fn calculate_intersection_point(r1: &Ray2D, r2: &Ray2D) -> Vec2 {
    // Calculate the intersection point using line-line intersection formula
    let d = r1.direction();
    let e = r2.direction();
    let f = r1.origin - r2.origin;

    let cross_de = d.perp_dot(e);
    let cross_ef = e.perp_dot(f);

    let t = cross_ef / cross_de;

    r1.origin + d * t
}

#[test]
fn intersection_first_works() {
    let l1 = Ray2D::new(Vec2::ONE, -Vec2::Y * 0.5);
    let l2 = Ray2D::new(Vec2::ZERO, Vec2::X * 2.0);
    assert_eq!(
        l2.intersection(&l1),
        RayRay2DIntersection::Intersection(Vec2::X)
    )
}

#[test]
fn intersection_both_works() {
    let l1 = Ray2D::new(Vec2::ZERO, Vec2::ONE);
    let l2 = Ray2D::new(Vec2::X, -Vec2::X + Vec2::Y);
    assert_eq!(
        classify_intersecting_relation_to(&l1, &l2),
        RayRay2DIntersection::Intersection(Vec2::ONE * 0.5)
    )
}

#[test]
fn parallel_works() {
    let l1 = Ray2D::X;
    let l2 = l1.offset_origin_by(Vec2::Y);
    assert_eq!(l1.intersection(&l2), RayRay2DIntersection::Parallel);
}

#[test]
fn collinear_no_overlap_works() {
    let l1 = Ray2D::X;
    let l2 = l1.offset_origin_by(Vec2::X);
    assert_eq!(l1.intersection(&l2), RayRay2DIntersection::Collinear);
}
