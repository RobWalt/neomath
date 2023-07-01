use crate::round_float::def::RoundFloat;
use crate::traits::def::Precision;
use float_next_after::NextAfter;

impl<D: Precision> NextAfter for RoundFloat<D> {
    fn next_after(self, y: Self) -> Self {
        Self::new(self.val.next_after(y.val))
    }
}
