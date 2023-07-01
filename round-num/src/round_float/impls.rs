use crate::round::round;
use crate::round_float::def::RoundFloat;
use crate::traits::def::Precision;

impl<D: Precision> RoundFloat<D> {
    pub fn new(val: f32) -> Self {
        Self {
            val: round::<D>(val),
            _pd: Default::default(),
        }
    }
}
