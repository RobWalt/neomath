use geo::{Contains, Intersects};
use geo_glam_interop::to_geo::ConvertToGeo;
use glam::Vec2;

use crate::d2::def::AABB2D;

impl AABB2D {
    pub fn contains(&self, point: Vec2) -> bool {
        let rect = self.as_rect();
        let point = point.to_geo();
        rect.contains(&point) || rect.intersects(&point)
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.contains(other.min) || self.contains(other.max)
    }

    pub fn as_rect(&self) -> geo::Rect<f32> {
        geo::Rect::new(self.min.to_geo(), self.max.to_geo())
    }
}

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
    let aabb1 = AABB2D::new(Vec2::ZERO, Vec2::ONE);
    assert_eq!(aabb1.intersects(&aabb1), true);
}

#[test]
fn flat_aabb_intersection_works() {
    let aabb1 = AABB2D::new(Vec2::ZERO, Vec2::X);
    let aabb2 = AABB2D::new(Vec2::X * 0.5, Vec2::X * 1.5);
    assert_eq!(aabb1.intersects(&aabb2), true);
}

#[test]
fn flat_aabb_self_intersection_works() {
    let aabb1 = AABB2D::new(Vec2::ZERO, Vec2::X);
    assert_eq!(aabb1.intersects(&aabb1), true);
}
