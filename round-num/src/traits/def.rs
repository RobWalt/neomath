pub trait Precision: PartialOrd + Copy + std::fmt::Debug {
    const FACTOR: f32;
}
