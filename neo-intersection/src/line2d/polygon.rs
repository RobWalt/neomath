use geo::{Contains, Intersects, LinesIter};
use glam::Vec2;
use neo_geo_glam_interop::to_geo::ConvertToGeo;
use neo_line_segment::d2::def::LineSegment2D;

use crate::float_ord_cmp;
use crate::line_intersection_parts::Line2DIntersectionParts;
use crate::trait_def::NeoIntersectable;

#[derive(Debug, PartialEq)]
pub enum LinePolygon2DIntersection {
    None,
    Point(Vec2),
    Line(LineSegment2D),
    Parts(Vec<Line2DIntersectionParts>),
}

impl NeoIntersectable<geo::Polygon<f32>> for LineSegment2D {
    type Output = LinePolygon2DIntersection;

    fn intersection(&self, rhs: &geo::Polygon<f32>) -> Self::Output {
        // calculate all intersection points of the line with the polygon
        let mut points_with_scalars = rhs
            .lines_iter()
            .map(LineSegment2D::from)
            .filter_map(|l| {
                let inter = self.intersection(&l);
                inter.intersection_point()
            })
            .map(|p| (self.scalar_of(p), p))
            .chain(
                [0.0_f32, 1.0]
                    .into_iter()
                    .zip(self.array().into_iter())
                    .filter(|(_, c)| rhs.intersects(&c.to_geo())),
            )
            .fold(vec![], |mut res, elem| {
                if !res
                    .iter()
                    .any(|(scalar, _)| (scalar - elem.0).abs() < 0.0001)
                {
                    res.push(elem);
                }
                res
            });

        // sort the intersection points by scalar of the intersection line
        points_with_scalars.sort_by(|(a, _), (b, _)| float_ord_cmp(a, b).reverse());

        // collect the intersection points and merge intersecting line part points to line segments
        let mut parts = vec![];
        while let Some((_, new_point)) = points_with_scalars.pop() {
            // get last point we added to the parts (if any exists)
            let last_point = parts.last().map(|last_part| match last_part {
                Line2DIntersectionParts::Point(p) => *p,
                Line2DIntersectionParts::Line(l) => l.dst,
            });

            // only consider the last point if the line would be contained in the poly (checking
            // the center is enough, because of sorted points)
            if let Some(last_point) = last_point.filter(|&last_point| {
                let center = (new_point + last_point) * 0.5;
                rhs.contains(&center.to_geo())
            }) {
                // if the last part was a single point, we convert it to a line
                //
                // else if the last part was a line we keep it and continue drawing from it
                if parts
                    .last()
                    .is_some_and(|part| matches!(part, Line2DIntersectionParts::Point(_)))
                {
                    parts.pop();
                }
                parts.push(Line2DIntersectionParts::Line(LineSegment2D::new(
                    last_point, new_point,
                )))
            } else {
                parts.push(Line2DIntersectionParts::Point(new_point))
            }
        }

        match parts.len() {
            0 => LinePolygon2DIntersection::None,
            1 => match parts[0] {
                Line2DIntersectionParts::Point(p) => LinePolygon2DIntersection::Point(p),
                Line2DIntersectionParts::Line(l) => LinePolygon2DIntersection::Line(l),
            },
            _ => LinePolygon2DIntersection::Parts(parts),
        }
    }
}

#[cfg(test)]
mod line_polygon {
    use glam::Vec2;
    use neo_geo_glam_interop::to_geo::ConvertToGeo;
    use neo_line_segment::d2::def::LineSegment2D;

