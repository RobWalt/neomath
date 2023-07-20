use neo_coordinate_system::CoordinateSystem;
use neo_ray::d3::def::Ray3D;

use crate::results::RayCoordSys3DIntersection;
use crate::trait_def::NeoIntersectable;

impl NeoIntersectable<Ray3D> for CoordinateSystem {
    type Output = RayCoordSys3DIntersection;

    fn intersection(&self, rhs: &Ray3D) -> Self::Output {
        rhs.intersection(self)
    }
}
