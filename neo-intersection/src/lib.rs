pub(crate) mod coord_sys;
pub(crate) mod line2d;
pub(crate) mod line3d;
pub mod line_intersection_parts;
pub(crate) mod plane;
pub(crate) mod ray2d;
pub(crate) mod ray3d;
pub mod results;
pub(crate) mod surface;
pub mod trait_def;

pub(crate) fn float_ord_cmp(f1: &f32, f2: &f32) -> std::cmp::Ordering {
    f1.partial_cmp(f2).unwrap_or(std::cmp::Ordering::Less)
}
