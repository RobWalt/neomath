use geo::{BooleanOps, MapCoords, Scale};
use round_num::round_float::def::RoundFloat;
use round_num::traits::def::Precision;

pub trait RoundBoolop<D: Precision> {
    fn union(&self, other: &Self) -> geo::MultiPolygon<RoundFloat<D>>;
    fn difference(&self, other: &Self) -> geo::MultiPolygon<RoundFloat<D>>;
    fn intersection(&self, other: &Self) -> geo::MultiPolygon<RoundFloat<D>>;
}

impl<D> RoundBoolop<D> for geo::Polygon<RoundFloat<D>>
where
    D: Precision,
{
    fn union(&self, other: &Self) -> geo::MultiPolygon<RoundFloat<D>> {
        let _self = self.map_coords(coord_mapper_to_raw);
        let other = other.map_coords(coord_mapper_to_raw);
        _self.union(&other).map_coords(coord_mapper_from_raw)
    }

    fn difference(&self, other: &Self) -> geo::MultiPolygon<RoundFloat<D>> {
        let _self = self.map_coords(coord_mapper_to_raw).scale(0.9999);
        let other = other.map_coords(coord_mapper_to_raw).scale(0.9999);
        _self.difference(&other).map_coords(coord_mapper_from_raw)
    }

    fn intersection(&self, other: &Self) -> geo::MultiPolygon<RoundFloat<D>> {
        let _self = self.map_coords(coord_mapper_to_raw);
        let other = other.map_coords(coord_mapper_to_raw);
        _self.intersection(&other).map_coords(coord_mapper_from_raw)
    }
}

impl<D> RoundBoolop<D> for geo::MultiPolygon<RoundFloat<D>>
where
    D: Precision,
{
    fn union(&self, other: &Self) -> geo::MultiPolygon<RoundFloat<D>> {
        let _self = self.map_coords(coord_mapper_to_raw);
        let other = other.map_coords(coord_mapper_to_raw);
        _self.union(&other).map_coords(coord_mapper_from_raw)
    }

    fn difference(&self, other: &Self) -> geo::MultiPolygon<RoundFloat<D>> {
        let _self = self.map_coords(coord_mapper_to_raw);
        let other = other.map_coords(coord_mapper_to_raw);
        _self.difference(&other).map_coords(coord_mapper_from_raw)
    }

    fn intersection(&self, other: &Self) -> geo::MultiPolygon<RoundFloat<D>> {
        let _self = self.map_coords(coord_mapper_to_raw);
        let other = other.map_coords(coord_mapper_to_raw);
        _self.intersection(&other).map_coords(coord_mapper_from_raw)
    }
}

pub fn coord_mapper_to_raw<D: Precision>(c: geo::Coord<RoundFloat<D>>) -> geo::Coord<f32> {
    let (x, y) = c.x_y();
    geo::Coord {
        x: x.to_raw(),
        y: y.to_raw(),
    }
}

pub fn coord_mapper_from_raw<D: Precision>(c: geo::Coord<f32>) -> geo::Coord<RoundFloat<D>> {
    let (x, y) = c.x_y();
    geo::Coord {
        x: RoundFloat::<D>::from_raw(x),
        y: RoundFloat::<D>::from_raw(y),
    }
}
