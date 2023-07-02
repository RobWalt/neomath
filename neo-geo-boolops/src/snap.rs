use geo::{BoundingRect, Contains, CoordsIter, EuclideanDistance, Scale};
use neo_float::NeoFloat;

#[derive(Clone, Copy, Debug)]
enum TrianglePoints {
    A,
    B,
    C,
}

impl TrianglePoints {
    pub fn get<F: NeoFloat>(self, t: &geo::Triangle<F>) -> geo::Coord<F> {
        match self {
            TrianglePoints::A => t.0,
            TrianglePoints::B => t.1,
            TrianglePoints::C => t.2,
        }
    }
    pub fn get_mut<F: NeoFloat>(self, t: &mut geo::Triangle<F>) -> &mut geo::Coord<F> {
        match self {
            TrianglePoints::A => &mut t.0,
            TrianglePoints::B => &mut t.1,
            TrianglePoints::C => &mut t.2,
        }
    }
    pub fn get_others<F: NeoFloat>(self, t: &geo::Triangle<F>) -> [geo::Coord<F>; 2] {
        match self {
            TrianglePoints::A => [t.1, t.2],
            TrianglePoints::B => [t.0, t.2],
            TrianglePoints::C => [t.0, t.1],
        }
    }
}

pub fn snap_points(
    t_in: &mut geo::Triangle<f64>,
    other: &geo::MultiPolygon<f64>,
    snap_distance: f64,
) {
    use TrianglePoints::*;
    let aabb = t_in.bounding_rect().scale(1.1);
    let coords_in_bound = other
        .iter()
        .flat_map(|p| p.coords_iter())
        .filter(|o| aabb.contains(o))
        .collect::<Vec<_>>();
    [A, B, C].into_iter().for_each(|p| {
        let c = p.get(t_in);
        let others = p.get_others(t_in);
        if let Some(new_c) = coords_in_bound
            .iter()
            .filter(|&o| c.euclidean_distance(o) < snap_distance)
            .find(|&o| !others.contains(o))
            .cloned()
        {
            *p.get_mut(t_in) = new_c;
        }
    });
}
