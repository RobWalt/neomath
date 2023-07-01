#![allow(non_snake_case)]
use glam::{Mat2, Vec2, Vec3};

use crate::d3::constants::NEO_LINE_RAY_3D_EPS;
use crate::d3::def::NeoLineRay3D;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NeoRayRayIntersection {
    Parallel,
    Collinear,
    Intersection(Vec3),
    Skewed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ZeroDim {
    X,
    Y,
    Z,
}

impl ZeroDim {
    fn make_getter(&self) -> fn(Vec3) -> f32 {
        match self {
            ZeroDim::X => |vec| vec.x,
            ZeroDim::Y => |vec| vec.y,
            ZeroDim::Z => |vec| vec.z,
        }
    }

    fn others(&self) -> [Self; 2] {
        match self {
            ZeroDim::X => [Self::Y, Self::Z],
            ZeroDim::Y => [Self::X, Self::Z],
            ZeroDim::Z => [Self::X, Self::Y],
        }
    }
}

impl NeoLineRay3D {
    pub fn intersection_ray(&self, other: &Self) -> NeoRayRayIntersection {
        if self.is_parallel_to(other) {
            self.classify_parallel_relation_to(other)
        } else {
            self.classify_intersecting_relation_to(other)
        }
    }

    fn classify_parallel_relation_to(&self, other: &Self) -> NeoRayRayIntersection {
        if self.is_point_on_ray(other.origin) {
            NeoRayRayIntersection::Collinear
        } else {
            NeoRayRayIntersection::Parallel
        }
    }

    fn classify_intersecting_relation_to(&self, other: &Self) -> NeoRayRayIntersection {
        match self.calculate_intersection_point(other) {
            Some(intersection_point) => NeoRayRayIntersection::Intersection(intersection_point),
            None => NeoRayRayIntersection::Skewed,
        }
    }

    fn calculate_intersection_point(&self, other: &Self) -> Option<Vec3> {
        let dir1 = self.direction;
        let dir2 = other.direction;

        let dirs: [[f32; 2]; 3] = [[dir1.x, dir2.x], [dir1.y, dir2.y], [dir1.z, dir2.z]];
        let near_zero = |x: f32| x.abs() < NEO_LINE_RAY_3D_EPS;

        let maybe_scalar = match dirs {
            // in these cases it is more like a 2D intersection
            [[a, b], _, _] if near_zero(a) && near_zero(b) => {
                self.solve_intersection_equations_dim_zero(other, ZeroDim::X)
            }
            [_, [a, b], _] if near_zero(a) && near_zero(b) => {
                self.solve_intersection_equations_dim_zero(other, ZeroDim::Y)
            }
            [_, _, [a, b]] if near_zero(a) && near_zero(b) => {
                self.solve_intersection_equations_dim_zero(other, ZeroDim::Z)
            }
            _ => self.solve_overdetermined_intersection_system(other),
        };

        maybe_scalar.map(|s| self.origin + s * self.direction)
    }

    // from https://stackoverflow.com/a/34604574
    fn solve_overdetermined_intersection_system(&self, other: &Self) -> Option<f32> {
        let dir1 = self.direction;
        let dir2 = other.direction;
        let origin_diff = other.origin - self.origin;

        let a = dir1.dot(dir1);
        let b = dir1.dot(dir2);
        let c = dir2.dot(dir2);
        let d = dir1.dot(origin_diff);
        let e = dir2.dot(origin_diff);

        let dd = a * c - b * b;
        (dd.abs() > NEO_LINE_RAY_3D_EPS).then(|| {
            let s = (b * e - c * d) / dd;
            // make the factors point in the right direction, tbh I don't know why they were inverted
            // here but negating them lead to the same results as before
            -s
        })
    }

    fn solve_intersection_equations_dim_zero(&self, other: &Self, d: ZeroDim) -> Option<f32> {
        let p1 = self.origin;
        let p2 = other.origin;
        let r1 = self.direction;
        let r2 = other.direction;

        let (get0, [get1, get2]) = (d.make_getter(), d.others().map(|d| d.make_getter()));

        let is_valid = (get0(p1) - get0(p2)).abs() < NEO_LINE_RAY_3D_EPS;

        is_valid
            .then(|| {
                Mat2::from_cols(
                    Vec2::new(get1(r1), get2(r1)),
                    -Vec2::new(get1(r2), get2(r2)),
                )
            })
            // check if we can calculate an inverse of the matrix (aka if it is solveable)
            .filter(|a| a.determinant().abs() >= NEO_LINE_RAY_3D_EPS)
            // inverse is valid since determinant was valid
            .map(|a| a.inverse())
            .map(|inv_A| {
                let b = Vec2::new(get1(p2) - get1(p1), get2(p2) - get2(p1));
                inv_A.mul_vec2(b)
            })
            .map(|x| x.x)
    }
}

#[test]
fn parallel_works() {
    let l1 = NeoLineRay3D::X;
    let l2 = l1.offset_origin_by(Vec3::Y);
    assert_eq!(l1.intersection_ray(&l2), NeoRayRayIntersection::Parallel);
}

#[test]
fn collinear_no_overlap_works() {
    let l1 = NeoLineRay3D::X;
    let l2 = l1.offset_origin_by(Vec3::X);
    assert_eq!(l1.intersection_ray(&l2), NeoRayRayIntersection::Collinear);
}

#[test]
fn intersection_works() {
    let l1 = NeoLineRay3D::Y;
    let l2 = NeoLineRay3D::X.offset_origin_by(Vec3::X);
    assert_eq!(
        l1.intersection_ray(&l2),
        NeoRayRayIntersection::Intersection(Vec3::ZERO)
    );
}

#[test]
fn intersection_swapped_args_works() {
    let l1 = NeoLineRay3D::Y;
    let l2 = NeoLineRay3D::X.offset_origin_by(Vec3::X);
    assert_eq!(
        l2.intersection_ray(&l1),
        NeoRayRayIntersection::Intersection(Vec3::ZERO)
    );
}

#[test]
fn name() {
    let origin = Vec3::new(4129.3123, 119239.412, -4123132.2);
    let v = origin.cross(Vec3::X).normalize();
    let (d1, d2) = v.any_orthonormal_pair();
    let l1 = NeoLineRay3D::new(origin + d1 * 1234.581, d1);
    let l2 = NeoLineRay3D::new(origin + d2 * 12314.31234, d2);

    let intersection = l1.intersection_ray(&l2);
    assert_eq!(intersection, NeoRayRayIntersection::Intersection(origin))
}
