use criterion::{criterion_group, criterion_main, Criterion};
use geometry_predicates::predicates::*;
use num_rational::BigRational;
use rand::distributions::Standard;
use rand::prelude::*;
use std::cmp::Ordering;
use std::ops::BitXor;

fn orient2d_word_exact(p: [i64; 2], q: [i64; 2], r: [i64; 2]) -> Ordering {
    fn diff(a: i64, b: i64) -> (u128, bool) {
        if b > a {
            (b.wrapping_sub(a) as u64 as u128, true)
        } else {
            (a.wrapping_sub(b) as u64 as u128, false)
        }
    }
    let (ux, ux_neg) = diff(q[0], p[0]);
    let (vy, vy_neg) = diff(r[1], p[1]);
    let ux_vy_neg = ux_neg.bitxor(vy_neg) && ux != 0 && vy != 0;
    let (uy, uy_neg) = diff(q[1], p[1]);
    let (vx, vx_neg) = diff(r[0], p[0]);
    let uy_vx_neg = uy_neg.bitxor(vx_neg) && uy != 0 && vx != 0;
    match (ux_vy_neg, uy_vx_neg) {
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        (true, true) => (uy * vx).cmp(&(ux * vy)),
        (false, false) => (ux * vy).cmp(&(uy * vx)),
    }
}

fn orient2d_word_inaccurate(p: [i64; 2], q: [i64; 2], r: [i64; 2]) -> Ordering {
    let acx = p[0] - r[0];
    let bcx = q[0] - r[0];
    let acy = p[1] - r[1];
    let bcy = q[1] - r[1];
    (acx * bcy).cmp(&(acy * bcx))
}

fn orient2d_bigrational(p: [f64; 2], q: [f64; 2], r: [f64; 2]) -> Ordering {
    let big_r_0 = BigRational::from_float(r[0]).unwrap();
    let big_r_1 = BigRational::from_float(r[1]).unwrap();
    let acx = BigRational::from_float(p[0]).unwrap() - &big_r_0;
    let bcx = BigRational::from_float(q[0]).unwrap() - big_r_0;
    let acy = BigRational::from_float(p[1]).unwrap() - &big_r_1;
    let bcy = BigRational::from_float(q[1]).unwrap() - big_r_1;
    (acx * bcy).cmp(&(acy * bcx))
}

fn random(c: &mut Criterion) {
    let mut group = c.benchmark_group("random");
    let mut rng = StdRng::from_entropy();
    let mut arr: [f64; 6] = [0.0; 6];
    for elt in arr.iter_mut() {
        *elt = rng.sample(Standard);
    }
    let inp = ([arr[0], arr[1]], [arr[2], arr[3]], [arr[4], arr[5]]);
    let inp_word = (
        [rng.sample(Standard), rng.sample(Standard)],
        [rng.sample(Standard), rng.sample(Standard)],
        [rng.sample(Standard), rng.sample(Standard)],
    );
    group.bench_with_input("inaccurate", &inp, |b, &(x, y, z)| {
        b.iter(|| orient2d_fast(x, y, z));
    });
    group.bench_with_input("exact", &inp, |b, &(x, y, z)| {
        b.iter(|| orient2d_exact(x, y, z));
    });
    group.bench_with_input("exact_slow", &inp, |b, &(x, y, z)| {
        b.iter(|| orient2d_slow(x, y, z));
    });
    group.bench_with_input("adaptive", &inp, |b, &(x, y, z)| {
        b.iter(|| orient2d(x, y, z));
    });
    group.bench_with_input("word_exact", &inp_word, |b, &(x, y, z)| {
        b.iter(|| orient2d_word_exact(x, y, z));
    });
    group.bench_with_input("word_inaccurate", &inp_word, |b, &(x, y, z)| {
        b.iter(|| orient2d_word_inaccurate(x, y, z));
    });
    group.bench_with_input("rational", &inp, |b, &(x, y, z)| {
        b.iter(|| orient2d_bigrational(x, y, z));
    });
    group.finish();
}

fn colinear(c: &mut Criterion) {
    let mut group = c.benchmark_group("colinear");
    let mut rng = StdRng::from_entropy();
    let mut arr: [f64; 2] = [0.0; 2];
    for elt in arr.iter_mut() {
        *elt = rng.sample(Standard);
    }
    let inp = (
        [arr[0], arr[1]],
        [arr[0] + 1.0, arr[1] + 1.0],
        [arr[0] + 2.0, arr[1] + 2.0],
    );
    let x = rng.sample(Standard);
    let y = rng.sample(Standard);
    let inp_word = ([x, y], [x + 1, y + 1], [x + 2, y + 2]);
    group.bench_with_input("inaccurate", &inp, |b, &(x, y, z)| {
        b.iter(|| orient2d_fast(x, y, z));
    });
    group.bench_with_input("exact", &inp, |b, &(x, y, z)| {
        b.iter(|| orient2d_exact(x, y, z));
    });
    group.bench_with_input("exact_slow", &inp, |b, &(x, y, z)| {
        b.iter(|| orient2d_slow(x, y, z));
    });
    group.bench_with_input("adaptive", &inp, |b, &(x, y, z)| {
        b.iter(|| orient2d(x, y, z));
    });
    group.bench_with_input("word_exact", &inp_word, |b, &(x, y, z)| {
        b.iter(|| orient2d_word_exact(x, y, z));
    });
    group.bench_with_input("word_inaccurate", &inp_word, |b, &(x, y, z)| {
        b.iter(|| orient2d_word_inaccurate(x, y, z));
    });
    group.bench_with_input("rational", &inp, |b, &(x, y, z)| {
        b.iter(|| orient2d_bigrational(x, y, z));
    });
    group.finish();
}

criterion_group!(benches, random, colinear);

criterion_main!(benches);
