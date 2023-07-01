use neo_float::NeoFloat;
use spade::{InsertionError, Triangulation};

use crate::helper::{delauny_from_geo, geo_from_delauny};

pub fn constrained_delaunay_tessellation_of_lines<F: NeoFloat>(
    lines: impl IntoIterator<Item = geo::Line<F>>,
) -> Vec<geo::Triangle<F>> {
    let mut cdt = spade::ConstrainedDelaunayTriangulation::<spade::Point2<F>>::new();
    let _res: Result<(), InsertionError> = lines.into_iter().try_for_each(|line| {
        let v1 = cdt.insert(delauny_from_geo(line.start_point().into()))?;
        let v2 = cdt.insert(delauny_from_geo(line.end_point().into()))?;
        if cdt.can_add_constraint(v1, v2) {
            cdt.add_constraint(v1, v2);
        } else {
            println!("couldn't insert constraint");
        }
        Ok(())
    });
    cdt.inner_faces()
        .map(|face| {
            geo::Triangle::from(face.vertices().map(|v| v.position()).map(geo_from_delauny))
        })
        .collect::<Vec<_>>()
}