    use crate::line2d::polygon::{Line2DIntersectionParts, LinePolygon2DIntersection};
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
        let line = LineSegment2D::new(Vec2::ZERO, Vec2::ONE * 0.5);
        assert_eq!(
            line.intersection(&rect),
            LinePolygon2DIntersection::Line(LineSegment2D::new(Vec2::ZERO, Vec2::ONE * 0.5))
        )
    }

    #[test]
    fn simple_line_startpoint_in_poly_works() {
        let rect = geo::Rect::new(Vec2::ZERO.to_geo(), Vec2::ONE.to_geo()).to_polygon();
        let line = LineSegment2D::new(Vec2::ZERO, Vec2::ONE * 0.5).offset_line_by(Vec2::ONE * 0.5);
        assert_eq!(
            line.intersection(&rect),
            LinePolygon2DIntersection::Line(LineSegment2D::new(Vec2::ONE * 0.5, Vec2::ONE))
        )
    }

    #[test]
    fn no_intersection_works() {
        let rect = geo::Rect::new(Vec2::ZERO.to_geo(), Vec2::ONE.to_geo()).to_polygon();
        let line = LineSegment2D::new(Vec2::NEG_ONE, Vec2::ONE * 2.0).offset_line_by(Vec2::Y * 2.0);

        assert_eq!(line.intersection(&rect), LinePolygon2DIntersection::None)
    }

    #[test]
    fn simple_line_intersection_works() {
        let rect = geo::Rect::new(Vec2::ZERO.to_geo(), Vec2::ONE.to_geo()).to_polygon();
        let line = LineSegment2D::new(Vec2::NEG_ONE, Vec2::ONE * 2.0);

        assert_eq!(
            line.intersection(&rect),
            LinePolygon2DIntersection::Line(LineSegment2D::new(Vec2::ZERO, Vec2::ONE))
        )
    }

    #[test]
    fn simple_point_intersection_works() {
        let rect = geo::Rect::new(Vec2::ZERO.to_geo(), Vec2::ONE.to_geo()).to_polygon();
        let line = LineSegment2D::new(Vec2::NEG_ONE, Vec2::ONE * 2.0).offset_line_by(Vec2::Y);

        assert_eq!(
            line.intersection(&rect),
            LinePolygon2DIntersection::Point(Vec2::Y)
        )
    }

    #[test]
    fn intersection_parts_work_with_lines() {
        let rect_with_hole = generate_rect_with_hole();
        let line = LineSegment2D::new(Vec2::ZERO, Vec2::X * 3.0).offset_line_by(Vec2::Y);

        assert_eq!(
            line.intersection(&rect_with_hole),
            LinePolygon2DIntersection::Parts(vec![
                Line2DIntersectionParts::Line(LineSegment2D::new(Vec2::Y, Vec2::Y + Vec2::X)),
                Line2DIntersectionParts::Line(
                    LineSegment2D::new(Vec2::Y, Vec2::Y + Vec2::X).offset_line_by(Vec2::X * 2.0)
                ),
            ])
        )
    }

    #[test]
    fn intersection_parts_work_with_lines_and_points() {
        let rect_with_hole = generate_rect_with_hole();
        let line = LineSegment2D::new(Vec2::ZERO, Vec2::X * 2.0).offset_line_by(Vec2::Y);

        assert_eq!(
            line.intersection(&rect_with_hole),
            LinePolygon2DIntersection::Parts(vec![
                Line2DIntersectionParts::Line(LineSegment2D::new(Vec2::Y, Vec2::Y + Vec2::X)),
                Line2DIntersectionParts::Point(Vec2::Y + Vec2::X * 2.0),
            ])
        )
    }

    #[test]
    fn two_connected_intersection_lines() {
        let rect_with_hole = generate_rect_with_hole();
        let line = LineSegment2D::new(Vec2::ZERO, Vec2::ONE * 2.0).offset_line_by(Vec2::Y);

        assert_eq!(
            line.intersection(&rect_with_hole),
            LinePolygon2DIntersection::Parts(vec![
                Line2DIntersectionParts::Line(
                    LineSegment2D::new(Vec2::ZERO, Vec2::ONE).offset_line_by(Vec2::Y)
                ),
                Line2DIntersectionParts::Line(
                    LineSegment2D::new(Vec2::ZERO, Vec2::ONE).offset_line_by(Vec2::Y + Vec2::ONE)
                ),
            ])
        )
    }
}
