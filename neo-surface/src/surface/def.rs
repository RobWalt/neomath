use neo_coordinate_system::CoordinateSystem;

pub const SURFACE_EPS: f32 = 0.000_1;

#[derive(Debug, Clone, PartialEq)]
pub struct NeoSurface {
    /// Coordinate system the 3D surface lives in
    pub coordinate_system: CoordinateSystem,
    /// "matching point" between the 2D and 3D. This point is injected onto the coordinate systems
    /// origin point
    pub shape_origin: geo::Coord<f32>,
    /// The shape of the surface in a 2D context
    pub shape: geo::Polygon<f32>,
}
