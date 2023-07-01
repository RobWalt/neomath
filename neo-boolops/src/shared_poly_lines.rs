use geo::{CoordsIter, EuclideanDistance, LinesIter};
use geo_glam_interop::neo_float::NeoFloatConversions;
use geo_glam_interop::to_glam::ConvertToGlam;
use glam::Vec2;
use neo_float::NeoFloat;
use neo_line_segment::d2::def::LineSegment2D;
use neo_line_segment::d2::intersection::Line2DIntersection;

// Magic Numbers
const MINIMUM_OVERLAP_LENGTH: f32 = 0.1;
const PROXIMITY_TOLERANCE: f32 = 0.001;

/// Given two polygons, this function returns the lines of the polygons without real intersections.
///
/// To facilitate this, intersecting or overlapping lines are split up into smaller pieces at the
/// intersection points / overlap end points
pub fn shared_lines<F: NeoFloat>(
    mut p1: geo::MultiPolygon<F>,
    mut p2: geo::MultiPolygon<F>,
) -> Vec<geo::Line<F>> {
    loop {
        let points = p1.coords_iter().chain(p2.coords_iter()).collect::<Vec<_>>();
        let intersecting_lines = poly_lines_iter_enumerated(&p1).find_map(|index_line1| {
            poly_lines_iter_enumerated(&p2)
                .find_map(|index_line2| intersect_indexed_lines(index_line1, index_line2, &points))
        });
        if let Some((idx1, idx2, line1, line2, intersection_kind)) = intersecting_lines {
            p1 = replace_line(p1, idx1, line1, intersection_kind);
            p2 = replace_line(p2, idx2, line2, intersection_kind);
        } else {
            return p1.lines_iter().chain(p2.lines_iter()).collect::<Vec<_>>();
        }
    }
}

/// predicate to check if a point is near one of the points in the given set
fn close_to_normal_points<F: NeoFloat>(other_points: &[geo::Coord<F>], point: Vec2) -> bool {
    other_points.iter().any(|other| {
        let p = other.to_f64_version().to_glam().as_vec2();
        point.distance(p) < PROXIMITY_TOLERANCE
    })
}

fn line_to_line_segment<F: NeoFloat>(line: geo::Line<F>) -> LineSegment2D {
    let start = line.start.to_f64_version().to_glam().as_vec2();
    let end = line.end.to_f64_version().to_glam().as_vec2();
    LineSegment2D::new(start, end)
}

/// Intersect two indexed lines
///
/// Returns `Some(...)` in case:
///
///  - we have a true intersection (intersection which is located on both lines)
///  - we have a true overlap (not complete overlap or tiny overlap)
fn intersect_indexed_lines<F: NeoFloat>(
    (idx1, line1): (PolygonLineIntersectionIdx, geo::Line<F>),
    (idx2, line2): (PolygonLineIntersectionIdx, geo::Line<F>),
    other_points: &[geo::Coord<F>],
) -> Option<(
    PolygonLineIntersectionIdx,
    PolygonLineIntersectionIdx,
    geo::Line<F>,
    geo::Line<F>,
    IntersectionKind,
)> {
    let l1 = line_to_line_segment(line1);
    let l2 = line_to_line_segment(line2);
    match l1.intersection(&l2) {
        Line2DIntersection::IntersectionInBoth(point)
            if !l1.is_endpoint(point)
                && !l2.is_endpoint(point)
                && !close_to_normal_points(other_points, point) =>
        {
            Some((idx1, idx2, line1, line2, IntersectionKind::Point(point)))
        }
        Line2DIntersection::CollinearOverlap(line)
            if line.overlap().length() > MINIMUM_OVERLAP_LENGTH && l1 != l2 && l1.flip() != l2 =>
        {
            Some((
                idx1,
                idx2,
                line1,
                line2,
                IntersectionKind::Overlap(line.overlap()),
            ))
        }
        _ => None,
    }
}

