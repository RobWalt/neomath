use glam::Vec3;

const PLANE_EPS: f32 = 0.000_1;

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub local_x: Vec3,
    pub local_y: Vec3,
    pub normal: Vec3,
}

impl Plane {
    pub fn from_local_axis(local_x: Vec3, local_y: Vec3) -> Self {
        Self {
            local_x,
            local_y,
            normal: local_x.cross(local_y),
        }
    }

    pub fn from_normal(normal: Vec3) -> Self {
        let local_x = normal.any_orthogonal_vector();
        let local_y = normal.cross(local_x);
        Self {
            local_x,
            local_y,
            normal,
        }
    }
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        self.normal
            .cross(other.normal)
            .abs_diff_eq(Vec3::ZERO, PLANE_EPS)
    }
}

impl Plane {
    pub fn is_point_in_plane(&self, point: Vec3) -> bool {
        self.normal.dot(point).abs() < PLANE_EPS
    }
}

#[test]
fn partial_eq_works() {
    use glam::Quat;

    let ax1 = Vec3::ONE;
    let ax2 = -Vec3::X + Vec3::Y + Vec3::Z;
    let n = ax1.cross(ax2);
    let p1 = Plane::from_local_axis(ax1, ax2);

    let rot = Quat::from_axis_angle(n, 90.0_f32.to_radians());
    let p2 = Plane::from_local_axis(rot * ax1, rot * ax2);

    assert_eq!(p1, p2)
}

#[test]
fn zero_on_plane_works() {
    let ax1 = Vec3::ONE;
    let ax2 = -Vec3::X + Vec3::Y + Vec3::Z;
    let p = Plane::from_local_axis(ax1, ax2);

    assert!(p.is_point_in_plane(Vec3::ZERO));
}

#[test]
fn axis_sum_on_plane_works() {
    let ax1 = Vec3::ONE;
    let ax2 = -Vec3::X + Vec3::Y + Vec3::Z;
    let p = Plane::from_local_axis(ax1, ax2);

    assert!(p.is_point_in_plane(ax1 + ax2));
}

#[test]
fn scaled_axis_sum_on_plane_works() {
    let ax1 = Vec3::ONE;
    let ax2 = -Vec3::X + Vec3::Y + Vec3::Z;
    let p = Plane::from_local_axis(ax1, ax2);

    assert!(p.is_point_in_plane(ax1 / 7.0 + ax2 * 0.33));
}

#[test]
fn axis_diff_on_plane_works() {
    let ax1 = Vec3::ONE;
    let ax2 = -Vec3::X + Vec3::Y + Vec3::Z;
    let p = Plane::from_local_axis(ax1, ax2);

    assert!(p.is_point_in_plane(ax1 - ax2));
}
