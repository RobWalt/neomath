use glam::Vec2;
use neo_aabb::d2::def::AABB2D;
use neo_line_segment::d2::def::LineSegment2D;
use neo_ray::d2::def::Ray2D;

use crate::trait_def::NeoIntersectable;

#[derive(Debug, PartialEq)]
pub enum RayAABB2DIntersection {
    None,
    Point(Vec2),
    Line(LineSegment2D),
}

impl NeoIntersectable<AABB2D> for Ray2D {
    type Output = RayAABB2DIntersection;

    fn intersection(&self, rhs: &AABB2D) -> Self::Output {
        let intersection_points = rhs
            .lines()
            .map(LineSegment2D::from)
            .into_iter()
            .filter_map(|l| {
                l.ray()
                    .intersection(self)
                    .intersection_point()
                    .filter(|&p| l.is_point_on_line(p))
            })
            .fold(vec![], |mut res, elem| {
                if !res.iter().any(|p: &Vec2| p.abs_diff_eq(elem, 0.000_1)) {
                    res.push(elem);
                }
                res
            });
        match intersection_points.len() {
            0 => RayAABB2DIntersection::None,
            1 => RayAABB2DIntersection::Point(intersection_points[0]),
            2 => RayAABB2DIntersection::Line(LineSegment2D::new(
                intersection_points[0],
                intersection_points[1],
            )),
            _ => unreachable!("too many intersection points"),
        }
    }
}

#[cfg(test)]
mod ray_aabb {
    use glam::Vec2;
    use neo_aabb::d2::def::AABB2D;
    use neo_line_segment::d2::def::LineSegment2D;
    use neo_ray::d2::def::Ray2D;

    use crate::ray2d::aabb::RayAABB2DIntersection;
    use crate::trait_def::NeoIntersectable;

    #[test]
    fn no_intersection_works() {
        let aabb = AABB2D::new(Vec2::ZERO, Vec2::ONE);
        let ray = Ray2D::new(-Vec2::X * 2.0, Vec2::ONE);

        assert_eq!(ray.intersection(&aabb), RayAABB2DIntersection::None)
    }

    #[test]
    fn intersection_point_works() {
        let aabb = AABB2D::new(Vec2::ZERO, Vec2::ONE);
        let ray = Ray2D::new(-Vec2::X, Vec2::ONE);

        assert_eq!(
            ray.intersection(&aabb),
            RayAABB2DIntersection::Point(Vec2::Y)
        )
    }

    #[test]
    fn another_intersection_point_works() {
        let aabb = AABB2D::new(Vec2::ZERO, Vec2::ONE);
        let ray = Ray2D::new(Vec2::Y - Vec2::X, -Vec2::Y + Vec2::X);

        assert_eq!(
            ray.intersection(&aabb),
            RayAABB2DIntersection::Point(Vec2::ZERO)
        )
    }

    #[test]
    fn overshooting_ray_intersects_at_point() {
        let aabb = AABB2D::new(Vec2::ZERO, Vec2::ONE);
        let ray = Ray2D::new(Vec2::Y - Vec2::X, (-Vec2::Y + Vec2::X) * 2.0);

        assert_eq!(
            ray.intersection(&aabb),
            RayAABB2DIntersection::Point(Vec2::ZERO)
        )
    }

    #[test]
    fn intersection_line_works() {
        let aabb = AABB2D::new(Vec2::ZERO, Vec2::ONE);
        let ray = Ray2D::new(-Vec2::X * 0.5, Vec2::ONE);

        assert_eq!(
            ray.intersection(&aabb),
            RayAABB2DIntersection::Line(LineSegment2D::new(Vec2::Y * 0.5, Vec2::Y + Vec2::X * 0.5))
        )
    }

    #[test]
    fn many_intersection_points_work() {
        let aabb = AABB2D::new(Vec2::ZERO, Vec2::ONE);
        let ray = Ray2D::new(Vec2::ZERO, Vec2::ONE);

        assert_eq!(
            ray.intersection(&aabb),
            RayAABB2DIntersection::Line(LineSegment2D::new(Vec2::ZERO, Vec2::ONE))
        )
    }

    #[test]
    fn many_intersection_points_eps_offset_work() {
        let aabb = AABB2D::new(Vec2::ZERO, Vec2::ONE);
        let ray = Ray2D::new(Vec2::X * f32::EPSILON, Vec2::ONE);

        assert_eq!(
            ray.intersection(&aabb),
            RayAABB2DIntersection::Line(LineSegment2D::new(
                Vec2::ZERO - f32::EPSILON * Vec2::Y,
                Vec2::ONE + f32::EPSILON * Vec2::X
            ))
        )
    }
}
