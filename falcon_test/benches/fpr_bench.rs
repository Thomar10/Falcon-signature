#![allow(dead_code)]

use criterion::{Criterion, criterion_group, criterion_main};
use rand::prelude::*;

use falcon::falcon::fpr;
use falcon::fpr::fpr_mul;
use falcon_masked::fpr_masked_deep::secure_mul;

pub fn fpr_unmasked_mul(c: &mut Criterion) {
    let x = create_random_fpr();
    let y = create_random_fpr();
    c.bench_function("fpr unmasked", |b| b.iter(|| fpr_mul(x, y)));
}


pub fn fpr_masked_mul(c: &mut Criterion) {
    let x = create_random_fpr();
    let y = create_random_fpr();
    let mut rng = thread_rng();
    let sharex: u64 = rng.gen_range(0..2);
    let x_sign = x >> 63;
    let s_sharex: [u64; 2] = [(x_sign as u64) ^ sharex, sharex];
    let share_ex: i16 = random();
    let x_exp = (x >> 52) & 0x7FF;
    let mut e_sharex: [i16; 2] = [((x_exp) as i16).wrapping_sub(share_ex), share_ex];
    let share_mx: i64 = rng.gen_range(0..18014398509481983);
    let x_man = x & 0xFFFFFFFFFFFFF;
    let mut m_sharex: [i128; 2] = [((x_man) as i128).wrapping_sub(share_mx as i128), share_mx as i128];

    let sharey: u64 = rng.gen_range(0..2);
    let s_sharey: [u64; 2] = [((y >> 63) as u64) ^ sharey, sharey];
    let share_ey: i16 = random();
    let mut e_sharey: [i16; 2] = [(((y >> 52) & 0x7FF) as i16).wrapping_sub(share_ey), share_ey];
    let share_my: i64 = rng.gen_range(0..18014398509481983);
    let y_man = y & 0xFFFFFFFFFFFFF;
    let mut m_sharey: [i128; 2] = [(y_man as i128).wrapping_sub(share_my as i128), share_my as i128];


    c.bench_function("fpr masked ", |b|
        b.iter(|| secure_mul::<2>(&s_sharex, &mut e_sharex, &mut m_sharex, &s_sharey, &mut e_sharey, &mut m_sharey)));
}

pub fn create_random_fpr() -> fpr {
    let mut rng = StdRng::seed_from_u64(42);
    let random: f64 = rng.gen_range(-100f64..100f64);
    return f64::to_bits(random);
}

criterion_group!(benches, fpr_unmasked_mul, fpr_masked_mul);
criterion_main!(benches);