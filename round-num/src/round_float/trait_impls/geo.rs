use geo::HasKernel;

use crate::round_float::def::RoundFloat;
use crate::traits::def::Precision;

impl<D: Precision> HasKernel for RoundFloat<D> {
    type Ker = <f32 as HasKernel>::Ker;
}
