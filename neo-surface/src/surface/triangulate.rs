use geo::triangulate_spade::SpadeTriangulationConfig;
use geo::TriangulateSpade;

use crate::surface::def::NeoSurface;

impl NeoSurface {
    pub fn triangulate(&self) -> Vec<Self> {
        let triangles = self
            .shape
            .constrained_triangulation(SpadeTriangulationConfig::default())
            .expect("Triangulation of Neosurface succeeds");

        triangles
            .into_iter()
            .map(|triangle| triangle.to_polygon())
            .map(|shape| {
                let coordinate_system = self.coordinate_system;
                let shape_origin = self.shape_origin;
                NeoSurface {
                    coordinate_system,
                    shape_origin,
                    shape,
                }
            })
            .collect::<Vec<_>>()
    }
}