/// Classifies the intersection type
#[derive(Clone, Copy, Debug)]
enum IntersectionKind {
    /// Intersection is a point
    Point(Vec2),
    /// Intersection is a overlap (line)
    Overlap(LineSegment2D),
}

/// Index type that points to a specific line in a polygon (which needs to be replaced)
#[derive(Clone, Copy, Debug)]
enum PolygonLineIntersectionIdx {
    /// index points into exterior
    Exterior {
        /// n-th line in the exterior
        line_idx: usize,
        /// n-th polygon in the multipolygon
        poly_idx: usize,
    },
    /// index points into interiors
    Interior {
        /// n-th interior contains the line
        interior_idx: usize,
        /// n-th line in the chosen interior
        line_idx: usize,
        /// n-th polygon in the multipolygon
        poly_idx: usize,
    },
}

/// helper function which provides an iterator over all lines in a poly together with an index
/// which helps to find that line again if it needs modifications
fn poly_lines_iter_enumerated<F: NeoFloat>(
    poly: &geo::MultiPolygon<F>,
) -> impl Iterator<Item = (PolygonLineIntersectionIdx, geo::Line<F>)> + '_ {
    poly.iter().enumerate().flat_map(|(poly_idx, poly)| {
        poly.exterior()
            .lines()
            .enumerate()
            .map(move |(line_idx, line)| {
                (
                    PolygonLineIntersectionIdx::Exterior { line_idx, poly_idx },
                    line,
                )
            })
            .chain(poly.interiors().into_iter().enumerate().flat_map(
                move |(interior_idx, interior)| {
                    interior.lines().enumerate().map(move |(line_idx, line)| {
                        (
                            PolygonLineIntersectionIdx::Interior {
                                interior_idx,
                                line_idx,
                                poly_idx,
                            },
                            line,
                        )
                    })
                },
            ))
    })
}

/// based on the `kind`, new lines are created which replace the argument `line`. These
/// lines are inserted at the right spot in the polygon with the help of the `idx`
fn replace_line<F: NeoFloat>(
    mut poly: geo::MultiPolygon<F>,
    idx: PolygonLineIntersectionIdx,
    line: geo::Line<F>,
    kind: IntersectionKind,
) -> geo::MultiPolygon<F> {
    let new_lines = match kind {
        IntersectionKind::Point(point) => {
            split_line_at_point(line, geo_from_glam_point(point)).to_vec()
        }
        IntersectionKind::Overlap(overlap) => {
            let [a, b] = overlap.array().map(geo_from_glam_point);
            let overlap = geo::Line::new(a, b);
            split_line_at_line(line, overlap)
        }
    };
    match idx {
        PolygonLineIntersectionIdx::Exterior { line_idx, poly_idx } => {
            poly.iter_mut()
                .enumerate()
                .find(|&(p_idx, _)| p_idx == poly_idx)
                .map(|(_, poly)| {
                    poly.exterior_mut(|ls| {
                        *ls = replace_lines_in_linestring(ls, line_idx, new_lines);
                    });
                });
        }
        PolygonLineIntersectionIdx::Interior {
            interior_idx,
            line_idx,
            poly_idx,
        } => {
            poly.iter_mut()
                .enumerate()
                .find(|&(p_idx, _)| p_idx == poly_idx)
                .map(|(_, poly)| {
                    poly.interiors_mut(|int| {
                        let ls = &mut int[interior_idx];
                        *ls = replace_lines_in_linestring(ls, line_idx, new_lines);
                    });
                });
        }
    }
    poly
}

/// replace the n-th (`idx`-th) line in a linestring with the `new_lines`
fn replace_lines_in_linestring<F: NeoFloat>(
    linestring: &geo::LineString<F>,
    idx: usize,
    new_lines: Vec<geo::Line<F>>,
) -> geo::LineString<F> {
    let mut new_linestring = geo::LineString::new(
        // I'm proud of this :D The garbage I used to even just try to do the same before here was
        // horrible!
        linestring
            .lines()
            .enumerate()
            .flat_map(|(line_idx, line)| {
                if line_idx == idx {
                    new_lines.clone()
                } else {
                    vec![line]
                }
            })
            .map(|line| line.start)
            .collect::<Vec<_>>(),
    );
    new_linestring.close();
    new_linestring
}

