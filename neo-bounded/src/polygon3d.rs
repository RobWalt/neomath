use neo_surface::polygon3d::def::NeoPolygon3D;
use neo_utils::float_ord::FloatOrd;

use crate::traits::NeoBounded3D;

impl NeoBounded3D for NeoPolygon3D {
    fn min_x(&self) -> f32 {
        self.iter_all_points()
            .map(|p| p.x)
            .min_by_key(|&x| FloatOrd(x))
            .unwrap_or_default()
    }

    fn min_y(&self) -> f32 {
        self.iter_all_points()
            .map(|p| p.y)
            .min_by_key(|&x| FloatOrd(x))
            .unwrap_or_default()
    }

    fn min_z(&self) -> f32 {
        self.iter_all_points()
            .map(|p| p.y)
            .min_by_key(|&x| FloatOrd(x))
            .unwrap_or_default()
    }

    fn max_x(&self) -> f32 {
        self.iter_all_points()
            .map(|p| p.x)
            .max_by_key(|&x| FloatOrd(x))
            .unwrap_or_default()
    }

    fn max_y(&self) -> f32 {
        self.iter_all_points()
            .map(|p| p.y)
            .max_by_key(|&x| FloatOrd(x))
            .unwrap_or_default()
    }

    fn max_z(&self) -> f32 {
        self.iter_all_points()
            .map(|p| p.y)
            .max_by_key(|&x| FloatOrd(x))
            .unwrap_or_default()
    }
}
