use num_traits::{Bounded, Float, Num, NumCast, One, Signed, ToPrimitive, Zero};

use crate::round_float::def::RoundFloat;
use crate::traits::def::Precision;

impl<D: Precision> Bounded for RoundFloat<D> {
    fn min_value() -> Self {
        Self::new(<f32 as Bounded>::min_value())
    }

    fn max_value() -> Self {
        Self::new(<f32 as Bounded>::max_value())
    }
}

impl<D: Precision> Signed for RoundFloat<D> {
    fn abs(&self) -> Self {
        Self::new(self.val.abs())
    }

    fn abs_sub(&self, other: &Self) -> Self {
        Self::new((self.val - other.val).abs())
    }

    fn signum(&self) -> Self {
        Self::new(self.val.signum())
    }

    fn is_positive(&self) -> bool {
        self.val.is_positive()
    }

    fn is_negative(&self) -> bool {
        self.val.is_negative()
    }
}

impl<D: Precision> ToPrimitive for RoundFloat<D> {
    fn to_i64(&self) -> Option<i64> {
        self.val.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.val.to_u64()
    }
}

impl<D: Precision> Num for RoundFloat<D> {
    type FromStrRadixErr = <f32 as Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(Self::new(f32::from_str_radix(str, radix)?))
    }
}

impl<D: Precision> One for RoundFloat<D> {
    fn one() -> Self {
        Self::new(f32::one())
    }
}

impl<D: Precision> Zero for RoundFloat<D> {
    fn zero() -> Self {
        Self::new(f32::zero())
    }

    fn is_zero(&self) -> bool {
        self.val.is_zero()
    }
}

impl<D: Precision> NumCast for RoundFloat<D> {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        Some(Self::new(<f32 as NumCast>::from(n)?))
    }
}

impl<D: Precision> Float for RoundFloat<D> {
    fn nan() -> Self {
        Self::new(f32::nan())
    }

    fn infinity() -> Self {
        Self::new(f32::infinity())
    }

    fn neg_infinity() -> Self {
        Self::new(f32::neg_infinity())
    }

    fn neg_zero() -> Self {
        Self::new(f32::neg_zero())
    }

    fn min_value() -> Self {
        Self::new(<f32 as Float>::min_value())
    }

    fn min_positive_value() -> Self {
        Self::new(f32::min_positive_value())
    }

    fn max_value() -> Self {
        Self::new(<f32 as Float>::max_value())
    }

    fn is_nan(self) -> bool {
        self.val.is_nan()
    }

    fn is_infinite(self) -> bool {
        self.val.is_infinite()
    }

    fn is_finite(self) -> bool {
        self.val.is_finite()
    }

    fn is_normal(self) -> bool {
        self.val.is_normal()
    }

    fn classify(self) -> std::num::FpCategory {
        self.val.classify()
    }

    fn floor(self) -> Self {
        Self::new(self.val.floor())
    }

    fn ceil(self) -> Self {
        Self::new(self.val.ceil())
    }

    fn round(self) -> Self {
        Self::new(self.val.round())
    }

    fn trunc(self) -> Self {
        Self::new(self.val.trunc())
    }

    fn fract(self) -> Self {
        Self::new(self.val.fract())
    }

    fn abs(self) -> Self {
        Self::new(self.val.abs())
    }

    fn signum(self) -> Self {
        Self::new(self.val.signum())
    }

    fn is_sign_positive(self) -> bool {
        self.val.is_sign_positive()
    }

    fn is_sign_negative(self) -> bool {
        self.val.is_sign_negative()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        Self::new(self.val.mul_add(a.val, b.val))
    }

    fn recip(self) -> Self {
        Self::new(self.val.recip())
    }

    fn powi(self, n: i32) -> Self {
        Self::new(self.val.powi(n))
    }

    fn powf(self, n: Self) -> Self {
        Self::new(self.val.powf(n.val))
    }

    fn sqrt(self) -> Self {
        Self::new(self.val.sqrt())
    }

    fn exp(self) -> Self {
        Self::new(self.val.exp())
    }

    fn exp2(self) -> Self {
        Self::new(self.val.exp2())
    }

    fn ln(self) -> Self {
        Self::new(self.val.ln())
    }

    fn log(self, base: Self) -> Self {
        Self::new(self.val.log(base.val))
    }

    fn log2(self) -> Self {
        Self::new(self.val.log2())
    }

    fn log10(self) -> Self {
        Self::new(self.val.log10())
    }

    fn max(self, other: Self) -> Self {
        Self::new(self.val.max(other.val))
    }

    fn min(self, other: Self) -> Self {
        Self::new(self.val.min(other.val))
    }

    fn abs_sub(self, other: Self) -> Self {
        Self::new((self.val - other.val).abs())
    }

    fn cbrt(self) -> Self {
        Self::new(self.val.cbrt())
    }

    fn hypot(self, other: Self) -> Self {
        Self::new(self.val.hypot(other.val))
    }

    fn sin(self) -> Self {
        Self::new(self.val.sin())
    }

    fn cos(self) -> Self {
        Self::new(self.val.cos())
    }

    fn tan(self) -> Self {
        Self::new(self.val.tan())
    }

    fn asin(self) -> Self {
        Self::new(self.val.asin())
    }

    fn acos(self) -> Self {
        Self::new(self.val.acos())
    }

    fn atan(self) -> Self {
        Self::new(self.val.atan())
    }

    fn atan2(self, other: Self) -> Self {
        Self::new(self.val.atan2(other.val))
    }

    fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = self.val.sin_cos();
        (Self::new(sin), Self::new(cos))
    }

    fn exp_m1(self) -> Self {
        Self::new(self.val.exp_m1())
    }

    fn ln_1p(self) -> Self {
        Self::new(self.val.ln_1p())
    }

    fn sinh(self) -> Self {
        Self::new(self.val.sinh())
    }

    fn cosh(self) -> Self {
        Self::new(self.val.cosh())
    }

    fn tanh(self) -> Self {
        Self::new(self.val.tanh())
    }

    fn asinh(self) -> Self {
        Self::new(self.val.asinh())
    }

    fn acosh(self) -> Self {
        Self::new(self.val.acosh())
    }

    fn atanh(self) -> Self {
        Self::new(self.val.atanh())
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        self.val.integer_decode()
    }
}
