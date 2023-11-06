use glam::Vec3;
use neo_coordinate_system::CoordinateSystem;
use neo_plane::Plane;
use neo_surface::surface::def::NeoSurface;

use crate::line_intersection_parts::Line3DIntersectionParts;
use crate::surface::coord_sys::SurfaceCoordSys3DIntersection;
use crate::trait_def::NeoIntersectable;

#[derive(Debug, Clone, PartialEq)]
pub enum SurfacePlane3DIntersection {
    None,
    Point(Vec3),
    Parts(Vec<Line3DIntersectionParts>),
    Surface(NeoSurface),
}

impl NeoIntersectable<Plane> for NeoSurface {
    type Output = SurfacePlane3DIntersection;

    fn intersection(&self, rhs: &Plane) -> Self::Output {
        let coord_sys_plane = CoordinateSystem::from_origin_and_plane(Vec3::ZERO, *rhs);
        let inter = self.intersection(&coord_sys_plane);
        match inter {
            SurfaceCoordSys3DIntersection::None => SurfacePlane3DIntersection::None,
            SurfaceCoordSys3DIntersection::Point(p) => SurfacePlane3DIntersection::Point(p),
            SurfaceCoordSys3DIntersection::Parts(ps) => SurfacePlane3DIntersection::Parts(ps),
            SurfaceCoordSys3DIntersection::Surface(s) => SurfacePlane3DIntersection::Surface(s),
        }
    }
}
