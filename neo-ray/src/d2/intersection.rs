use glam::Vec2;

use crate::d2::def::Ray2D;

#[derive(Debug, PartialEq)]
pub enum RayRay2DIntersection {
    Parallel,
    Collinear,
    Intersection(Vec2),
}

impl Ray2D {
    pub fn intersection(&self, other: &Self) -> RayRay2DIntersection {
        if self.is_parallel_to(other) {
            self.classify_parallel_relation_to(other)
        } else {
            self.classify_intersecting_relation_to(other)
        }
    }

    fn classify_parallel_relation_to(&self, other: &Self) -> RayRay2DIntersection {
        if self.is_point_on_ray(other.origin) {
            RayRay2DIntersection::Collinear
        } else {
            RayRay2DIntersection::Parallel
        }
    }

    fn classify_intersecting_relation_to(&self, other: &Self) -> RayRay2DIntersection {
        let intersection_point = self.calculate_intersection_point(other);
        RayRay2DIntersection::Intersection(intersection_point)
    }

    fn calculate_intersection_point(&self, other: &Self) -> Vec2 {
        // Calculate the intersection point using line-line intersection formula
        let d = self.direction();
        let e = other.direction();
        let f = self.origin - other.origin;

        let cross_de = d.perp_dot(e);
        let cross_ef = e.perp_dot(f);

        let t = cross_ef / cross_de;

        self.origin + d * t
    }
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
        l1.classify_intersecting_relation_to(&l2),
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
