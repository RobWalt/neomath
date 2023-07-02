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
    use geo::Intersects;
    use geo_svg::ToSvg;
    use std::f32::consts::TAU;
    use std::io::Write;

    use rand::{thread_rng, Rng};

    use crate::ops::{difference, intersect};

    fn save_svg(g: &impl ToSvg, path: &str, color: &'static str) {
        let svg = g.to_svg();
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(format!("{path}.html"))
            .unwrap();
        _ = f.write(
            svg.with_opacity(0.3)
                .with_color(geo_svg::Color::Named(color))
                .to_string()
                .as_bytes(),
        );
    }

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
        let mut int = vec![];
        (1..=rng.gen_range(3..=6)).for_each(|i| {
            while int.len() != i {
                let mid = geo::Coord {
                    x: rng.gen_range(-50.0..=50.0),
                    y: rng.gen_range(-50.0..=50.0),
                };
                let c = geo::Polygon::new(make_circle(20, mid, 20.0), vec![]);
                if int.iter().all(|o: &geo::Polygon<f32>| !o.intersects(&c)) {
                    int.push(c);
                }
            }
        });
        let int = int
            .into_iter()
            .map(|i| i.exterior().clone())
            .collect::<Vec<_>>();
        geo::MultiPolygon::new(vec![geo::Polygon::<f32>::new(ext, int)])
    }

    #[test]
    fn random_intersect_test() {
        _ = std::fs::remove_file("inter.html");
        let first = make_random_poly();
        save_svg(&first, "inter", "blue");
        let second = make_random_poly();
        save_svg(&second, "inter", "red");
        let x = intersect(&first, &second);
        save_svg(&x, "inter", "black");
    }

    #[test]
    fn random_difference_test() {
        _ = std::fs::remove_file("diff.html");
        let first = make_random_poly();
        save_svg(&first, "diff", "blue");
        let second = make_random_poly();
        save_svg(&second, "diff", "red");
        let x = difference(&first, &second);
        save_svg(&x, "diff", "black");
    }
}
