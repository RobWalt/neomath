use neo_float::NeoFloat;
use spade::HasPosition;

use crate::def::vertex_pub::DelaunayVertex;
use crate::helper::geo_from_delauny;

#[derive(Clone)]
pub(crate) struct DelaunayVertexPrivate<T: Clone, F: NeoFloat> {
    pub(crate) point: spade::Point2<F>,
    pub(crate) extra_data: T,
}

impl<T: Clone, F: NeoFloat> HasPosition for DelaunayVertexPrivate<T, F> {
    type Scalar = F;
    fn position(&self) -> spade::Point2<Self::Scalar> {
        self.point
    }
}

impl<T: Clone, F: NeoFloat> DelaunayVertexPrivate<T, F> {
    pub(crate) fn to_interface_type(self) -> DelaunayVertex<T, F> {
        DelaunayVertex {
            point: geo_from_delauny(self.point),
            extra_data: self.extra_data,
        }
    }
}
