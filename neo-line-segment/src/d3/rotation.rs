use glam::{Quat, Vec3};

use crate::d3::def::LineSegment3D;

impl LineSegment3D {
    pub fn aligning_rotation_between(&self, other: &Self) -> Quat {
        self.ray().aligning_rotation_between(&other.ray())
    }

    pub fn angle_xaxis(&self) -> Quat {
        self.ray().angle_xaxis()
    }

    pub fn angle_yaxis(&self) -> Quat {
        self.ray().angle_yaxis()
    }

    pub fn angle_zaxis(&self) -> Quat {
        self.ray().angle_zaxis()
    }

    pub fn rotate_around(&self, p: Vec3, quat: Quat) -> Self {
        Self::from(self.array().map(|v| p + quat * (v - p)))
    }
}

#[test]
fn aligning_rotation_works() {
    let l1 = LineSegment3D::UNIT_X.offset_line_by(Vec3::ONE);
    let l2 = LineSegment3D::UNIT_X
        .rotate_around(Vec3::X * 1.0, Quat::from_rotation_x(41.043))
        .offset_line_by(Vec3::ONE);
    assert_eq!(
        l2.rotate_around(l2.src, l2.aligning_rotation_between(&l1))
            .direction(),
        l1.direction()
    )
}

#[test]
fn rotation_around_works() {
    let l = LineSegment3D::UNIT_X.offset_line_by(Vec3::ONE);
    let p = Vec3::ONE;
    let angle = 90.0_f32.to_radians();
    let axis = Vec3::Z;
    let quat = Quat::from_axis_angle(axis, angle);
    assert_eq!(
        l.rotate_around(p, quat),
        LineSegment3D::new(Vec3::ONE, Vec3::ONE + Vec3::Y),
    );
}
