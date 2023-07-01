use neo_float::NeoFloat;
use neo_tessellation::with_constraints::constrained_delaunay_tessellation_of_lines;

use crate::mesh_ops::difference_mesh::collect_difference_tri_mesh;
use crate::mesh_ops::intersect_mesh::collect_intersection_tri_mesh;
use crate::mesh_ops::union_mesh::collect_union_tri_mesh;
use crate::shared_poly_lines::shared_lines;
use crate::unify_mesh::unify_tri_mesh;

pub fn intersect<F: NeoFloat>(
    p1: &geo::MultiPolygon<F>,
    p2: &geo::MultiPolygon<F>,
) -> geo::MultiPolygon<F> {
    let lines = shared_lines(p1.clone(), p2.clone());
    let cdt_tris = constrained_delaunay_tessellation_of_lines(lines);
    let intersection_mesh = collect_intersection_tri_mesh(cdt_tris, &p1, &p2);
    unify_tri_mesh(intersection_mesh)
}

pub fn difference<F: NeoFloat>(
    p1: &geo::MultiPolygon<F>,
    p2: &geo::MultiPolygon<F>,
) -> geo::MultiPolygon<F> {
    let lines = shared_lines(p1.clone(), p2.clone());
    let cdt_tris = constrained_delaunay_tessellation_of_lines(lines);
    let difference_mesh = collect_difference_tri_mesh(cdt_tris, &p1, &p2);
    unify_tri_mesh(difference_mesh)
}

pub fn union<F: NeoFloat>(
    p1: &geo::MultiPolygon<F>,
    p2: &geo::MultiPolygon<F>,
) -> geo::MultiPolygon<F> {
    let lines = shared_lines(p1.clone(), p2.clone());
    let cdt_tris = constrained_delaunay_tessellation_of_lines(lines);
    let union_mesh = collect_union_tri_mesh(cdt_tris, p1, p2);
    unify_tri_mesh(union_mesh)
}

#[cfg(test)]
mod stress_tests {
    use std::f32::consts::TAU;

    use rand::{thread_rng, Rng};

    use crate::ops::{difference, intersect};

    fn make_circle(n: usize, mid: geo::Coord<f32>, radius: f32) -> geo::LineString<f32> {
        geo::LineString::<f32>::new(
            (0..n)
                .map(|i| {
                    geo::Coord {
                        x: (i as f32 / n as f32 * TAU).sin() * radius,
                        y: (i as f32 / n as f32 * TAU).cos() * radius,
                    } + mid
                })
                .collect::<Vec<_>>(),
        )
    }

    fn make_random_poly() -> geo::MultiPolygon<f32> {
        let mut rng = thread_rng();
        let ext = make_circle(100, geo::Coord::zero(), 100.0);
        let int = (1..=rng.gen_range(3..=6))
            .map(|_| {
                let mid = geo::Coord {
                    x: rng.gen_range(-50.0..=50.0),
                    y: rng.gen_range(-50.0..=50.0),
                };
                make_circle(20, mid, 20.0)
            })
            .collect::<Vec<_>>();
        geo::MultiPolygon::new(vec![geo::Polygon::<f32>::new(ext, int)])
    }

    #[test]
    fn random_intersect_test() {
        let first = make_random_poly();
        let second = make_random_poly();
        let _x = intersect(&first, &second);
    }

    #[test]
    fn random_difference_test() {
        let first = make_random_poly();
        let second = make_random_poly();
        let _x = difference(&first, &second);
    }
}
