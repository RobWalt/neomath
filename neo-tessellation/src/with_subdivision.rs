use neo_float::NeoFloat;

use crate::helper::subdivide_triangles;
use crate::simple::delaunay_tessellation;

pub fn delaunay_tessellation_with_subdivision<F: NeoFloat>(
    polygon: geo::Polygon<F>,
    max_area: F,
) -> Vec<geo::Triangle<F>> {
    subdivide_triangles(delaunay_tessellation(polygon), max_area)
}
