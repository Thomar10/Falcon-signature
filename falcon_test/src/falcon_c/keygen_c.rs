#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::falcon_c::shake_c::InnerShake256Context;

#[link(name = "keygen", kind = "static")]
extern "C" {
    pub fn modp_set_func(x: i32, p: u32) -> u32;
    pub fn modp_norm_func(x: u32, p: u32) -> i32;
    pub fn modp_ninv31_func(p: u32) -> u32;
    pub fn modp_R_func(p: u32) -> u32;
    pub fn modp_add_func(a: u32, b: u32, p: u32) -> u32;
    pub fn modp_sub_func(a: u32, b: u32, p: u32) -> u32;
    pub fn modp_montymul_func(a: u32, b: u32, p: u32, p0i: u32) -> u32;
    pub fn modp_R2_func(p: u32, p0i: u32) -> u32;
    pub fn modp_Rx_func(x: u32, p: u32, p0i: u32, R2: u32) -> u32;
    pub fn modp_div_func(a: u32, b: u32, p: u32, p0i: u32, r: u32) -> u32;
    pub fn modp_mkgm2_func(gm: *const u32, igm: *const u32, logn: u32, g: u32, p: u32, p0i: u32);
    pub fn modp_NTT2_ext_func(a: *const u32, stride: usize, gm: *const u32, logn: u32, p: u32, p0i: u32);
    pub fn modp_iNTT2_ext_func(a: *const u32, stride: usize, igm: *const u32, logn: u32, p: u32, p0i: u32);
    pub fn modp_poly_rec_res_func(f: *const u32, logn: u32, p: u32, p0i: u32, R2: u32);
    pub fn zint_sub_func(a: *const u32, b: *const u32, len: usize, ctl: u32) -> u32;
    pub fn zint_mul_small_func(m: *const u32, mlen: usize, x: u32) -> u32;
    pub fn zint_mod_small_unsigned_func(d: *const u32, dlen: usize, p: u32, p0i: u32, R2: u32) -> u32;
    pub fn zint_mod_small_signed_func(d: *const u32, dlen: usize, p: u32, p0i: u32, R2: u32, Rx: u32) -> u32;
    pub fn zint_add_mul_small_func(x: *const u32, y: *const u32, len: usize, s: u32);
    pub fn zint_norm_zero_func(x: *const u32, p: *const u32, len: usize);
    pub fn zint_rebuild_CRT_func(xx: *const u32, xlen: usize, xstride: usize, num: u64, primes: *const small_prime, normalized_signed: i32, tmp: *const u32);
    pub fn zint_negate_func(a: *const u32, len: usize, ctl: u32);
    pub fn zint_co_reduce_func(a: *const u32, b: *const u32, len: usize, xa: i64, xb: i64, ya: i64, yb: i64) -> u32;
    pub fn zint_finish_mod_func(a: *const u32, len: usize, m: *const u32, neg: u32);
    pub fn zint_co_reduce_mod_func(a: *const u32, b: *const u32, m: *const u32, len: usize, m0i: u32, xa: i64, xb: i64, ya: i64, yb: i64);
    pub fn zint_bezout_func(u: *const u32, v: *const u32, x: *const u32, y: *const u32, len: usize, tmp: *const u32) -> i32;
    pub fn zint_add_scaled_mul_small_func(x: *const u32, xlen: usize, y: *const u32, ylen: usize, k: i32, sch: u32, scl: u32);
    pub fn zint_sub_scaled_func(x: *const u32, xlen: usize, y: *const u32, ylen: usize, sch: u32, scl: u32);
    pub fn zint_one_to_plain_func(x: *const u32) -> i32;
    pub fn poly_big_to_fp_func(d: *const u64, f: *const u32, flen: usize, fstride: usize, logn: u32);
    pub fn poly_big_to_small_func(d: *const i8, s: *const u32, lim: i32, logn: u32) -> i32;
    pub fn poly_sub_scaled_func(F: *const u32, Flen: usize, Fstride: usize, f: *const u32, flen: usize, fstride: usize, k: *const i32, sch: u32, scl: u32, logn: u32);
    pub fn poly_sub_scaled_ntt_func(F: *const u32, Flen: usize, Fstride: usize, f: *const u32, flen: usize, fstride: usize, k: *const i32, sch: u32, scl: u32, logn: u32, tmp: *const u32);
    pub fn get_rng_u64_func(rng: *const InnerShake256Context) -> u64;
    pub fn mkgauss_func(rng: *const InnerShake256Context, logn: u32) -> i32;
    pub fn poly_small_sqnorm_func(f: *const i8, logn: u32) -> u32;
    pub fn poly_small_to_fp_func(x: *const u64, f: *const i8, logn: u32);
    pub fn make_fg_step_func(data: *const u32, logn: u32, depth: usize, in_ntt: bool, out_ntt: bool);
    pub fn make_fg_func(data: *const u32, f: *const i8, g: *const i8, logn: u32, depth: u32, out_ntt: bool);
    pub fn solve_NTRU_deepest_func(logn_top: u32, f: *const i8, g: *const i8, tmp: *const u32) -> i32;
    pub fn solve_NTRU_intermediate_func(logn_top: u32, f: *const i8, g: *const i8, depth: u32, tmp: *const u32) -> i32;
    pub fn solve_NTRU_binary_depth1_func(logn_top: u32, f: *const i8, g: *const i8, tmp: *const u32) -> i32;
    pub fn solve_NTRU_binary_depth0_func(logn_top: u32, f: *const i8, g: *const i8, tmp: *const u32) -> i32;
    pub fn solve_NTRU_func(logn_top: u32, F: *const i8, G: *const i8, f: *const i8, g: *const i8, lim: i32, tmp: *const u32) -> i32;
    pub fn poly_small_mkgauss_func(rng: *const InnerShake256Context, f: *const i8, logn: u32);
    pub fn falcon_inner_keygen(rng: *const InnerShake256Context, f: *const i8, g: *const i8, F: *const i8, G: *const i8, h: *const u16, logn: u32, tmp: *const u8);
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct small_prime {
    pub p: u32,
    pub g: u32,
    pub s: u32,
}