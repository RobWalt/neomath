use neo_float::NeoFloat;
use spade::{InsertionError, Triangulation};

use crate::def::vertex_private::DelaunayVertexPrivate;

pub(crate) fn add_custom_vertex_poly<T: Clone, F: NeoFloat>(
    cdt: &mut spade::ConstrainedDelaunayTriangulation<DelaunayVertexPrivate<T, F>>,
    polygon: &[DelaunayVertexPrivate<T, F>],
) -> Result<(), InsertionError> {
    polygon
        .iter()
        .zip(polygon.iter().cycle().skip(1))
        .try_for_each(|(a, b)| {
            let v1 = cdt.insert(a.clone())?;
            let v2 = cdt.insert(b.clone())?;
            if cdt.can_add_constraint(v1, v2) {
                cdt.add_constraint(v1, v2);
                Ok(())
            } else {
                // not optimal
                Err(InsertionError::NAN)
            }
        })
}
