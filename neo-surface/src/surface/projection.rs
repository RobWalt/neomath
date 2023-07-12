use geo_glam_interop::to_geo::ConvertToGeo;
use geo_glam_interop::to_glam::ConvertToGlam;
use glam::Vec3;

use crate::surface::def::NeoSurface;

impl NeoSurface {
    /// translation that transfers a point, which was rotated to the coordinate system plane to a
    /// position so that it is placed correctly with respect to the origin points.
    ///
    /// That means the relations (distances + positions with respect to local axis) of
    ///   - 2d origin ~ 2d point
    ///   - 3d origin ~ 3d point ( = rotation * 2d point + translation )
    /// are the same (where the rotation comes from the `injection_rotation` from the plane)
    pub fn matching_translation(&self) -> Vec3 {
        let rotation = self.coordinate_system.plane.injection_rotation();
        let shape_origin_3d = rotation * self.shape_origin.to_glam().extend(0.0);
        self.coordinate_system.origin - shape_origin_3d
    }

    /// rotates 3D point, so that it lies in a plane which is parallel to the X-Y plane. If the
    /// point was in the coordinate system of the surface, then the point is located on the X-Y
    /// plane.
    pub fn rotate_point_xy(&self, point: Vec3) -> Vec3 {
        let offset_point = point - self.matching_translation();
        let rotation = self.coordinate_system.plane.xy_projection_rotation();
        rotation * offset_point
    }

    /// rotates 3D point, so that it lies in a plane which is parallel to the X-Y plane and then
    /// projects it to the X-Y plane regardless of the plane it is located in
    pub fn project_point_xy(&self, point: Vec3) -> geo::Coord<f32> {
        self.rotate_point_xy(point).truncate().to_geo()
    }
}

#[cfg(test)]
mod test {
    use glam::Vec3;
    use neo_coordinate_system::CoordinateSystem;
    use neo_plane::Plane;

    use crate::surface::def::NeoSurface;

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
    fn rotate_origin_to_xy_plane() {
        let surface = create_standard_surface();
        let projected_origin = surface.rotate_point_xy(surface.coordinate_system.origin);
        assert_eq!(projected_origin.z, 0.0);
    }

    #[test]
    fn rotate_point_in_coordinate_system_to_xy_plane() {
        let surface = create_standard_surface();
        let projected_point = surface.rotate_point_xy(
            surface.coordinate_system.origin
                + surface.coordinate_system.plane.local_x
                + surface.coordinate_system.plane.local_y,
        );
        assert!((projected_point.z - 0.0).abs() < 0.000_1);
    }
}
