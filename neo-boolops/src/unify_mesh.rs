use geo::{BooleanOps, MapCoords};
use geo_glam_interop::neo_float::NeoFloatConversions;
use geo_glam_interop::to_glam::ConvertToGlam;
use neo_float::NeoFloat;
use neo_tessellation::simple::delaunay_tessellation;

use crate::snap::snap_points;

const MINIMUM_ANGLE: f32 = 1.0;
const SNAP_TOLERANCE: f64 = 0.1;

pub fn unify_tri_mesh<F: NeoFloat>(tri_mesh: Vec<geo::Triangle<F>>) -> geo::MultiPolygon<F> {
    let mut res = tri_mesh
        .into_iter()
        .map(|t| geo::Triangle::<f64>::from(t.to_array().map(|c| c.to_f64_version())))
        .fold(geo::MultiPolygon::new(vec![]), |s, mut t| {
            snap_points(&mut t, &s, SNAP_TOLERANCE);
            s.union(&geo::MultiPolygon::new(vec![t.to_polygon()]))
        })
        .map_coords(|c| geo::Coord::from_f64_version(c));
    res.iter_mut()
        .filter(|poly| poly.interiors().len() != 0)
        .for_each(|poly| {
            let interiors = poly
                .interiors()
                .iter()
                .map(|ls| geo::Polygon::new(ls.clone(), vec![]))
                .filter_map(|p| {
                    let tris = delaunay_tessellation(p)
                        .into_iter()
                        .filter(|tri| triangle_is_valid(tri))
                        .collect::<Vec<_>>();
                    (!tris.is_empty()).then(|| unify_tri_mesh(tris).0[0].clone())
                })
                .map(|p| p.exterior().clone())
                .collect::<Vec<_>>();
            *poly = geo::Polygon::new(poly.exterior().clone(), interiors);
        });
    res
}

fn triangle_is_valid<F: NeoFloat>(tri: &geo::Triangle<F>) -> bool {
    let [a, b, c] = tri
        .to_lines()
        .map(|l| l.delta().to_f64_version().to_glam().as_vec2());
    [(a, b), (b, c), (c, a)]
        .into_iter()
        .map(|(a, b)| a.angle_between(b))
        .map(|a| angle_ok(a))
        .all(|x| x)
}

fn angle_ok(angle: f32) -> bool {
    let angle = (angle.to_degrees() + 720.0) % 180.0;
    angle.abs() > MINIMUM_ANGLE && (180.0 - angle).abs() > MINIMUM_ANGLE
}
