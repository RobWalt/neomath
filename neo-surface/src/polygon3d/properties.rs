use glam::Vec3;
use neo_line_segment::d3::def::LineSegment3D;

use crate::polygon3d::def::NeoPolygon3D;

impl NeoPolygon3D {
    pub fn iter_exterior_points(&self) -> impl Iterator<Item = &Vec3> {
        self.exterior.iter()
    }

    pub fn iter_interior_points(&self) -> impl Iterator<Item = &Vec3> {
        self.interiors.iter().flatten()
    }

    pub fn iter_all_points(&self) -> impl Iterator<Item = &Vec3> {
        self.iter_exterior_points()
            .chain(self.iter_interior_points())
    }

    pub fn iter_exterior_lines(&self) -> impl Iterator<Item = LineSegment3D> + '_ {
        self.exterior
            .iter()
            .zip(self.exterior.iter().cycle().skip(1))
            .map(|(src, dst)| LineSegment3D::new(*src, *dst))
    }

    pub fn iter_interior_lines(&self) -> impl Iterator<Item = LineSegment3D> + '_ {
        self.interiors.iter().flat_map(|int| {
            int.iter()
                .zip(int.iter().cycle().skip(1))
                .map(|(src, dst)| LineSegment3D::new(*src, *dst))
        })
    }

    pub fn iter_all_lines(&self) -> impl Iterator<Item = LineSegment3D> + '_ {
        self.iter_exterior_lines().chain(self.iter_interior_lines())
    }
}
