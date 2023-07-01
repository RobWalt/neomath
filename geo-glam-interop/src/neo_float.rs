use neo_float::NeoFloat;

pub trait NeoFloatConversions<F: NeoFloat> {
    type Target;
    fn from_f64_version(a: Self::Target) -> Self;
    fn to_f64_version(self) -> Self::Target;
}

impl<F: NeoFloat> NeoFloatConversions<F> for geo::Coord<F> {
    type Target = geo::Coord<f64>;
    fn from_f64_version(a: Self::Target) -> Self {
        Self {
            x: F::from_raw_f64(a.x),
            y: F::from_raw_f64(a.y),
        }
    }
    fn to_f64_version(self) -> Self::Target {
        geo::Coord::<f64> {
            x: self.x.to_raw_f64(),
            y: self.y.to_raw_f64(),
        }
    }
}
