use geo::Centroid;
use geo_glam_interop::to_geo::ConvertToGeo;
use geo_glam_interop::to_glam::ConvertToGlam;
use glam::Vec3;
use neo_coordinate_system::CoordinateSystem;
use neo_line_segment::d2::def::LineSegment2D;

use crate::polygon3d::def::NeoPolygon3D;
use crate::surface::def::NeoSurface;

impl NeoSurface {
    pub fn new(
        coordinate_system: CoordinateSystem,
        shape_origin: geo::Coord<f32>,
        shape: geo::Polygon<f32>,
    ) -> Self {
        Self {
            coordinate_system,
            shape_origin,
            shape,
        }
    }

    pub fn new_origin_at_shape_center(
        coordinate_system: CoordinateSystem,
        shape: geo::Polygon<f32>,
    ) -> Self {
        let center = shape.centroid().expect("polygon has a center");
        Self {
            coordinate_system,
            shape_origin: center.into(),
            shape,
        }
    }

    /// inverts the normal of the coordinate system
    pub fn invert_facing_direction(self) -> Self {
        Self {
            coordinate_system: self.coordinate_system.flip(),
            ..self
        }
    }

    /// inverts the winding of the shape (exterior and all interiors)
    pub fn invert_winding(self) -> Self {
        let invert_linestring_winding = |ls: &geo::LineString<f32>| -> geo::LineString<f32> {
            geo::LineString::new(ls.0.clone().into_iter().rev().collect::<Vec<_>>())
        };
        let ext = invert_linestring_winding(self.shape.exterior());
        let ints = self
            .shape
            .interiors()
            .iter()
            .map(|ls| invert_linestring_winding(ls))
            .collect::<Vec<_>>();
        Self {
            shape: geo::Polygon::new(ext, ints),
            ..self
        }
    }

    /// inverts the normal of the coordinate system and the winding of the shape
    pub fn flip(self) -> Self {
        self.invert_facing_direction().invert_winding()
    }

    pub fn empty_with_normal(normal: Vec3) -> Self {
        Self {
            coordinate_system: CoordinateSystem::from_origin_and_normal(Vec3::ZERO, normal),
            shape_origin: geo::Coord::zero(),
            shape: geo::Polygon::new(geo::LineString::new(vec![]), vec![]),
        }
    }

    pub fn from_polygon_3d(poly3d: NeoPolygon3D) -> Self {
        let NeoPolygon3D {
            normal,
            exterior,
            interiors,
        } = poly3d;
        if exterior.is_empty() {
            return Self::empty_with_normal(normal);
        }
        let coordinate_system = CoordinateSystem::from_origin_and_normal(exterior[0], normal);
        let rot = coordinate_system.plane.xy_projection_rotation();
        let ext = exterior
            .into_iter()
            .map(|p| rot * p)
            .map(|p| p.truncate())
            .collect::<Vec<_>>()
            .to_geo();
        let shape_origin = ext[0];
        let ints = interiors
            .into_iter()
            .map(|int| {
                int.into_iter()
                    .map(|p| rot * p)
                    .map(|p| p.truncate())
                    .collect::<Vec<_>>()
                    .to_geo()
            })
            .collect::<Vec<_>>();
        let shape = geo::Polygon::new(ext, ints);
        Self::new(coordinate_system, shape_origin, shape)
    }

    pub fn from_line_and_heights_vertical(line: LineSegment2D, z_low: f32, z_high: f32) -> Self {
        let normal = line.normal().extend(0.0).normalize();
        let outline = [
            line.src.extend(z_low),
            line.dst.extend(z_low),
            line.dst.extend(z_high),
            line.src.extend(z_high),
        ]
        .to_vec();

        let poly3d = NeoPolygon3D::from_outline_and_normal(outline, normal);

        Self::from_polygon_3d(poly3d)
    }

    pub fn from_linestring_and_height_horizontal(
        mut linestring: geo::LineString<f32>,
        height: f32,
    ) -> Self {
        let normal = Vec3::Z;

        // open linestring
        linestring.close();
        linestring.0.pop();

        let outline = linestring
            .into_iter()
            .map(|c| c.to_glam().extend(height))
            .collect::<Vec<_>>();

        let poly3d = NeoPolygon3D::from_outline_and_normal(outline, normal);

        Self::from_polygon_3d(poly3d)
    }
}

#[cfg(test)]
mod constructor_tests {
    use glam::Vec2;
    use neo_line_segment::d2::def::LineSegment2D;

    use crate::surface::def::{NeoSurface, SURFACE_EPS};

    #[test]
    fn line_constructor_works() {
        let line = LineSegment2D::new(Vec2::Y, Vec2::X);
        let z_low = 2.0;
        let z_high = 4.0;

        let surface = NeoSurface::from_line_and_heights_vertical(line, z_low, z_high);

        let injected = surface.as_polygon_3d();

        assert!(!injected.exterior.is_empty());
        assert!(injected.interiors.is_empty());

        let expected_points = [
            Vec2::Y.extend(2.0),
            Vec2::X.extend(2.0),
            Vec2::X.extend(4.0),
            Vec2::Y.extend(4.0),
            Vec2::Y.extend(2.0),
        ]
        .to_vec();

        for (calculated, expected) in injected.exterior.iter().zip(expected_points) {
            assert!(calculated.abs_diff_eq(expected, SURFACE_EPS));
        }
    }

    #[test]
    fn roundtrip_over_polygon3d_works() {
        let line = LineSegment2D::new(Vec2::Y, Vec2::X);
        let z_low = 2.0;
        let z_high = 4.0;

        let surface = NeoSurface::from_line_and_heights_vertical(line, z_low, z_high);

        let injected = surface.as_polygon_3d();

        let reconstructed_surface = NeoSurface::from_polygon_3d(injected.clone());

        let injected_reconstructed = reconstructed_surface.as_polygon_3d();

        for (original, reconstruced) in injected
            .iter_all_points()
            .zip(injected_reconstructed.iter_all_points())
        {
            assert!(original.abs_diff_eq(*reconstruced, SURFACE_EPS));
        }
    }
}
