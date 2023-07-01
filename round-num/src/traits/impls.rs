use crate::marker::{Deci0, Deci1, Deci2, Deci3, Deci4, Deci5, Deci6};
use crate::traits::def::Precision;

impl Precision for Deci0 {
    const FACTOR: f32 = 1.0;
}

impl Precision for Deci1 {
    const FACTOR: f32 = 10.0;
}

impl Precision for Deci2 {
    const FACTOR: f32 = 100.0;
}

impl Precision for Deci3 {
    const FACTOR: f32 = 1_000.0;
}

impl Precision for Deci4 {
    const FACTOR: f32 = 10_000.0;
}

impl Precision for Deci5 {
    const FACTOR: f32 = 100_000.0;
}

impl Precision for Deci6 {
    const FACTOR: f32 = 1_000_000.0;
}
