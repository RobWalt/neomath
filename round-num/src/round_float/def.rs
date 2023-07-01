use std::fmt::Debug;
use std::marker::PhantomData;

use crate::traits::def::Precision;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct RoundFloat<D: Precision> {
    pub(crate) val: f32,
    pub(crate) _pd: PhantomData<D>,
}

impl<D: Precision> RoundFloat<D> {
    pub fn to_raw(&self) -> f32 {
        self.val
    }
    pub fn from_raw(raw: f32) -> Self {
        Self {
            val: raw,
            _pd: Default::default(),
        }
    }
}

impl<D: Precision> Debug for RoundFloat<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}
