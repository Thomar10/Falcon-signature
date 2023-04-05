#![allow(dead_code)]

use criterion::{Criterion, criterion_group, criterion_main};
use rand::prelude::*;

use falcon::falcon::fpr;
use falcon::fft::fft;
use falcon::fpr::{fpr_add, fpr_mul, fpr_norm64, fpr_sub};
use falcon_masked::fft_masked::fft as mfft;
use falcon_masked::fft_masked_deep::secure_fft;
use falcon_masked::fpr_masked::{fpr_add as fpr_madd, fpr_mul as fpr_mmul};
use falcon_masked::fpr_masked_deep::{secure_fpr_add, secure_fpr_norm, secure_mul};

pub fn fft_unmasked(c: &mut Criterion) {
    const LOGN: usize = 10;
    const SIZE: usize = 1 << LOGN;
    let mut x: [fpr; SIZE] = create_random_fpr_array::<SIZE>();
    c.bench_function("fft_unmasked unmasked add", |b| b.iter(|| fft(&mut x, LOGN as u32)));
}

pub fn fft_fpr_masked(c: &mut Criterion) {
    const LOGN: usize = 10;
    const SIZE: usize = 1 << LOGN;
    let mut x: [[fpr; 2]; SIZE] = create_random_masked_fpr_array::<SIZE, 2>();
    c.bench_function("fft masked fpr", |b| b.iter(|| {
        secure_fft::<2>(&mut x, LOGN as u32);
    }));
}

pub fn better_fft_fpr_masked(c: &mut Criterion) {
    const LOGN: usize = 10;
    const SIZE: usize = 1 << LOGN;
    let mut x: [[fpr; 2]; SIZE] = create_random_masked_better_fpr_array::<SIZE, 2>();
    c.bench_function("fft masked fpr better", |b| b.iter(|| {
        mfft(&mut x, LOGN as u32);
    }));
}

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

pub fn fpr_masked_add_better(c: &mut Criterion) {
    let x = create_random_fpr();
    let x_share = create_random_fpr();
    let y = create_random_fpr();
    let y_share = create_random_fpr();
    let x_mask: [fpr; 2] = [fpr_sub(x, x_share), x_share];
    let y_mask: [fpr; 2] = [fpr_sub(y, y_share), y_share];

    c.bench_function("fpr masked add better", |b|
        b.iter(|| fpr_madd(&x_mask, &y_mask)));
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

pub fn fpr_masked_mul_better(c: &mut Criterion) {
    let x = create_random_fpr();
    let x_share = create_random_fpr();
    let y = create_random_fpr();
    let y_share = create_random_fpr();
    let x_mask: [fpr; 2] = [fpr_sub(x, x_share), x_share];
    let y_mask: [fpr; 2] = [fpr_sub(y, y_share), y_share];

    c.bench_function("fpr masked mul better", |b|
        b.iter(|| fpr_mmul(&x_mask, &y_mask)));
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

pub fn fpr_unmasked_mul(c: &mut Criterion) {
    let x = create_random_fpr();
    let y = create_random_fpr();
    c.bench_function("fpr unmasked mul", |b| b.iter(|| fpr_mul(x, y)));
}

pub fn create_random_fpr_array<const SIZE: usize>() -> [fpr; SIZE] {
    let mut array: [fpr; SIZE] = [0; SIZE];
    for i in 0..SIZE {
        array[i] = create_random_fpr();
    }
    array
}

pub fn create_random_masked_fpr_array<const SIZE: usize, const ORDER: usize>() -> [[fpr; ORDER]; SIZE] {
    let mut array: [[fpr; ORDER]; SIZE] = [[0; ORDER]; SIZE];
    for i in 0..SIZE {
        let mut val: [fpr; ORDER] = [0; ORDER];
        let value = create_random_fpr();
        let mask = create_random_fpr();
        val[0] = value;
        val[1] = value ^ mask;
        array[i] = val;
    }
    array
}

pub fn create_random_masked_better_fpr_array<const SIZE: usize, const ORDER: usize>() -> [[fpr; ORDER]; SIZE] {
    let mut array: [[fpr; ORDER]; SIZE] = [[0; ORDER]; SIZE];
    for i in 0..SIZE {
        let mut val: [fpr; ORDER] = [0; ORDER];
        let value = create_random_fpr();
        let mask = create_random_fpr();
        val[0] = value;
        val[1] = fpr_sub(value, mask);
        array[i] = val;
    }
    array
}

pub fn create_random_fpr() -> fpr {
    let mut rng = StdRng::seed_from_u64(42);
    let random: f64 = rng.gen_range(-100f64..100f64);
    return f64::to_bits(random);
}

criterion_group!(benches, fpr_unmasked_mul, fpr_masked_mul, fpr_masked_mul_better, fpr_unmasked_norm, fpr_masked_norm, fpr_unmasked_add,
    fpr_masked_add, fpr_masked_add_better, better_fft_fpr_masked, fft_fpr_masked, fft_unmasked);
criterion_main!(benches);