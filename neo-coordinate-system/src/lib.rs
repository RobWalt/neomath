use glam::Vec3;
use neo_plane::Plane;
use serde::{Deserialize, Serialize};

pub const COORDINATE_SYSTEM_EPS: f32 = 0.000_1;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CoordinateSystem {
    pub plane: Plane,
    pub origin: Vec3,
}

impl CoordinateSystem {
    pub fn from_origin_and_axis(origin: Vec3, local_x: Vec3, local_y: Vec3) -> Self {
        Self {
            plane: Plane::from_local_axis(local_x, local_y),
            origin,
        }
    }

    pub fn from_origin_and_plane(origin: Vec3, plane: Plane) -> Self {
        Self { plane, origin }
    }

    pub fn from_origin_and_normal(origin: Vec3, normal: Vec3) -> Self {
        Self {
            plane: Plane::from_normal(normal),
            origin,
        }
    }

    pub fn flip(self) -> Self {
        Self {
            plane: self.plane.flip(),
            ..self
        }
    }
}

impl PartialEq for CoordinateSystem {
    fn eq(&self, other: &Self) -> bool {
        self.plane == other.plane && self.is_point_in_coordinate_system(other.origin)
    }
}

impl CoordinateSystem {
    pub fn offset_origin_by(&self, offset: Vec3) -> Self {
        Self {
            origin: self.origin + offset,
            plane: self.plane,
        }
    }
}

impl CoordinateSystem {
    pub fn is_point_in_coordinate_system(&self, point: Vec3) -> bool {
        self.plane.is_point_in_plane(point - self.origin)
    }
}

#[cfg(test)]
mod coord_sys {
    use glam::Vec3;

    use crate::CoordinateSystem;

    #[test]
    fn partial_eq_works() {
        let ax1 = Vec3::ONE;
        let ax2 = -Vec3::X + Vec3::Y + Vec3::Z;

        let c1 = CoordinateSystem::from_origin_and_axis(Vec3::ZERO, ax1, ax2);
        let c2 = CoordinateSystem::from_origin_and_axis(ax1 + ax2, ax1, ax2);

        assert_eq!(c1, c2);
    }

    #[test]
    fn partial_eq_non_zero_origin_fails_correctly() {
        let ax1 = Vec3::ONE;
        let ax2 = -Vec3::X + Vec3::Y + Vec3::Z;

        let c1 = CoordinateSystem::from_origin_and_axis(Vec3::X + Vec3::Y, ax1, ax2);
        let c2 = CoordinateSystem::from_origin_and_axis(Vec3::ONE * 10.0, ax1, ax2);

        assert_ne!(c1, c2);
    }

    #[test]
    fn partial_eq_non_zero_origin_works() {
        let ax1 = Vec3::ONE;
        let ax2 = -Vec3::X + Vec3::Y + Vec3::Z;

        let c1 = CoordinateSystem::from_origin_and_axis(Vec3::X, ax1, ax2);
        let c2 = CoordinateSystem::from_origin_and_axis(ax1 + ax2 + Vec3::X, ax1, ax2);

        assert_eq!(c1, c2);
    }

    #[test]
    fn partial_eq_linear_comb_works() {
        let ax1 = Vec3::ONE;
        let ax2 = -Vec3::X + Vec3::Y + Vec3::Z;

        let c1 = CoordinateSystem::from_origin_and_axis(Vec3::ZERO, ax1, ax2);
        let c2 = CoordinateSystem::from_origin_and_axis(ax1 / 7.33 + ax2 * 0.333, ax1, ax2);

        assert_eq!(c1, c2);
    }

    #[test]
    fn origin_on_plane_works() {
        let ax1 = Vec3::ONE;
        let ax2 = -Vec3::X + Vec3::Y + Vec3::Z;
        let origin = Vec3::Y * 0.25;
        let c = CoordinateSystem::from_origin_and_axis(origin, ax1, ax2);

        assert!(c.is_point_in_coordinate_system(origin));
    }

    #[test]
    fn axis_sum_on_plane_works() {
        let ax1 = Vec3::ONE;
        let ax2 = -Vec3::X + Vec3::Y + Vec3::Z;
        let origin = Vec3::Y * 0.25;
        let c = CoordinateSystem::from_origin_and_axis(origin, ax1, ax2);

        assert!(c.is_point_in_coordinate_system(origin + ax1 + ax2));
    }

    #[test]
    fn scaled_axis_sum_on_plane_works() {
        let ax1 = Vec3::ONE;
        let ax2 = -Vec3::X + Vec3::Y + Vec3::Z;
        let origin = Vec3::Y * 0.25;
        let c = CoordinateSystem::from_origin_and_axis(origin, ax1, ax2);

        assert!(c.is_point_in_coordinate_system(origin + ax1 / 7.0 + ax2 * 0.333));
    }

    #[test]
    fn axis_diff_on_plane_works() {
        let ax1 = Vec3::ONE;
        let ax2 = -Vec3::X + Vec3::Y + Vec3::Z;
        let origin = Vec3::Y * 0.25;
        let c = CoordinateSystem::from_origin_and_axis(origin, ax1, ax2);

        assert!(c.is_point_in_coordinate_system(origin - ax1 - ax2));
    }
}
