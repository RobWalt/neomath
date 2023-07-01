use geo::{Centroid, Contains};
use neo_float::NeoFloat;
use spade::{HasPosition, Triangulation};

use crate::add_custom_vertex::add_custom_vertex_poly;
use crate::def::geo_spade::DelaunyPolygon;
use crate::def::vertex_private::DelaunayVertexPrivate;
use crate::def::vertex_pub::DelaunayVertex;
use crate::helper::geo_from_delauny;

pub fn delaunay_tessellation_with_attributes<T: Clone, F: NeoFloat>(
    delauny_poly: DelaunyPolygon<T, F>,
) -> Vec<[DelaunayVertex<T, F>; 3]> {
    let DelaunyPolygon {
        exterior,
        interiors,
    } = delauny_poly;
    let exterior = exterior
        .into_iter()
        .map(DelaunayVertex::to_private_type)
        .collect::<Vec<_>>();
    let interiors = interiors
        .into_iter()
        .map(|hole| {
            hole.into_iter()
                .map(DelaunayVertex::to_private_type)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut cdt = spade::ConstrainedDelaunayTriangulation::<DelaunayVertexPrivate<T, F>>::new();
    let _res = add_custom_vertex_poly(&mut cdt, &exterior);
    let _res = interiors
        .iter()
        .try_for_each(|hole| add_custom_vertex_poly(&mut cdt, hole));

    let outer = geo::Polygon::new(
        geo::LineString::new(
            exterior
                .into_iter()
                .map(|v| v.position())
                .map(geo_from_delauny)
                .collect::<Vec<_>>(),
        ),
        vec![],
    );

    let holes = interiors
        .into_iter()
        .map(|hole| {
            geo::Polygon::new(
                geo::LineString::new(
                    hole.into_iter()
                        .map(|v| v.position())
                        .map(geo_from_delauny)
                        .collect::<Vec<_>>(),
                ),
                vec![],
            )
        })
        .collect::<Vec<_>>();

    cdt.inner_faces()
        .map(|face| face.vertices())
        .filter(|tri| {
            let tri = geo::Triangle::from(tri.map(|p| p.position()).map(geo_from_delauny));
            let tri_center = tri.centroid();
            outer.contains(&tri_center) && !holes.iter().any(|hole| hole.contains(&tri_center))
        })
        .map(|tri| {
            tri.map(|v| v.data().clone())
                .map(DelaunayVertexPrivate::to_interface_type)
        })
        .collect::<Vec<_>>()
}
