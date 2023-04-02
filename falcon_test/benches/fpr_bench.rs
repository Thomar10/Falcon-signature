#![allow(dead_code)]

use criterion::{Criterion, criterion_group, criterion_main};
use rand::prelude::*;

use falcon::falcon::fpr;
use falcon::fpr::{fpr_add, fpr_mul, fpr_norm64, fpr_sub};
use falcon_masked::fpr_masked_deep::{secure_fpr_add, secure_fpr_norm, secure_fpr_sub, secure_mul};

pub fn fpr_unmasked_add(c: &mut Criterion) {
    let x = create_random_fpr();
    let y = create_random_fpr();
    c.bench_function("fpr unmasked add", |b| b.iter(|| fpr_add(x, y)));
}

pub fn fpr_masked_add(c: &mut Criterion) {
    let x = create_random_fpr();
    let x_share = create_random_fpr();
    let y = create_random_fpr();
    let y_share = create_random_fpr();
    let x_mask: [fpr; 2] = [x ^ x_share, x_share];
    let y_mask: [fpr; 2] = [y ^ y_share, y_share];
    c.bench_function("fpr masked add", |b| b.iter(|| {
        secure_fpr_add::<2>(&x_mask, &y_mask);
    }));
}

pub fn fpr_unmasked_sub(c: &mut Criterion) {
    let x = create_random_fpr();
    let y = create_random_fpr();
    c.bench_function("fpr unmasked sub", |b| b.iter(|| fpr_sub(x, y)));
}

pub fn fpr_masked_sub(c: &mut Criterion) {
    let x = create_random_fpr();
    let x_share = create_random_fpr();
    let y = create_random_fpr();
    let y_share = create_random_fpr();
    let x_mask: [fpr; 2] = [x ^ x_share, x_share];
    let y_mask: [fpr; 2] = [y ^ y_share, y_share];
    c.bench_function("fpr masked sub", |b| b.iter(|| {
        secure_fpr_sub::<2>(&x_mask, &y_mask);
    }));
}


pub fn fpr_unmasked_mul(c: &mut Criterion) {
    let x = create_random_fpr();
    let y = create_random_fpr();
    c.bench_function("fpr unmasked mul", |b| b.iter(|| fpr_mul(x, y)));
}

pub fn fpr_unmasked_norm(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(42);
    let x: u64 = rng.next_u64();
    let e: i16 = (rng.next_u32() >> 16) as i16;
    c.bench_function("fpr unmasked norm", |b| b.iter(|| fpr_norm64(x, e as i32)));
}

pub fn fpr_masked_norm(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(42);
    let x: u64 = rng.next_u64();
    let share_x: u64 = rng.next_u64();
    let x_share: [u64; 2] = [x ^ share_x, share_x];
    let e: i16 = (rng.next_u32() >> 16) as i16;
    let share_e: i16 = (rng.next_u32() >> 16) as i16;
    let e_share: [i16; 2] = [(e as i16).wrapping_sub(share_e), share_e];
    c.bench_function("fpr masked norm", |b| b.iter(|| secure_fpr_norm::<2>(&x_share, &e_share)));
}

pub fn fpr_masked_mul(c: &mut Criterion) {
    let x = create_random_fpr();
    let x_share = create_random_fpr();
    let y = create_random_fpr();
    let y_share = create_random_fpr();
    let x_mask: [fpr; 2] = [x ^ x_share, x_share];
    let y_mask: [fpr; 2] = [y ^ y_share, y_share];

    c.bench_function("fpr masked mul", |b|
        b.iter(|| secure_mul::<2>(&x_mask, &y_mask)));
}

pub fn create_random_fpr() -> fpr {
    let mut rng = StdRng::seed_from_u64(42);
    let random: f64 = rng.gen_range(-100f64..100f64);
    return f64::to_bits(random);
}

criterion_group!(benches, fpr_unmasked_mul, fpr_masked_mul, fpr_unmasked_norm, fpr_masked_norm, fpr_unmasked_add, fpr_masked_add, fpr_unmasked_sub, fpr_masked_sub);
criterion_main!(benches);