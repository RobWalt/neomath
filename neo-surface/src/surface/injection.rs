use neo_geo_glam_interop::to_glam::ConvertToGlam;

use crate::polygon3d::def::NeoPolygon3D;
use crate::surface::def::NeoSurface;

impl NeoSurface {
    pub fn as_polygon_3d(&self) -> NeoPolygon3D {
        let rotation = self.coordinate_system.plane.injection_rotation();
        let translation = self.matching_translation();

        let transform_linestring = move |ls: &geo::LineString<f32>| {
            ls.points()
                .map(|p| geo::Coord::<f32>::from(p))
                .map(|c| rotation * c.to_glam().extend(0.0) + translation)
                .collect::<Vec<_>>()
        };

        let exterior = transform_linestring(self.shape.exterior());
        let interiors = self
            .shape
            .interiors()
            .iter()
            .map(transform_linestring)
            .collect::<Vec<_>>();

        NeoPolygon3D {
            normal: self.coordinate_system.plane.normal,
            exterior,
            interiors,
        }
    }
}

#[cfg(test)]
mod test {
    use glam::Vec3;
    use neo_coordinate_system::CoordinateSystem;
    use neo_plane::Plane;

    use crate::surface::def::{NeoSurface, SURFACE_EPS};

    fn create_standard_surface() -> NeoSurface {
        let local_x = Vec3::X + Vec3::Z;
        let local_y = -Vec3::X * 0.5 + Vec3::Y + Vec3::Z * 0.5;

        let p = Plane::from_local_axis(local_x, local_y);
        let c = CoordinateSystem::from_origin_and_plane(Vec3::ONE, p);
        let o = geo::Coord::<f32>::zero();
        let shape = geo::Triangle::<f32>::new(
            geo::Coord::zero(),
            geo::Coord { x: 1.0, y: 0.0 },
            geo::Coord { x: 0.0, y: 1.0 },
        )
        .to_polygon();

        NeoSurface::new(c, o, shape)
    }

    #[test]
    fn shape_injection_works() {
        let surface = create_standard_surface();

        let surface_3d_points = surface.as_polygon_3d();

        assert!(!surface_3d_points.exterior.is_empty());
        assert!(surface_3d_points.interiors.is_empty());

        let local_x = Vec3::X + Vec3::Z;
        let local_y = -Vec3::X * 0.5 + Vec3::Y + Vec3::Z * 0.5;
        let expected_3d_points = vec![
            Vec3::ONE,
            Vec3::ONE + local_x.normalize(),
            Vec3::ONE + local_y.normalize(),
        ];

        for (i, (calculated, expected)) in surface_3d_points
            .exterior
            .iter()
            .zip(expected_3d_points)
            .enumerate()
        {
            assert!(
                calculated.abs_diff_eq(expected, SURFACE_EPS),
                "{i}th points: {calculated:?} == {expected:?} ?"
            );
        }
    }
}
