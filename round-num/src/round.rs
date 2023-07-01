use crate::traits::def::Precision;

pub(crate) fn round<D: Precision>(v: f32) -> f32 {
    (v * D::FACTOR).round() / D::FACTOR
}
