use geo::{Contains, Scale};
use glam::Vec3;

use crate::def::NeoSurface;

impl NeoSurface {
    pub fn is_point_in_surface(&self, point: Vec3) -> bool {
        let in_surface = self.coordinate_system.is_point_in_coordinate_system(point);
        let projected_point_xy = self.project_point_xy(point);
        // This magic is a bit ugly, but needed since points on the boundary are not correctly
        // classified otherwise
        let shape = geo::Polygon::new(
            // make exterior slightly bigger
            self.shape.scale(1.0 + f32::EPSILON).exterior().clone(),
            // make interiors slightly smaller
            self.shape
                .scale(1.0 - f32::EPSILON)
                .interiors()
                .into_iter()
                .cloned()
                .collect::<Vec<_>>(),
        );
        let in_shape = shape.contains(&projected_point_xy);
        in_surface && in_shape
    }
}

#[cfg(test)]
mod surface_predicates {
    use glam::Vec3;
    use neo_coordinate_system::CoordinateSystem;
    use neo_plane::Plane;

    use crate::def::NeoSurface;

    fn create_standard_surface() -> NeoSurface {
        let local_x = Vec3::X + Vec3::Z;
        let local_y = -Vec3::X * 0.5 + Vec3::Y + Vec3::Z * 0.5;

        let p = Plane::from_local_axis(local_x, local_y);
        let c = CoordinateSystem::from_origin_and_plane(Vec3::ONE, p);
        let o = geo::Coord::<f32>::zero();
        let shape = geo::Rect::<f32>::new(
            geo::Coord { x: -1.0, y: -1.0 },
            geo::Coord { x: 1.0, y: 1.0 },
        )
        .to_polygon();

        NeoSurface::new(c, o, shape)
    }

    #[test]
    fn origin_is_located_in_surface() {
        let surface = create_standard_surface();
        assert!(surface.is_point_in_surface(Vec3::ONE));
    }

    #[test]
    fn origin_is_corner_in_surface() {
        let surface = create_standard_surface();
        let x_axis = surface.coordinate_system.plane.local_x;
        let y_axis = surface.coordinate_system.plane.local_y;
        assert!(surface.is_point_in_surface(Vec3::ONE + x_axis + y_axis));
        assert!(surface.is_point_in_surface(Vec3::ONE - x_axis + y_axis));
        assert!(surface.is_point_in_surface(Vec3::ONE + x_axis - y_axis));
        assert!(surface.is_point_in_surface(Vec3::ONE - x_axis - y_axis));
    }
}
