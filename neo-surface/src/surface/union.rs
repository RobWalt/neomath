use geo::{MapCoords, SpadeBoolops};

use crate::surface::def::NeoSurface;

impl NeoSurface {
    pub fn union(&self, rhs: &Self) -> Option<Vec<Self>> {
        (self.coordinate_system == rhs.coordinate_system)
            .then(|| {
                let diff_2d = self.shape_origin - rhs.shape_origin;
                let rhs_translated = rhs.shape.map_coords(|c| c + diff_2d);
                let union = geo::Polygon::union(&self.shape, &rhs_translated).ok()?;
                let neo_surfaces = union
                    .into_iter()
                    .map(|p| NeoSurface {
                        shape: p,
                        ..self.clone()
                    })
                    .collect::<Vec<_>>();
                Some(neo_surfaces)
            })
            .flatten()
    }
}
