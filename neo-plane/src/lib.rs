use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

const PLANE_EPS: f32 = 0.000_1;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Plane {
    pub local_x: Vec3,
    pub local_y: Vec3,
    pub normal: Vec3,
}

impl Plane {
    pub fn new(local_x: Vec3, local_y: Vec3, normal: Vec3) -> Self {
        Self {
            local_x: local_x.normalize(),
            local_y: local_y.normalize(),
            normal: normal.normalize(),
        }
    }

    pub fn from_local_axis(local_x: Vec3, local_y: Vec3) -> Self {
        let normal = local_x.cross(local_y).normalize();
        let ortho_to_x = local_x.cross(normal).normalize();
        Self {
            local_x: local_x.normalize(),
            normal,
            local_y: ortho_to_x,
        }
    }

    pub fn from_axis_and_normal(local_x: Vec3, normal: Vec3) -> Self {
        let local_y = local_x.cross(normal).normalize();
        Self {
            local_x: local_x.normalize(),
            local_y,
            normal: normal.normalize(),
        }
    }

    pub fn from_normal(normal: Vec3) -> Self {
        let local_x = normal.any_orthogonal_vector().normalize();
        let local_y = normal.cross(local_x).normalize();
        Self {
            local_x,
            local_y,
            normal: normal.normalize(),
        }
    }

    pub fn flip(self) -> Self {
        Self {
            normal: -self.normal,
            ..self
        }
    }

    pub fn injection_rotation(&self) -> Quat {
        let normal_align = Quat::from_rotation_arc(Vec3::Z, self.normal);
        let rotated_x = normal_align * Vec3::X;
        let local_x_align = Quat::from_rotation_arc(rotated_x, self.local_x);
        local_x_align * normal_align
    }

    pub fn xy_projection_rotation(&self) -> Quat {
        self.injection_rotation().inverse()
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

#[test]
fn injection_rotation_local_x_works() {
    let p = Plane::from_normal(Vec3::ONE);
    let rot = p.injection_rotation();
    assert!(p.local_x.abs_diff_eq(rot * Vec3::X, PLANE_EPS))
}

#[test]
fn injection_rotation_local_y_works() {
    let p = Plane::from_normal(Vec3::ONE);
    let rot = p.injection_rotation();
    assert!(p.local_y.abs_diff_eq(rot * Vec3::Y, PLANE_EPS))
}

#[test]
fn injection_rotation_normal_works() {
    let p = Plane::from_normal(Vec3::ONE);
    let rot = p.injection_rotation();
    assert!(p.normal.abs_diff_eq(rot * Vec3::Z, PLANE_EPS))
}

#[test]
fn injection_tilted_plane() {
    let p = Plane::new(
        Vec3::X + Vec3::Z,
        -Vec3::X * 0.5 + Vec3::Y + Vec3::Z * 0.5,
        -Vec3::X - Vec3::Y + Vec3::Z,
    );

    // sanity checks
    assert_eq!(p.local_x.dot(p.local_y), 0.0);
    assert_eq!(p.local_x.dot(p.normal), 0.0);
    assert_eq!(p.local_y.dot(p.normal), 0.0);

    let rot = p.injection_rotation();
    let rot_x = rot * Vec3::X;
    let rot_y = rot * Vec3::Y;
    let rot_z = rot * Vec3::Z;
    assert!(
        p.normal.abs_diff_eq(rot_z, PLANE_EPS),
        "normal: {rot_z:?} == {:?} ?",
        p.normal
    );
    assert!(
        p.local_x.abs_diff_eq(rot_x, PLANE_EPS),
        "local_x: {rot_x:?} == {:?} ?",
        p.local_x
    );
    assert!(
        p.local_y.abs_diff_eq(rot_y, PLANE_EPS),
        "local_y: {rot_y:?} == {:?} ?",
        p.local_y
    );
}

#[test]
fn projection_rotation_local_x_works() {
    let p = Plane::from_normal(Vec3::ONE);
    let rot = p.xy_projection_rotation();
    assert!(Vec3::X.abs_diff_eq(rot * p.local_x, PLANE_EPS))
}

#[test]
fn projection_rotation_local_y_works() {
    let p = Plane::from_normal(Vec3::ONE);
    let rot = p.xy_projection_rotation();
    assert!(Vec3::Y.abs_diff_eq(rot * p.local_y, PLANE_EPS))
}

#[test]
fn projection_rotation_normal_works() {
    let p = Plane::from_normal(Vec3::ONE);
    let rot = p.xy_projection_rotation();
    assert!(Vec3::Z.abs_diff_eq(rot * p.normal, PLANE_EPS))
}

#[test]
fn projection_tilted_plane() {
    let p = Plane::new(
        Vec3::X + Vec3::Z,
        -Vec3::X * 0.5 + Vec3::Y + Vec3::Z * 0.5,
        -Vec3::X - Vec3::Y + Vec3::Z,
    );
    let rot = p.xy_projection_rotation();
    assert!(Vec3::X.abs_diff_eq(rot * p.local_x, PLANE_EPS));
    assert!(Vec3::Y.abs_diff_eq(rot * p.local_y, PLANE_EPS));
    assert!(Vec3::Z.abs_diff_eq(rot * p.normal, PLANE_EPS));
}
