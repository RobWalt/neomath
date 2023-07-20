use geo::Intersects;
use glam::Vec2;
use neo_bounded::traits::NeoBounded2D;
use neo_geo_glam_interop::to_geo::ConvertToGeo;
use neo_line_segment::d2::def::LineSegment2D;
use neo_ray::d2::def::Ray2D;

use crate::line2d::polygon::{LinePolygon2DIntersection, LinePolygon2DIntersectionPart};
use crate::ray2d::aabb::RayAABB2DIntersection;
use crate::trait_def::NeoIntersectable;

#[derive(Debug, PartialEq)]
pub enum RayPolygon2DIntersection {
    None,
    Point(Vec2),
    Line(LineSegment2D),
    Parts(Vec<RayPolygon2DIntersectionPart>),
}

impl RayPolygon2DIntersection {
    pub fn list_parts(self) -> Vec<RayPolygon2DIntersectionPart> {
        match self {
            RayPolygon2DIntersection::None => vec![],
            RayPolygon2DIntersection::Point(p) => vec![RayPolygon2DIntersectionPart::Point(p)],
            RayPolygon2DIntersection::Line(l) => vec![RayPolygon2DIntersectionPart::Line(l)],
            RayPolygon2DIntersection::Parts(ps) => ps,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RayPolygon2DIntersectionPart {
    Point(Vec2),
    Line(LineSegment2D),
}

impl NeoIntersectable<geo::Polygon<f32>> for Ray2D {
    type Output = RayPolygon2DIntersection;

    fn intersection(&self, rhs: &geo::Polygon<f32>) -> Self::Output {
        let rhs_aabb = rhs.aabb();
        let aabb_inter = self.intersection(&rhs_aabb);
        match aabb_inter {
            RayAABB2DIntersection::None => RayPolygon2DIntersection::None,
            RayAABB2DIntersection::Point(p) => aabb_point_case_analysis(p, rhs),
            RayAABB2DIntersection::Line(aabb_line) => aabb_line_case_analysis(aabb_line, rhs),
        }
    }
}

fn aabb_point_case_analysis(point: Vec2, rhs: &geo::Polygon<f32>) -> RayPolygon2DIntersection {
    if rhs.intersects(&point.to_geo()) {
        RayPolygon2DIntersection::Point(point)
    } else {
        RayPolygon2DIntersection::None
    }
}

fn aabb_line_case_analysis(
    aabb_line: LineSegment2D,
    rhs: &geo::Polygon<f32>,
) -> RayPolygon2DIntersection {
    match aabb_line.intersection(rhs) {
        LinePolygon2DIntersection::None => RayPolygon2DIntersection::None,
        LinePolygon2DIntersection::Point(p) => RayPolygon2DIntersection::Point(p),
        LinePolygon2DIntersection::Line(l) => RayPolygon2DIntersection::Line(l),
        LinePolygon2DIntersection::Parts(ps) => RayPolygon2DIntersection::Parts(convert_parts(ps)),
    }
}

fn convert_parts(parts: Vec<LinePolygon2DIntersectionPart>) -> Vec<RayPolygon2DIntersectionPart> {
    parts
        .into_iter()
        .map(|part| match part {
            LinePolygon2DIntersectionPart::Point(p) => RayPolygon2DIntersectionPart::Point(p),
            LinePolygon2DIntersectionPart::Line(l) => RayPolygon2DIntersectionPart::Line(l),
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod ray_polygon {
    use glam::Vec2;
    use neo_geo_glam_interop::to_geo::ConvertToGeo;
    use neo_line_segment::d2::def::LineSegment2D;
    use neo_ray::d2::def::Ray2D;

    use crate::ray2d::polygon::{RayPolygon2DIntersection, RayPolygon2DIntersectionPart};
    use crate::trait_def::NeoIntersectable;

    fn generate_rect_with_hole() -> geo::Polygon<f32> {
        let ext = geo::Rect::new(Vec2::ZERO.to_geo(), (Vec2::ONE * 3.0).to_geo())
            .to_polygon()
            .exterior()
            .clone();
        let int = geo::Rect::new(Vec2::ONE.to_geo(), (Vec2::ONE * 2.0).to_geo())
            .to_polygon()
            .exterior()
            .clone();
        geo::Polygon::new(ext, vec![int])
    }

    #[test]
    fn simple_line_endpoint_in_poly_works() {
        let rect = geo::Rect::new(Vec2::ZERO.to_geo(), Vec2::ONE.to_geo()).to_polygon();
        let ray = Ray2D::new(Vec2::ZERO, Vec2::ONE * 0.5);
        assert_eq!(
            ray.intersection(&rect),
            RayPolygon2DIntersection::Line(LineSegment2D::new(Vec2::ZERO, Vec2::ONE))
        )
    }

    #[test]
    fn simple_line_startpoint_in_poly_works() {
        let rect = geo::Rect::new(Vec2::ZERO.to_geo(), Vec2::ONE.to_geo()).to_polygon();
        let ray = Ray2D::new(Vec2::ZERO, Vec2::ONE * 0.5).offset_origin_by(Vec2::ONE * 0.5);
        assert_eq!(
            ray.intersection(&rect),
            RayPolygon2DIntersection::Line(LineSegment2D::new(Vec2::ZERO, Vec2::ONE))
        )
    }

    #[test]
    fn no_intersection_works() {
        let rect = geo::Rect::new(Vec2::ZERO.to_geo(), Vec2::ONE.to_geo()).to_polygon();
        let ray = Ray2D::new(Vec2::NEG_ONE, Vec2::ONE * 2.0).offset_origin_by(Vec2::Y * 2.0);

        assert_eq!(ray.intersection(&rect), RayPolygon2DIntersection::None)
    }

    #[test]
    fn simple_line_intersection_works() {
        let rect = geo::Rect::new(Vec2::ZERO.to_geo(), Vec2::ONE.to_geo()).to_polygon();
        let ray = Ray2D::new(Vec2::NEG_ONE, Vec2::ONE * 2.0);

        assert_eq!(
            ray.intersection(&rect),
            RayPolygon2DIntersection::Line(LineSegment2D::new(Vec2::ZERO, Vec2::ONE))
        )
    }

    #[test]
    fn simple_point_intersection_works() {
        let rect = geo::Rect::new(Vec2::ZERO.to_geo(), Vec2::ONE.to_geo()).to_polygon();
        let ray = Ray2D::new(Vec2::NEG_ONE, Vec2::ONE * 2.0).offset_origin_by(Vec2::Y);

        assert_eq!(
            ray.intersection(&rect),
            RayPolygon2DIntersection::Point(Vec2::Y)
        )
    }

    #[test]
    fn intersection_parts_work_with_lines() {
        let rect_with_hole = generate_rect_with_hole();
        let ray = Ray2D::new(Vec2::ZERO, Vec2::X).offset_origin_by(Vec2::Y);

        assert_eq!(
            ray.intersection(&rect_with_hole),
            RayPolygon2DIntersection::Parts(vec![
                RayPolygon2DIntersectionPart::Line(LineSegment2D::new(Vec2::Y, Vec2::Y + Vec2::X)),
                RayPolygon2DIntersectionPart::Line(
                    LineSegment2D::new(Vec2::Y, Vec2::Y + Vec2::X).offset_line_by(Vec2::X * 2.0)
                ),
            ])
        )
    }

    #[test]
    fn intersection_parts_work_with_lines_and_points() {
        let rect_with_hole = generate_rect_with_hole();
        let ray = Ray2D::new(Vec2::ZERO, Vec2::X * 2.0).offset_origin_by(Vec2::Y);

        assert_eq!(
            ray.intersection(&rect_with_hole),
            RayPolygon2DIntersection::Parts(vec![
                RayPolygon2DIntersectionPart::Line(LineSegment2D::new(Vec2::Y, Vec2::Y + Vec2::X)),
                RayPolygon2DIntersectionPart::Line(
                    LineSegment2D::new(Vec2::Y, Vec2::Y + Vec2::X).offset_line_by(Vec2::X * 2.0)
                ),
            ])
        )
    }
}
