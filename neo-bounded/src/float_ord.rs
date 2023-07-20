pub(crate) fn float_ord_cmp(f1: &f32, f2: &f32) -> std::cmp::Ordering {
    f1.partial_cmp(f2).unwrap_or(std::cmp::Ordering::Less)
}
