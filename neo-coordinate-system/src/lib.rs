use glam::Vec3;
use neo_plane::Plane;

#[derive(Debug, Clone, Copy)]
pub struct CoordinateSystem {
    plane: Plane,
    origin: Vec3,
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
}

impl PartialEq for CoordinateSystem {
    fn eq(&self, other: &Self) -> bool {
        self.plane == other.plane && self.plane.is_point_in_plane(other.origin)
    }
}

impl CoordinateSystem {
    pub fn is_point_in_coordinate_system(&self, point: Vec3) -> bool {
        self.plane.is_point_in_plane(point - self.origin)
    }

    pub fn normal(&self) -> Vec3 {
        self.plane.normal
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }
}

#[test]
fn partial_eq_works() {
    let ax1 = Vec3::ONE;
    let ax2 = -Vec3::X + Vec3::Y + Vec3::Z;

    let c1 = CoordinateSystem::from_origin_and_axis(Vec3::ZERO, ax1, ax2);
    let c2 = CoordinateSystem::from_origin_and_axis(ax1 + ax2, ax1, ax2);

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
