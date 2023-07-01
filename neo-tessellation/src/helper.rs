use geo::Area;
use neo_float::NeoFloat;

pub(crate) fn subdivide_triangles<F: NeoFloat>(
    triangles: Vec<geo::Triangle<F>>,
    max_area: F,
) -> Vec<geo::Triangle<F>> {
    let mut final_polys = vec![];
    let mut in_process_polys = triangles;
    while !in_process_polys.is_empty() {
        let (new_final, new_in_process) = in_process_polys
            .iter()
            .partition::<Vec<geo::Triangle<F>>, _>(|tri| tri.unsigned_area() < max_area);
        in_process_polys = new_in_process
            .into_iter()
            .flat_map(|t| {
                let a = t.0;
                let b = t.1;
                let c = t.2;
                let half = F::from_raw_f64(0.5);
                let ab = (a + b) * half;
                let bc = (b + c) * half;
                let ca = (c + a) * half;
                [
                    geo::Triangle::new(a, ab, ca),
                    geo::Triangle::new(b, ab, bc),
                    geo::Triangle::new(c, bc, ca),
                    geo::Triangle::new(ab, bc, ca),
                ]
            })
            .collect();
        final_polys.extend(new_final);
    }
    final_polys
}

pub(crate) fn delauny_from_geo<F: NeoFloat>(geo_point: geo::Coord<F>) -> spade::Point2<F> {
    spade::Point2::new(geo_point.x, geo_point.y)
}

pub(crate) fn geo_from_delauny<F: NeoFloat>(spade_point: spade::Point2<F>) -> geo::Coord<F> {
    geo::Coord {
        x: spade_point.x,
        y: spade_point.y,
    }
}
