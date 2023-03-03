use core::ops::Sub;
use falcon::falcon::fpr;
use falcon::fpr::{fpr_add as add, fpr_div as div, fpr_double as double, fpr_expm_p63 as expm_p63, fpr_floor as floor, fpr_half as half, fpr_inv as inv, fpr_lt as lt, fpr_mul as mul, fpr_neg as neg, fpr_of, fpr_rint as rint, fpr_sqrt as sqrt, fpr_sub as sub, fpr_trunc as trunc};
use rand::{Rng, thread_rng};

pub fn fpr_add(x: &[fpr], y: &[fpr]) -> [fpr; 2] {
    let mut d = [0; 2];
    d[0] = add(x[0], y[0]);
    d[1] = add(x[1], y[1]);
    d
}

#[inline(always)]
pub fn fpr_sub(x: &[fpr], y: &[fpr]) -> [fpr; 2] {
    let mut d = [0; 2];
    d[0] = sub(x[0], y[0]);
    d[1] = sub(x[1], y[1]);
    d
}

pub fn fpr_expm_p63(x: &[fpr], ccs: &[fpr]) -> [u64; 2] {
    let mut d = [0; 2];
    d[0] = expm_p63(x[0], ccs[0]);
    d[1] = expm_p63(x[1], ccs[1]);
    d
}

pub fn fpr_mul(x: &[fpr], y: &[fpr]) -> [fpr; 2] {
    let mut d = [0; 2];
    d[0] = mul(x[0], y[0]);
    d[1] = add(mul(x[1], y[0]),
               add(mul(y[1], x[0]), mul(x[1], y[1])));
    d
}

pub fn fpr_sqrt(x: &[fpr]) -> [fpr; 2] {
    let mut d = [0; 2];
    d[0] = sqrt(x[0]);
    d[1] = sqrt(x[1]);
    d
}

#[inline(always)]
pub fn fpr_trunc(x: &[fpr]) -> [i64; 2] {
    let mut d = [0; 2];
    d[0] = trunc(x[0]);
    d[1] = trunc(x[1]);
    d
}


pub fn fpr_div(x: &[fpr], y: &[fpr]) -> [fpr; 2] {
    let d = fpr_inv(y);
    fpr_mul(x, &d)
}


#[inline(always)]
pub fn fpr_rint(x: &[fpr]) -> [i64; 2] {
    let mut d = [0; 2];
    d[0] = rint(x[0]);
    d[1] = rint(x[1]);
    d
}

#[inline(always)]
pub fn fpr_floor(x: &[fpr]) -> [i64; 2] {
    let mut d = [0; 2];
    d[0] = floor(x[0]);
    d[1] = floor(x[1]);
    d
}


#[inline(always)]
pub fn fpr_neg(x: &[fpr]) -> [fpr; 2] {
    let mut d = [0; 2];
    d[0] = neg(x[0]);
    d[1] = neg(x[1]);
    d
}

#[inline(always)]
pub fn fpr_half(x: &[fpr]) -> [fpr; 2] {
    let mut d = [0; 2];
    d[0] = half(x[0]);
    d[1] = half(x[1]);
    d
}

#[inline(always)]
pub fn fpr_double(x: &[fpr]) -> [fpr; 2] {
    let mut d = [0; 2];
    d[0] = double(x[0]);
    d[1] = double(x[1]);
    d
}

#[inline(always)]
pub fn fpr_inv(x: &[fpr]) -> [fpr; 2] {
    let mut d = [0; 2];
    let mut rng = thread_rng();
    let r1: fpr = f64::to_bits(rng.gen_range(-100f64..100f64));
    let share_two: fpr = f64::to_bits(rng.gen_range(-100f64..100f64));
    let share_one = sub(r1, share_two);
    let y = fpr_mul(&[share_one, share_two], x);
    let y_open_inv = inv(add(y[0], y[1]));
    d[0] = mul(share_one, y_open_inv);
    d[1] = mul(share_two, y_open_inv);
    d
}


#[inline(always)]
pub fn fpr_lt(x: &[fpr], y: fpr) -> i32 {
    let xx = add(x[0], x[1]);
    lt(xx, y)
}

#[inline(always)]
pub fn fpr_sqr(x: &[fpr]) -> [fpr; 2] {
    fpr_mul(x, x)
}