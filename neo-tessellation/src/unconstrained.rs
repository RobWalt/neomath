use neo_float::NeoFloat;
use spade::Triangulation;

use crate::helper::{delauny_from_geo, geo_from_delauny};

pub fn unconstrained_delaunay_tessellation_of_points<F: NeoFloat>(
    points: Vec<geo::Coord<F>>,
) -> Vec<geo::Triangle<F>> {
    let mut dt = spade::DelaunayTriangulation::<spade::Point2<F>>::new();
    points.into_iter().map(delauny_from_geo).for_each(|p| {
        _ = dt.insert(p);
    });
    dt.inner_faces()
        .map(|face| {
            geo::Triangle::from(face.vertices().map(|v| v.position()).map(geo_from_delauny))
        })
        .collect::<Vec<_>>()
}
