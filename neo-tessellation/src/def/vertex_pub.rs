use neo_float::NeoFloat;

use crate::def::vertex_private::DelaunayVertexPrivate;
use crate::helper::delauny_from_geo;

#[derive(Clone)]
pub struct DelaunayVertex<T: Clone, F: NeoFloat> {
    pub point: geo::Coord<F>,
    pub extra_data: T,
}

impl<T: Clone, F: NeoFloat> DelaunayVertex<T, F> {
    pub(crate) fn to_private_type(self) -> DelaunayVertexPrivate<T, F> {
        DelaunayVertexPrivate {
            point: delauny_from_geo(self.point),
            extra_data: self.extra_data,
        }
    }
}
