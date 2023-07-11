use geo::Centroid;
use neo_coordinate_system::CoordinateSystem;

use crate::def::NeoSurface;

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
}
