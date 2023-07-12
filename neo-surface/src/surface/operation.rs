use glam::Vec3;

use crate::surface::def::NeoSurface;

impl NeoSurface {
    pub fn translate_by(&self, offset: Vec3) -> Self {
        Self {
            coordinate_system: self.coordinate_system.offset_origin_by(offset),
            shape_origin: self.shape_origin,
            shape: self.shape.clone(),
        }
    }
}

#[cfg(test)]
mod operation_tests {
    use glam::{Vec2, Vec3};
    use neo_line_segment::d2::def::LineSegment2D;

    use crate::surface::def::{NeoSurface, SURFACE_EPS};

    #[test]
    fn translation_works() {
        let line = LineSegment2D::new(Vec2::Y, Vec2::X);
        let z_low = 2.0;
        let z_high = 4.0;

        let surface = NeoSurface::from_line_and_heights_vertical(line, z_low, z_high);

        let injected = surface.as_polygon_3d();

        let translated_surface = surface.translate_by(Vec3::ONE);

        let translated_injected = translated_surface.as_polygon_3d();

        for (normal, translated) in injected
            .iter_all_points()
            .zip(translated_injected.iter_all_points())
        {
            assert!(translated.abs_diff_eq(*normal + Vec3::ONE, SURFACE_EPS));
        }
    }
}
