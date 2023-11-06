use std::iter::Sum;

use geo::GeoFloat;

pub trait NeoFloat: GeoFloat + Default + Sum + Into<f64> + From<f32> {
    fn from_raw_f64(a: f64) -> Self;
    fn to_raw_f64(self) -> f64;
}

impl NeoFloat for f32 {
    fn from_raw_f64(a: f64) -> Self {
        a as Self
    }
    fn to_raw_f64(self) -> f64 {
        self as f64
    }
}
impl NeoFloat for f64 {
    fn from_raw_f64(a: f64) -> Self {
        a as Self
    }
    fn to_raw_f64(self) -> f64 {
        self
    }
}
