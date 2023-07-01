use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use crate::round_float::def::RoundFloat;
use crate::traits::def::Precision;

impl<D: Precision> Add for RoundFloat<D> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        RoundFloat::new(self.val + rhs.val)
    }
}

impl<D: Precision> Sub for RoundFloat<D> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        RoundFloat::new(self.val - rhs.val)
    }
}

impl<D: Precision> Mul for RoundFloat<D> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        RoundFloat::new(self.val * rhs.val)
    }
}

impl<D: Precision> Div for RoundFloat<D> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        RoundFloat::new(self.val / rhs.val)
    }
}

impl<D: Precision> Rem for RoundFloat<D> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        RoundFloat::new(self.val % rhs.val)
    }
}

impl<D: Precision> Neg for RoundFloat<D> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        RoundFloat::new(-self.val)
    }
}
