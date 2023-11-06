use std::ops::RangeInclusive;

use glam::Vec3;

use crate::d3::def::AABB3D;

impl AABB3D {
    pub fn x_range(&self) -> RangeInclusive<f32> {
        self.min.x..=self.max.x
    }

    pub fn y_range(&self) -> RangeInclusive<f32> {
        self.min.y..=self.max.y
    }

    pub fn z_range(&self) -> RangeInclusive<f32> {
        self.min.z..=self.max.z
    }

    pub fn center(&self) -> Vec3 {
        (self.max + self.min) * 0.5
    }

    pub fn half_extends(&self) -> Vec3 {
        (self.max - self.min) * 0.5
    }

    pub fn contains(&self, point: Vec3) -> bool {
        self.x_range().contains(&point.x)
            && self.y_range().contains(&point.y)
            && self.z_range().contains(&point.z)
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
}

#[cfg(test)]
mod aabb_impls {
    use glam::Vec3;

    use crate::d3::def::AABB3D;

    #[test]
    fn internal_point_is_contained() {
        let aabb = AABB3D::new(Vec3::ZERO, Vec3::ONE);
        assert!(aabb.contains(Vec3::ONE * 0.5));
    }

    #[test]
    fn border_point_contained() {
        let aabb = AABB3D::new(Vec3::ZERO, Vec3::ONE);
        assert!(aabb.contains(Vec3::X * 0.5));
    }

    #[test]
    fn corner_point_contained() {
        let aabb = AABB3D::new(Vec3::ZERO, Vec3::ONE);
        assert!(aabb.contains(Vec3::ONE));
    }

    #[test]
    fn flat_2d_aabb_contains_point_on_face() {
        let aabb = AABB3D::new(Vec3::ZERO, Vec3::X + Vec3::Y);
        assert!(aabb.contains((Vec3::X + Vec3::Y) * 0.5));
    }

    #[test]
    fn flat_2d_aabb_contains_point_on_line() {
        let aabb = AABB3D::new(Vec3::ZERO, Vec3::X + Vec3::Y);
        assert!(aabb.contains(Vec3::X * 0.5));
    }

    #[test]
    fn flat_2d_aabb_contains_point_on_corner() {
        let aabb = AABB3D::new(Vec3::ZERO, Vec3::X + Vec3::Y);
        assert!(aabb.contains(Vec3::X));
    }

    #[test]
    fn flat_aabb_contains_point_on_line() {
        let aabb = AABB3D::new(Vec3::ZERO, Vec3::X);
        assert!(aabb.contains(Vec3::X * 0.5));
    }

    #[test]
    fn flat_aabb_contains_endpoint_of_line() {
        let aabb = AABB3D::new(Vec3::ZERO, Vec3::X);
        assert!(aabb.contains(Vec3::X));
    }

    #[test]
    fn aabb_intersection_works() {
        let aabb1 = AABB3D::new(Vec3::ZERO, Vec3::ONE);
        let aabb2 = AABB3D::new(Vec3::ONE * 0.5, Vec3::ONE * 1.5);
        assert!(aabb1.intersects(&aabb2));
    }

    #[test]
    fn aabb_self_intersection_works() {
        let aabb = AABB3D::new(Vec3::ZERO, Vec3::ONE);
        assert!(aabb.intersects(&aabb));
    }

    #[test]
    fn flat_aabb_intersection_works() {
        let aabb1 = AABB3D::new(Vec3::ZERO, Vec3::X);
        let aabb2 = AABB3D::new(Vec3::X * 0.5, Vec3::X * 1.5);
        assert!(aabb1.intersects(&aabb2));
    }

    #[test]
    fn flat_aabb_self_intersection_works() {
        let aabb = AABB3D::new(Vec3::ZERO, Vec3::X);
        assert!(aabb.intersects(&aabb));
    }

    #[test]
    fn small_intersects_big() {
        let aabb_big = AABB3D::new(Vec3::ZERO, Vec3::ONE * 3.0);
        let aabb_small = AABB3D::new(Vec3::ONE, Vec3::ONE * 2.0);
        assert!(aabb_small.intersects(&aabb_big))
    }

    #[test]
    fn big_intersects_small() {
        let aabb_big = AABB3D::new(Vec3::ZERO, Vec3::ONE * 3.0);
        let aabb_small = AABB3D::new(Vec3::ONE, Vec3::ONE * 2.0);
        assert!(aabb_big.intersects(&aabb_small))
    }

    #[test]
    fn corners_not_contained_in_each_other_works() {
        let aabb_a = AABB3D::new(
            -Vec3::X - Vec3::Y * 0.5 - Vec3::Z,
            Vec3::X + Vec3::Y * 0.5 + Vec3::Z,
        );
        let aabb_b = AABB3D::new(
            -Vec3::X * 0.5 - Vec3::Y - Vec3::Z,
            Vec3::X * 0.5 + Vec3::Y + Vec3::Z,
        );
        assert!(aabb_a.intersects(&aabb_b))
    }
}
