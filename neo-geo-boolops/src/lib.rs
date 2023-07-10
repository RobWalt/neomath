use geo::{BooleanOps, MapCoords, OpType};
use neo_float::NeoFloat;

pub trait NeoGeoBoolops<F: NeoFloat> {
    fn boolop(&self, other: &Self, op: OpType) -> geo::MultiPolygon<F>;

    fn union(&self, other: &Self) -> geo::MultiPolygon<F> {
        self.boolop(other, OpType::Union)
    }
    fn intersection(&self, other: &Self) -> geo::MultiPolygon<F> {
        self.boolop(other, OpType::Intersection)
    }
    fn difference(&self, other: &Self) -> geo::MultiPolygon<F> {
        self.boolop(other, OpType::Difference)
    }
}

impl<F, Boolable, BoolableMapped> NeoGeoBoolops<F> for Boolable
where
    F: NeoFloat,
    BoolableMapped: BooleanOps<Scalar = f64> + MapCoords<f64, F, Output = Boolable>,
    Boolable: BooleanOps<Scalar = F> + MapCoords<F, f64, Output = BoolableMapped>,
{
    fn boolop(&self, other: &Self, op: OpType) -> geo::MultiPolygon<F> {
        self.try_boolean_op(other, op)
            .or_else(|_| {
                let s = self.map_coords(coord_upcast);
                let o = other.map_coords(coord_upcast);
                s.try_boolean_op(&o, op)
                    .map(|res| res.map_coords(coord_downcast))
            })
            .expect("At least we tried Q.Q")
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
