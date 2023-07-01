#![allow(dead_code)]
#![allow(unused_results)]
#![allow(unused_must_use)]
#![allow(unused_variables)]

use geo::{BooleanOps, Intersects};
use geo_svg::ToSvg;
use rand::{thread_rng, Rng};

type T = f64;
const NUM_RUNS: usize = 100_000;
const RANGE: f32 = 10_000.0;

fn random_coord(rng: &mut impl Rng) -> geo::Coord<T> {
    let range = rng.gen_range(1.0..RANGE);
    let x = rng.gen_range(-range..range) as f64;
    let y = rng.gen_range(-range..range) as f64;
    geo::Coord { x, y }
}

fn random_coords(rng: &mut impl Rng) -> [Option<geo::Coord<T>>; 3] {
    (0..3)
        .map(|_| random_coord(rng))
        .enumerate()
        // check that points are unique and we don't have degenerate triangles (lines still
        // possible)
        .fold([None; 3], |mut vec, (i, p)| {
            if !vec.contains(&Some(p)) {
                vec[i].replace(p);
            }
            vec
        })
}

fn random_poly(rng: &mut impl Rng) -> geo::Polygon<T> {
    let mut cs = random_coords(rng);
    while cs.contains(&None) {
        cs = random_coords(rng);
    }
    let p = geo::Polygon::new(
        geo::LineString::new(cs.map(|x| x.unwrap()).to_vec()),
        vec![],
    );
    p
}

fn save_polys(ps: [&geo::Polygon<T>; 2], path: &str) {
    if std::path::Path::new(path).exists() {}
    let sps = serde_json::to_string(&ps).unwrap();
    std::fs::write(path, sps);
}

fn test_prog(f: impl Fn(&geo::Polygon<T>, &geo::Polygon<T>) -> geo::MultiPolygon<T>, name: &str) {
    let mut rng = thread_rng();
    let mut make = || random_poly(&mut rng);
    for i in 0..NUM_RUNS {
        //let name = format!("{name}-{i}.html");
        let mut p1 = make();
        let mut p2 = make();
        // make sure polys are intersecting
        while !p1.intersects(&p2) {
            p1 = make();
            p2 = make();
        }
        //save_polys([&p1, &p2], &name);
        f(&p1, &p2);
        //std::fs::remove_file(name);
    }
}

fn test_new_prog(
    f: impl Fn(&geo::Polygon<T>, &geo::Polygon<T>) -> geo::MultiPolygon<T>,
    name: &str,
) {
    let ps = std::fs::read_to_string(name).unwrap();
    let [p1, p2]: [geo::Polygon<T>; 2] = serde_json::from_str(&ps).unwrap();
    f(&p1, &p2);
}

#[test]
fn new() {
    std::fs::read_dir("faildata")
        .unwrap()
        .filter_map(|p| {
            let path = p.ok()?.path();
            let path_str = path.to_str()?;
            (path_str.contains("difference") && path_str.contains("raw-data"))
                .then_some(path_str.to_string())
        })
        .for_each(|path| {
            println!("{path}");
            test_new_prog(BooleanOps::difference, path.as_str());
        })
}

#[test]
fn convert() {
    let path = "intersection-174307.html";
    std::fs::copy(path, format!("{path}-raw-data"));
    let ps = std::fs::read_to_string(path).unwrap();
    let ps: [geo::Polygon<T>; 2] = serde_json::from_str(&ps).unwrap();
    let mp = geo::MultiPolygon::new(ps.to_vec());
    let svg = mp.to_svg();
    std::fs::write(format!("{path}-fail.svg"), svg.to_string());
    std::fs::remove_file(path);
}

const FNS: [fn(&geo::Polygon<T>, &geo::Polygon<T>) -> geo::MultiPolygon<T>; 3] = [
    BooleanOps::difference,
    BooleanOps::union,
    BooleanOps::intersection,
];

macro_rules! make_test_thread {
    ($name:ident) => {
        paste! {
            #[test]
            fn [<test_thread_$name>]() {
                test_prog(FNS.choose(&mut thread_rng()).unwrap(), "thread");
            }
        }
    };
    ( $($name:ident),* ) => {
        $(
            make_test_thread!($name);
        )*
    };
}

use paste::paste;
use rand::seq::SliceRandom;

make_test_thread!(
    t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20,
    t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39
);

#[test]
fn mass_triangle_union_test() {
    let mut rng = thread_rng();
    for i in 0..NUM_RUNS {
        let polys = (0..rng.gen_range(10..=20))
            .map(|_| random_poly(&mut rng))
            .collect::<Vec<_>>();
        let (p1, p2) = polys.split_at(polys.len() / 2);
        let (mp1, mp2) = (
            geo::MultiPolygon::new(p1.to_vec()),
            geo::MultiPolygon::new(p2.to_vec()),
        );
        mp1.union(&mp2);
    }
}
