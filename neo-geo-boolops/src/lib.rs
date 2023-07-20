use geo::{BooleanOps, MapCoords, OpType};
use neo_float::NeoFloat;

pub trait NeoGeoBoolops<F: NeoFloat> {
    fn neo_boolop(&self, other: &Self, op: OpType) -> Option<geo::MultiPolygon<F>>;

    fn neo_union(&self, other: &Self) -> Option<geo::MultiPolygon<F>> {
        self.neo_boolop(other, OpType::Union)
    }
    fn neo_intersection(&self, other: &Self) -> Option<geo::MultiPolygon<F>> {
        self.neo_boolop(other, OpType::Intersection)
    }
    fn neo_difference(&self, other: &Self) -> Option<geo::MultiPolygon<F>> {
        self.neo_boolop(other, OpType::Difference)
    }
}

impl<F, Boolable, BoolableMapped> NeoGeoBoolops<F> for Boolable
where
    F: NeoFloat,
    BoolableMapped: BooleanOps<Scalar = f64> + MapCoords<f64, F, Output = Boolable>,
    Boolable: BooleanOps<Scalar = F> + MapCoords<F, f64, Output = BoolableMapped>,
{
    fn neo_boolop(&self, other: &Self, op: OpType) -> Option<geo::MultiPolygon<F>> {
        self.try_boolean_op(other, op)
            .or_else(|_| {
                let s = self.map_coords(coord_upcast);
                let o = other.map_coords(coord_upcast);
                s.try_boolean_op(&o, op)
                    .map(|res| res.map_coords(coord_downcast))
            })
            .ok()
    }
}

fn coord_upcast<F: NeoFloat>(c: geo::Coord<F>) -> geo::Coord<f64> {
    geo::Coord {
        x: c.x.to_f64().unwrap(),
        y: c.y.to_f64().unwrap(),
    }
}

fn coord_downcast<F: NeoFloat>(c: geo::Coord<f64>) -> geo::Coord<F> {
    geo::Coord {
        x: F::from_raw_f64(c.x),
        y: F::from_raw_f64(c.y),
    }
}

#[test]
fn f32_version_compiles() {
    type T = f32;
    let poly = geo::Polygon::<T>::new(geo::LineString::<T>::new(vec![]), vec![]);
    _ = poly.try_union(&poly);

    let multi_poly = geo::MultiPolygon::<T>::new(vec![]);
    _ = multi_poly.try_union(&multi_poly);
}

#[test]
fn f64_version_compiles() {
    type T = f64;
    let poly = geo::Polygon::<T>::new(geo::LineString::<T>::new(vec![]), vec![]);
    _ = poly.try_union(&poly);

    let multi_poly = geo::MultiPolygon::<T>::new(vec![]);
    _ = multi_poly.try_union(&multi_poly);
}
