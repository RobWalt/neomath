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

    pub fn contains(&self, point: Vec3) -> bool {
        self.x_range().contains(&point.x)
            && self.y_range().contains(&point.y)
            && self.z_range().contains(&point.z)
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.contains(other.min) || self.contains(other.max)
    }
}

#[test]
fn internal_point_is_contained() {
    let aabb = AABB3D::new(Vec3::ZERO, Vec3::ONE);
    assert_eq!(aabb.contains(Vec3::ONE * 0.5), true);
}

#[test]
fn border_point_contained() {
    let aabb = AABB3D::new(Vec3::ZERO, Vec3::ONE);
    assert_eq!(aabb.contains(Vec3::X * 0.5), true);
}

#[test]
fn corner_point_contained() {
    let aabb = AABB3D::new(Vec3::ZERO, Vec3::ONE);
    assert_eq!(aabb.contains(Vec3::ONE), true);
}

#[test]
fn flat_2d_aabb_contains_point_on_face() {
    let aabb = AABB3D::new(Vec3::ZERO, Vec3::X + Vec3::Y);
    assert_eq!(aabb.contains((Vec3::X + Vec3::Y) * 0.5), true);
}

#[test]
fn flat_2d_aabb_contains_point_on_line() {
    let aabb = AABB3D::new(Vec3::ZERO, Vec3::X + Vec3::Y);
    assert_eq!(aabb.contains(Vec3::X * 0.5), true);
}

#[test]
fn flat_2d_aabb_contains_point_on_corner() {
    let aabb = AABB3D::new(Vec3::ZERO, Vec3::X + Vec3::Y);
    assert_eq!(aabb.contains(Vec3::X), true);
}

#[test]
fn flat_aabb_contains_point_on_line() {
    let aabb = AABB3D::new(Vec3::ZERO, Vec3::X);
    assert_eq!(aabb.contains(Vec3::X * 0.5), true);
}

#[test]
fn flat_aabb_contains_endpoint_of_line() {
    let aabb = AABB3D::new(Vec3::ZERO, Vec3::X);
    assert_eq!(aabb.contains(Vec3::X), true);
}

#[test]
fn aabb_intersection_works() {
    let aabb1 = AABB3D::new(Vec3::ZERO, Vec3::ONE);
    let aabb2 = AABB3D::new(Vec3::ONE * 0.5, Vec3::ONE * 1.5);
    assert_eq!(aabb1.intersects(&aabb2), true);
}

#[test]
fn aabb_self_intersection_works() {
    let aabb = AABB3D::new(Vec3::ZERO, Vec3::ONE);
    assert_eq!(aabb.intersects(&aabb), true);
}

#[test]
fn flat_aabb_intersection_works() {
    let aabb1 = AABB3D::new(Vec3::ZERO, Vec3::X);
    let aabb2 = AABB3D::new(Vec3::X * 0.5, Vec3::X * 1.5);
    assert_eq!(aabb1.intersects(&aabb2), true);
}

#[test]
fn flat_aabb_self_intersection_works() {
    let aabb = AABB3D::new(Vec3::ZERO, Vec3::X);
    assert_eq!(aabb.intersects(&aabb), true);
}
