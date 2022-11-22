#[macro_use]
extern crate criterion;
extern crate dev_utils;

use downsample_rs::minmaxlttb as minmaxlttb_mod;

use criterion::{black_box, Criterion};
use dev_utils::{config, utils};
use ndarray::Array1;

fn minmaxlttb_f32_random_array_long_single_core(c: &mut Criterion) {
    let n = config::ARRAY_LENGTH_LONG;
    let x = Array1::from((0..n).map(|i| i as i32).collect::<Vec<i32>>());
    let y = utils::get_random_array::<f32>(n, f32::MIN, f32::MAX);
    c.bench_function("mmlttb_scal_f32", |b| {
        b.iter(|| {
            minmaxlttb_mod::minmaxlttb_scalar(
                black_box(x.view()),
                black_box(y.view()),
                black_box(2_000),
            )
        })
    });
    c.bench_function("mlttb_simd_f32", |b| {
        b.iter(|| {
            minmaxlttb_mod::minmaxlttb_simd(
                black_box(x.view()),
                black_box(y.view()),
                black_box(2_000),
            )
        })
    });
}

fn minmaxlttb_f32_random_array_long_multi_core(c: &mut Criterion) {
    let n = config::ARRAY_LENGTH_LONG;
    let x = Array1::from((0..n).map(|i| i as i32).collect::<Vec<i32>>());
    let y = utils::get_random_array::<f32>(n, f32::MIN, f32::MAX);
    c.bench_function("mmlttb_scal_p_f32", |b| {
        b.iter(|| {
            minmaxlttb_mod::minmaxlttb_scalar_parallel(
                black_box(x.view()),
                black_box(y.view()),
                black_box(2_000),
            )
        })
    });
    c.bench_function("mlttb_simd_p_f32", |b| {
        b.iter(|| {
            minmaxlttb_mod::minmaxlttb_simd_parallel(
                black_box(x.view()),
                black_box(y.view()),
                black_box(2_000),
            )
        })
    });
}

fn minmaxlttb_f32_random_array_50M_single_core(c: &mut Criterion) {
    let n = 50_000_000;
    let x = Array1::from((0..n).map(|i| i as i32).collect::<Vec<i32>>());
    let y = utils::get_random_array::<f32>(n, f32::MIN, f32::MAX);
    c.bench_function("mmlttb_scal_50M_f32", |b| {
        b.iter(|| {
            minmaxlttb_mod::minmaxlttb_scalar(
                black_box(x.view()),
                black_box(y.view()),
                black_box(2_000),
            )
        })
    });
    c.bench_function("mmlttb_simd_50M_f32", |b| {
        b.iter(|| {
            minmaxlttb_mod::minmaxlttb_simd(
                black_box(x.view()),
                black_box(y.view()),
                black_box(2_000),
            )
        })
    });
}

fn minmaxlttb_f32_random_array_50M_multi_core(c: &mut Criterion) {
    let n = 50_000_000;
    let x = Array1::from((0..n).map(|i| i as i32).collect::<Vec<i32>>());
    let y = utils::get_random_array::<f32>(n, f32::MIN, f32::MAX);
    c.bench_function("mmlttb_scal_p_50M_f32", |b| {
        b.iter(|| {
            minmaxlttb_mod::minmaxlttb_scalar_parallel(
                black_box(x.view()),
                black_box(y.view()),
                black_box(2_000),
            )
        })
    });
    c.bench_function("mmlttb_simd_p_50M_f32", |b| {
        b.iter(|| {
            minmaxlttb_mod::minmaxlttb_simd_parallel(
                black_box(x.view()),
                black_box(y.view()),
                black_box(2_000),
            )
        })
    });
}

fn minmaxlttb_without_x_f32_random_array_50M_single_core(c: &mut Criterion) {
    let n = 50_000_000;
    let y = utils::get_random_array::<f32>(n, f32::MIN, f32::MAX);
    c.bench_function("mmlttbnox_scal_50M_f32", |b| {
        b.iter(|| {
            minmaxlttb_mod::minmaxlttb_scalar_without_x(black_box(y.view()), black_box(2_000))
        })
    });
    c.bench_function("mmlttbnox_simd_50M_f32", |b| {
        b.iter(|| minmaxlttb_mod::minmaxlttb_simd_without_x(black_box(y.view()), black_box(2_000)))
    });
}

fn minmaxlttb_without_x_f32_random_array_50M_multi_core(c: &mut Criterion) {
    let n = 50_000_000;
    let y = utils::get_random_array::<f32>(n, f32::MIN, f32::MAX);
    c.bench_function("mlttbnox_scal_p_50M_f32", |b| {
        b.iter(|| {
            minmaxlttb_mod::minmaxlttb_scalar_without_x_parallel(
                black_box(y.view()),
                black_box(2_000),
            )
        })
    });
    c.bench_function("mlttbnox_simd_p_50M_f32", |b| {
        b.iter(|| {
            minmaxlttb_mod::minmaxlttb_simd_without_x_parallel(
                black_box(y.view()),
                black_box(2_000),
            )
        })
    });
}

criterion_group!(
    benches,
    // minmaxlttb_f32_random_array_long_single_core,
    // minmaxlttb_f32_random_array_long_multi_core,
    minmaxlttb_f32_random_array_50M_single_core,
    minmaxlttb_f32_random_array_50M_multi_core,
    minmaxlttb_without_x_f32_random_array_50M_single_core,
    minmaxlttb_without_x_f32_random_array_50M_multi_core,
    // minmaxlttb_f32_random_array_100m
);
criterion_main!(benches);