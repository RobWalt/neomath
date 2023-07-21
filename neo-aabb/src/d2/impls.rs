use std::ops::RangeInclusive;

use geo::{Contains, Intersects};
use glam::Vec2;
use neo_geo_glam_interop::to_geo::ConvertToGeo;

use crate::d2::def::AABB2D;

impl AABB2D {
    pub fn x_range(&self) -> RangeInclusive<f32> {
        self.min.x..=self.max.x
    }

    pub fn y_range(&self) -> RangeInclusive<f32> {
        self.min.y..=self.max.y
    }

    pub fn center(&self) -> Vec2 {
        (self.max + self.min) * 0.5
    }

    pub fn half_extends(&self) -> Vec2 {
        (self.max - self.min) * 0.5
    }

    pub fn contains(&self, point: Vec2) -> bool {
        let rect = self.as_rect();
        let point = point.to_geo();
        rect.contains(&point) || rect.intersects(&point)
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let center_diff = (self.center() - other.center()).abs();
        let combined_half_extends = self.half_extends() + other.half_extends();

        center_diff
            .to_array()
            .iter()
            .zip(combined_half_extends.to_array().iter())
            .all(|(dist, max_dist)| dist <= max_dist)
    }

    pub fn as_rect(&self) -> geo::Rect<f32> {
        geo::Rect::new(self.min.to_geo(), self.max.to_geo())
    }
}

#[cfg(test)]
mod aabb_impls {

    use crate::d2::def::AABB2D;
    use glam::Vec2;

    #[test]
    fn internal_point_is_contained() {
        let aabb = AABB2D::new(Vec2::ZERO, Vec2::ONE);
        assert_eq!(aabb.contains(Vec2::ONE * 0.5), true);
    }

    #[test]
    fn border_point_contained() {
        let aabb = AABB2D::new(Vec2::ZERO, Vec2::ONE);
        assert_eq!(aabb.contains(Vec2::X * 0.5), true);
    }

    #[test]
    fn corner_point_contained() {
        let aabb = AABB2D::new(Vec2::ZERO, Vec2::ONE);
        assert_eq!(aabb.contains(Vec2::ONE), true);
    }

    #[test]
    fn flat_aabb_contains_point_on_line() {
        let aabb = AABB2D::new(Vec2::ZERO, Vec2::X);
        assert_eq!(aabb.contains(Vec2::X * 0.5), true);
    }

    #[test]
    fn flat_aabb_contains_endpoint_of_line() {
        let aabb = AABB2D::new(Vec2::ZERO, Vec2::X);
        assert_eq!(aabb.contains(Vec2::X), true);
    }

    #[test]
    fn aabb_intersection_works() {
        let aabb1 = AABB2D::new(Vec2::ZERO, Vec2::ONE);
        let aabb2 = AABB2D::new(Vec2::ONE * 0.5, Vec2::ONE * 1.5);
        assert_eq!(aabb1.intersects(&aabb2), true);
    }

    #[test]
    fn aabb_self_intersection_works() {
        let aabb = AABB2D::new(Vec2::ZERO, Vec2::ONE);
        assert_eq!(aabb.intersects(&aabb), true);
    }

    #[test]
    fn flat_aabb_intersection_works() {
        let aabb1 = AABB2D::new(Vec2::ZERO, Vec2::X);
        let aabb2 = AABB2D::new(Vec2::X * 0.5, Vec2::X * 1.5);
        assert_eq!(aabb1.intersects(&aabb2), true);
    }

    #[test]
    fn flat_aabb_self_intersection_works() {
        let aabb = AABB2D::new(Vec2::ZERO, Vec2::X);
        assert_eq!(aabb.intersects(&aabb), true);
    }

    #[test]
    fn small_intersects_big() {
        let aabb_big = AABB2D::new(Vec2::ZERO, Vec2::ONE * 3.0);
        let aabb_small = AABB2D::new(Vec2::ONE, Vec2::ONE * 2.0);
        assert!(aabb_small.intersects(&aabb_big))
    }

    #[test]
    fn big_intersects_small() {
        let aabb_big = AABB2D::new(Vec2::ZERO, Vec2::ONE * 3.0);
        let aabb_small = AABB2D::new(Vec2::ONE, Vec2::ONE * 2.0);
        assert!(aabb_big.intersects(&aabb_small))
    }

    #[test]
    fn corners_not_contained_in_each_other_works() {
        let aabb_a = AABB2D::new(-Vec2::X - Vec2::Y * 0.5, Vec2::X + Vec2::Y * 0.5);
        let aabb_b = AABB2D::new(-Vec2::X * 0.5 - Vec2::Y, Vec2::X * 0.5 + Vec2::Y);
        assert!(aabb_a.intersects(&aabb_b))
    }
}
