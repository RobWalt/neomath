use geo::{Centroid, Contains};
use neo_float::NeoFloat;

pub fn collect_difference_tri_mesh<F: NeoFloat>(
    tri_mesh: Vec<geo::Triangle<F>>,
    p1: &geo::MultiPolygon<F>,
    p2: &geo::MultiPolygon<F>,
) -> Vec<geo::Triangle<F>> {
    tri_mesh
        .into_iter()
        .filter(|tri| {
            let center = tri.centroid();
            p1.contains(&center) && !p2.contains(&center)
        })
        .collect::<Vec<_>>()
}
