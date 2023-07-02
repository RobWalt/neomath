#![allow(non_snake_case)]
use glam::{Mat2, Vec2, Vec3};
use neo_ray::d3::constants::NEO_LINE_RAY_3D_EPS;
use neo_ray::d3::def::Ray3D;

use crate::trait_def::NeoIntersectable;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RayRay3DIntersection {
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

impl NeoIntersectable for Ray3D {
    type Output = RayRay3DIntersection;

    fn intersection(&self, rhs: &Self) -> Self::Output {
        if self.is_parallel_to(rhs) {
            classify_parallel_relation_to(self, rhs)
        } else {
            classify_intersecting_relation_to(self, rhs)
        }
    }
}

pub(crate) fn classify_parallel_relation_to(r1: &Ray3D, r2: &Ray3D) -> RayRay3DIntersection {
    if r1.is_point_on_ray(r2.origin) {
        RayRay3DIntersection::Collinear
    } else {
        RayRay3DIntersection::Parallel
    }
}

pub(crate) fn classify_intersecting_relation_to(r1: &Ray3D, r2: &Ray3D) -> RayRay3DIntersection {
    match calculate_intersection_point(r1, r2) {
        Some(intersection_point) => RayRay3DIntersection::Intersection(intersection_point),
        None => RayRay3DIntersection::Skewed,
    }
}

pub(crate) fn calculate_intersection_point(r1: &Ray3D, r2: &Ray3D) -> Option<Vec3> {
    let dir1 = r1.direction;
    let dir2 = r2.direction;

    let dirs: [[f32; 2]; 3] = [[dir1.x, dir2.x], [dir1.y, dir2.y], [dir1.z, dir2.z]];
    let near_zero = |x: f32| x.abs() < NEO_LINE_RAY_3D_EPS;

    let maybe_scalar = match dirs {
        // in these cases it is more like a 2D intersection
        [[a, b], _, _] if near_zero(a) && near_zero(b) => {
            solve_intersection_equations_dim_zero(r1, r2, ZeroDim::X)
        }
        [_, [a, b], _] if near_zero(a) && near_zero(b) => {
            solve_intersection_equations_dim_zero(r1, r2, ZeroDim::Y)
        }
        [_, _, [a, b]] if near_zero(a) && near_zero(b) => {
            solve_intersection_equations_dim_zero(r1, r2, ZeroDim::Z)
        }
        _ => solve_overdetermined_intersection_system(r1, r2),
    };

    maybe_scalar.map(|s| r1.origin + s * r1.direction)
}

// from https://stackoverflow.com/a/34604574
pub(crate) fn solve_overdetermined_intersection_system(r1: &Ray3D, r2: &Ray3D) -> Option<f32> {
    let dir1 = r1.direction;
    let dir2 = r2.direction;
    let origin_diff = r2.origin - r1.origin;

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

fn solve_intersection_equations_dim_zero(r1: &Ray3D, r2: &Ray3D, d: ZeroDim) -> Option<f32> {
    let p1 = r1.origin;
    let p2 = r2.origin;
    let dir1 = r1.direction;
    let dir2 = r2.direction;

    let (get0, [get1, get2]) = (d.make_getter(), d.others().map(|d| d.make_getter()));

    let is_valid = (get0(p1) - get0(p2)).abs() < NEO_LINE_RAY_3D_EPS;

    is_valid
        .then(|| {
            Mat2::from_cols(
                Vec2::new(get1(dir1), get2(dir1)),
                -Vec2::new(get1(dir2), get2(dir2)),
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

#[test]
fn parallel_works() {
    let l1 = Ray3D::X;
    let l2 = l1.offset_origin_by(Vec3::Y);
    assert_eq!(l1.intersection(&l2), RayRay3DIntersection::Parallel);
}

#[test]
fn collinear_no_overlap_works() {
    let l1 = Ray3D::X;
    let l2 = l1.offset_origin_by(Vec3::X);
    assert_eq!(l1.intersection(&l2), RayRay3DIntersection::Collinear);
}

#[test]
fn intersection_works() {
    let l1 = Ray3D::Y;
    let l2 = Ray3D::X.offset_origin_by(Vec3::X);
    assert_eq!(
        l1.intersection(&l2),
        RayRay3DIntersection::Intersection(Vec3::ZERO)
    );
}

#[test]
fn intersection_swapped_args_works() {
    let l1 = Ray3D::Y;
    let l2 = Ray3D::X.offset_origin_by(Vec3::X);
    assert_eq!(
        l2.intersection(&l1),
        RayRay3DIntersection::Intersection(Vec3::ZERO)
    );
}

#[test]
fn name() {
    let origin = Vec3::new(4129.3123, 119239.412, -4123132.2);
    let v = origin.cross(Vec3::X).normalize();
    let (d1, d2) = v.any_orthonormal_pair();
    let l1 = Ray3D::new(origin + d1 * 1234.581, d1);
    let l2 = Ray3D::new(origin + d2 * 12314.31234, d2);

    let intersection = l1.intersection(&l2);
    assert_eq!(intersection, RayRay3DIntersection::Intersection(origin))
}
