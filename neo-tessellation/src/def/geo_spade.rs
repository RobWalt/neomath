use neo_float::NeoFloat;

use crate::def::vertex_pub::DelaunayVertex;

pub struct DelaunyLineString<T: Clone, F: NeoFloat>(pub Vec<DelaunayVertex<T, F>>);

impl<T: Clone, F: NeoFloat> DelaunyLineString<T, F> {
    pub fn into_iter(self) -> impl Iterator<Item = DelaunayVertex<T, F>> {
        self.0.into_iter()
    }
}

pub struct DelaunyPolygon<T: Clone, F: NeoFloat> {
    pub exterior: DelaunyLineString<T, F>,
    pub interiors: Vec<DelaunyLineString<T, F>>,
}
