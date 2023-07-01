use neo_float::NeoFloat;

use crate::def::geo_spade::{DelaunyLineString, DelaunyPolygon};
use crate::def::vertex_pub::DelaunayVertex;
use crate::with_attributes::delaunay_tessellation_with_attributes;

pub fn delaunay_tessellation<F: NeoFloat>(polygon: geo::Polygon<F>) -> Vec<geo::Triangle<F>> {
    let exterior = DelaunyLineString(
        polygon
            .exterior()
            .into_iter()
            .map(|v| DelaunayVertex {
                point: v.clone(),
                extra_data: (),
            })
            .collect::<Vec<_>>(),
    );
    let interiors = polygon
        .interiors()
        .into_iter()
        .map(|hole| {
            DelaunyLineString(
                hole.into_iter()
                    .map(|v| DelaunayVertex {
                        point: v.clone(),
                        extra_data: (),
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    let delauny_poly = DelaunyPolygon {
        exterior,
        interiors,
    };
    let tris = delaunay_tessellation_with_attributes(delauny_poly);
    tris.into_iter()
        .map(|tri| geo::Triangle::from(tri.map(|v| v.point)))
        .collect::<Vec<_>>()
}
