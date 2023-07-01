use glam::Quat;

use crate::d3::def::Ray3D;

impl Ray3D {
    fn angle_between(&self, other: &Self) -> f32 {
        self.direction().angle_between(other.direction())
    }

    pub fn aligning_rotation_between(&self, other: &Self) -> Quat {
        self.orthogonal_ray(other)
            .map(|normal| Quat::from_axis_angle(normal.direction, self.angle_between(other)))
            .unwrap_or(Quat::IDENTITY)
    }

    pub fn angle_xaxis(&self) -> Quat {
        self.aligning_rotation_between(&Ray3D::X)
    }

    pub fn angle_yaxis(&self) -> Quat {
        self.aligning_rotation_between(&Ray3D::Y)
    }

    pub fn angle_zaxis(&self) -> Quat {
        self.aligning_rotation_between(&Ray3D::Z)
    }

    pub fn rotate(&self, quat: Quat) -> Self {
        Self::from((
            self.origin,
            self.origin + quat * (self.direction - self.origin),
        ))
    }
}

#[test]
fn rotation_works() {
    use crate::d3::constants::NEO_LINE_RAY_3D_EPS;
    use glam::Quat;
    use glam::Vec3;

    let l = Ray3D::X;
    let axis = Vec3::Y;
    let angle = 90.0_f32.to_radians();
    let quat = Quat::from_axis_angle(axis, angle);
    let rotated = l.rotate(quat);
    assert!(
        rotated.origin.abs_diff_eq(Vec3::ZERO, NEO_LINE_RAY_3D_EPS),
        "rotated: {rotated:?}"
    );
    assert!(
        rotated.direction.abs_diff_eq(-Vec3::Z, NEO_LINE_RAY_3D_EPS),
        "rotated: {rotated:?}"
    );
}