/// splits line at point and creates two new lines
fn split_line_at_point<F: NeoFloat>(line: geo::Line<F>, point: geo::Coord<F>) -> [geo::Line<F>; 2] {
    [
        geo::Line::new(line.start, point),
        geo::Line::new(point, line.end),
    ]
}

fn geo_from_glam_point<F: NeoFloat>(v: Vec2) -> geo::Coord<F> {
    geo::Coord {
        x: F::from_raw_f64(v.x as f64),
        y: F::from_raw_f64(v.y as f64),
    }
}

/// splits `line` at endpoints of `overlapping_line`. The size of the outcome vector can be
/// anything in the range 1..=3 based on the overlap
fn split_line_at_line<F: NeoFloat>(
    line: geo::Line<F>,
    overlapping_line: geo::Line<F>,
) -> Vec<geo::Line<F>> {
    // ensure the direction of the resulting lines aligns with the input `line`
    let (start, end) = {
        if overlapping_line.start.euclidean_distance(&line.start)
            < overlapping_line.end.euclidean_distance(&line.start)
        {
            (overlapping_line.start, overlapping_line.end)
        } else {
            (overlapping_line.end, overlapping_line.start)
        }
    };
    [(line.start, start), (start, end), (end, line.end)]
        .into_iter()
        .filter(|(a, b)| a != b)
        .map(|(a, b)| geo::Line::new(a, b))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod shared_poly_lines {
    use rand::{thread_rng, Rng};

    use crate::shared_poly_lines::{
        replace_lines_in_linestring, shared_lines, split_line_at_line, split_line_at_point,
    };
    use crate::unify_mesh::unify_tri_mesh;

    fn random_tri() -> geo::Triangle<f32> {
        fn random_offset() -> geo::Coord<f32> {
            let mut rng = thread_rng();
            geo::Coord::<f32> {
                x: rng.gen_range(-10.0..=10.0),
                y: rng.gen_range(-10.0..=10.0),
            }
        }
        geo::Triangle::<f32>::from(
            [
                geo::Coord::<f32> { x: -10.0, y: -10.0 },
                geo::Coord::<f32> { x: 0.0, y: 10.0 },
                geo::Coord::<f32> { x: 10.0, y: -10.0 },
            ]
            .map(|c| c + random_offset()),
        )
    }

    #[test]
    fn split_line_at_line_test() {
        let l1 = geo::Line::new(
            geo::Coord { x: 0.0, y: 0.0 },
            geo::Coord { x: 10.0, y: 10.0 },
        );
        let l2 = geo::Line::new(geo::Coord { x: 2.0, y: 2.0 }, geo::Coord { x: 8.0, y: 8.0 });
        let split = split_line_at_line(l1, l2);
        assert_eq!(split.len(), 3);
        assert_eq!(
            split[0],
            geo::Line::new(geo::Coord { x: 0.0, y: 0.0 }, geo::Coord { x: 2.0, y: 2.0 },)
        );
        assert_eq!(
            split[1],
            geo::Line::new(geo::Coord { x: 2.0, y: 2.0 }, geo::Coord { x: 8.0, y: 8.0 },)
        );
        assert_eq!(
            split[2],
            geo::Line::new(
                geo::Coord { x: 8.0, y: 8.0 },
                geo::Coord { x: 10.0, y: 10.0 },
            )
        );
    }

    #[test]
    fn split_line_at_point_test() {
        let l = geo::Line::new(
            geo::Coord { x: 0.0, y: 0.0 },
            geo::Coord { x: 10.0, y: 10.0 },
        );
        let p = geo::Coord { x: 5.0, y: 5.0 };
        let split = split_line_at_point(l, p).to_vec();
        assert_eq!(split.len(), 2);
        assert_eq!(
            split[0],
            geo::Line::new(geo::Coord { x: 0.0, y: 0.0 }, geo::Coord { x: 5.0, y: 5.0 },)
        );
        assert_eq!(
            split[1],
            geo::Line::new(
                geo::Coord { x: 5.0, y: 5.0 },
                geo::Coord { x: 10.0, y: 10.0 },
            )
        );
    }

    #[test]
    fn replace_lines_in_linestring_test_1() {
        let mut rect = geo::LineString::new(vec![
            geo::Coord { x: 0.0, y: 0.0 },
            geo::Coord { x: 10.0, y: 0.0 },
            geo::Coord { x: 10.0, y: 10.0 },
            geo::Coord { x: 0.0, y: 10.0 },
        ]);
        rect.close();
        let _rect = replace_lines_in_linestring(
            &rect,
            0,
            vec![
                geo::Line::new(geo::Coord { x: 0.0, y: 0.0 }, geo::Coord { x: 5.0, y: 0.0 }),
                geo::Line::new(
                    geo::Coord { x: 5.0, y: 0.0 },
                    geo::Coord { x: 10.0, y: 0.0 },
                ),
            ],
        );
    }

    #[test]
    fn replace_lines_in_linestring_test_2() {
        let mut rect = geo::LineString::new(vec![
            geo::Coord { x: 0.0, y: 0.0 },
            geo::Coord { x: 10.0, y: 0.0 },
            geo::Coord { x: 10.0, y: 10.0 },
            geo::Coord { x: 0.0, y: 10.0 },
        ]);
        rect.close();
        let rect = replace_lines_in_linestring(
            &rect,
            1,
            vec![
                geo::Line::new(
                    geo::Coord { x: 10.0, y: 0.0 },
                    geo::Coord { x: 10.0, y: 5.0 },
                ),
                geo::Line::new(
                    geo::Coord { x: 10.0, y: 5.0 },
                    geo::Coord { x: 10.0, y: 10.0 },
                ),
            ],
        );
        println!(
            "{}",
            rect.lines()
                .map(|l| { format!("{l:?}") })
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    #[test]
    fn replace_lines_in_linestring_test_3() {
        let mut rect = geo::LineString::new(vec![
            geo::Coord { x: 0.0, y: 0.0 },
            geo::Coord { x: 10.0, y: 0.0 },
            geo::Coord { x: 10.0, y: 10.0 },
            geo::Coord { x: 0.0, y: 10.0 },
        ]);
        rect.close();
        let _rect = replace_lines_in_linestring(
            &rect,
            3,
            vec![
                geo::Line::new(
                    geo::Coord { x: 0.0, y: 10.0 },
                    geo::Coord { x: 0.0, y: 5.0 },
                ),
                geo::Line::new(geo::Coord { x: 0.0, y: 5.0 }, geo::Coord { x: 0.0, y: 0.0 }),
            ],
        );
    }

    #[test]
    fn simple_triangle() {
        let tri1 = random_tri();
        let tri2 = random_tri();

        let _shared_lines = shared_lines(
            geo::MultiPolygon::new(vec![tri1.to_polygon()]),
            geo::MultiPolygon::new(vec![tri2.to_polygon()]),
        );
        println!(
            "{}",
            _shared_lines
                .iter()
                .map(|l| { format!("{l:?}") })
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    const POINT_NEAR_LINE_CASE_DATA_1: &str = "[[[{\"x\":4.1805367,\"y\":12.566143},{\"x\":-10.097773,\"y\":4.4208674},{\"x\":-3.4187489,\"y\":-9.284574}],[{\"x\":4.2416673,\"y\":-19.868622},{\"x\":4.1805367,\"y\":12.566143},{\"x\":-3.4187489,\"y\":-9.284574}],[{\"x\":14.083126,\"y\":6.6489034},{\"x\":-5.416109,\"y\":13.041119},{\"x\":-9.384744,\"y\":-19.87027}],[{\"x\":21.062029,\"y\":-19.671753},{\"x\":14.083126,\"y\":6.6489034},{\"x\":-9.384744,\"y\":-19.87027}]],[[{\"x\":4.1805367,\"y\":12.566143},{\"x\":-10.097773,\"y\":4.4208674},{\"x\":-3.4187489,\"y\":-9.284574}],[{\"x\":4.2416673,\"y\":-19.868622},{\"x\":4.1805367,\"y\":12.566143},{\"x\":-3.4187489,\"y\":-9.284574}]],[[{\"x\":14.083126,\"y\":6.6489034},{\"x\":-5.416109,\"y\":13.041119},{\"x\":-9.384744,\"y\":-19.87027}],[{\"x\":21.062029,\"y\":-19.671753},{\"x\":14.083126,\"y\":6.6489034},{\"x\":-9.384744,\"y\":-19.87027}]]]";

    const POINT_NEAR_LINE_CASE_DATA_2: &str = "[[[{\"x\":15.748137,\"y\":-2.8293},{\"x\":7.9399643,\"y\":8.222724},{\"x\":-0.60329056,\"y\":0.032589912}],[{\"x\":-0.60329056,\"y\":0.032589912},{\"x\":-18.32907,\"y\":-9.072751},{\"x\":15.748137,\"y\":-2.8293}],[{\"x\":8.529032,\"y\":7.4794416},{\"x\":-16.22086,\"y\":7.2657876},{\"x\":9.948565,\"y\":-7.7774897}],[{\"x\":-3.8156624,\"y\":-18.928986},{\"x\":9.948565,\"y\":-7.7774897},{\"x\":-16.22086,\"y\":7.2657876}]],[[{\"x\":15.748137,\"y\":-2.8293},{\"x\":7.9399643,\"y\":8.222724},{\"x\":-0.60329056,\"y\":0.032589912}],[{\"x\":-0.60329056,\"y\":0.032589912},{\"x\":-18.32907,\"y\":-9.072751},{\"x\":15.748137,\"y\":-2.8293}]],[[{\"x\":8.529032,\"y\":7.4794416},{\"x\":-16.22086,\"y\":7.2657876},{\"x\":9.948565,\"y\":-7.7774897}],[{\"x\":-3.8156624,\"y\":-18.928986},{\"x\":9.948565,\"y\":-7.7774897},{\"x\":-16.22086,\"y\":7.2657876}]]]";

    // context this previously failed because the point that was near the line created a small
    // triangle. The small triangle created a small overlap on the line the near point was located
    // on. This overlap isn't processed (no splits or anything similar) and then it is found in the
    // next loop again and the loop is stuck.
    //
    // It was resolved by requiring a minimum length for the overlap line. I'm not sure if the
    // issue is fixed completely with this
    #[test]
    fn point_on_edge() {
        for data in [POINT_NEAR_LINE_CASE_DATA_1, POINT_NEAR_LINE_CASE_DATA_2].into_iter() {
            let [_, p1, p2] = serde_json::from_str::<[Vec<geo::Triangle<f32>>; 3]>(&data)
                .unwrap()
                .map(|mesh| unify_tri_mesh(mesh).0[0].clone());
            let _shared_lines = shared_lines(
                geo::MultiPolygon::new(vec![p1]),
                geo::MultiPolygon::new(vec![p2]),
            );
        }
    }

    #[test]
    fn with_hole() {
        let p1 = geo::Rect::new(
            geo::Coord { x: 0.0, y: 0.0 },
            geo::Coord { x: 10.0, y: 10.0 },
        );
        let p1_inner = geo::Rect::new(geo::Coord { x: 2.5, y: 2.5 }, geo::Coord { x: 7.5, y: 7.5 });
        let p1 = geo::Polygon::new(
            p1.to_polygon().exterior().clone(),
            vec![p1_inner.to_polygon().exterior().clone()],
        );
        let p2 = geo::Rect::new(
            geo::Coord { x: 5.0, y: 0.0 },
            geo::Coord { x: 15.0, y: 10.0 },
        )
        .to_polygon();
        let _shared_lines = shared_lines(
            geo::MultiPolygon::new(vec![p1]),
            geo::MultiPolygon::new(vec![p2]),
        );
    }
}
