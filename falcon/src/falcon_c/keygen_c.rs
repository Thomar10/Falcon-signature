use std::ffi::c_void;
use crate::falcon_c::shake_c::InnerShake256Context;

#[link(name = "keygen", kind = "static")]
extern "C" {
    #[allow(dead_code)]
    pub fn modp_set(x: i32, p: u32) -> u32;
    #[allow(dead_code)]
    pub fn modp_norm(x: i32, p: u32) -> i32;
    #[allow(dead_code)]
    pub fn modp_ninv31(p: u32) -> u32;
    #[allow(dead_code)]
    pub fn modp_R(p: u32) -> u32;
    #[allow(dead_code)]
    pub fn modp_add(a: u32, b: u32, p: u32) -> u32;
    #[allow(dead_code)]
    pub fn modp_sub(a: u32, b: u32, p: u32) -> u32;
    #[allow(dead_code)]
    pub fn modp_montymul(a: u32, b: u32, p: u32, p0i: u32) -> u32;
    #[allow(dead_code)]
    pub fn modp_R2(p: u32, p0i: u32) -> u32;
    #[allow(dead_code)]
    pub fn modp_Rx(x: u32, p: u32, p0i: u32, R2: u32) -> u32;
    #[allow(dead_code)]
    pub fn modp_div(a: u32, bp: u32, p: u32, p0i: u32, R2: u32) -> u32;
    #[allow(dead_code)]
    pub fn modp_mkgm2(gm: *const u32, igm: *const u32, logn: u32, g: u32, p: u32, p0i: u32);
    #[allow(dead_code)]
    pub fn modp_NTT2_ext(a: *const u32, stride: u64, gm: *const u32, logn: u32, p: u32, p0i: u32);
    #[allow(dead_code)]
    pub fn modp_iNTT2_ext(a: *const u32, stride: u64, igm: *const u32, logn: u32, p: u32, p0i: u32);
    #[allow(dead_code)]
    pub fn modp_poly_rec_res(f: *const u32, logn: u32, p: u32, p0i: u32, R2: u32);
    #[allow(dead_code)]
    pub fn zint_sub(a: *const u32, b: *const u32, len: u64, ctl: u32) -> u32;
    #[allow(dead_code)]
    pub fn zint_mul_small(m: *const u32, mlen: u64, x: u32) -> u32;
    #[allow(dead_code)]
    pub fn zint_mod_small_unsigned(d: *const u32, dlen: u64, p: u32, p0i: u32, R2: u32) -> u32;
    #[allow(dead_code)]
    pub fn zint_mod_small_signed(d: *const u32, dlen: u64, p: u32, p0i: u32, R2: u32) -> u32;
    #[allow(dead_code)]
    pub fn zint_add_mul_small(x: *const u32, y: *const u32, len: u64, s: u32);
    #[allow(dead_code)]
    pub fn zint_norm_zero(x: *const u32, p: *const u32, len: u64);
    #[allow(dead_code)]
    pub fn zint_rebuild_CRT(xx: *const u32, xlen: u64, xstride: u64, num: u64, primes: *const small_prime, normalized_signed: i32, tmp: *const u32);
    #[allow(dead_code)]
    pub fn zint_negate(a: *const u32, len: u64, ctl: u32);
    #[allow(dead_code)]
    pub fn zint_co_reduce(a: *const u32, b: *const u32, len: u64, xa: i64, xb: i64, ya: i64, yb: i64) -> u32;
    #[allow(dead_code)]
    pub fn zint_finish_mod(a: *const u32, len: u64, m: *const u32, neg: u32);
    #[allow(dead_code)]
    pub fn zint_co_reduce_mod(a: *const u32, b: *const u32, m: *const u32, len: u64, m0i: u32, xa: i64, xb: i64, ya: i64, yb: i64);
    #[allow(dead_code)]
    pub fn zint_bezout(u: *const u32, v: *const u32, x: *const u32, y: *const u32, len: u64, tmp: *const u32) -> i32;
    #[allow(dead_code)]
    pub fn zint_add_scaled_mul_small(x: *const u32, xlen: u64, y: *const u32, ylen: u64, k: i32, sch: u32, scl: u32);
    #[allow(dead_code)]
    pub fn zint_sub_scaled(x: *const u32, xlen: u64, y: *const u32, ylen: u64, sch: u32, scl: u32);
    #[allow(dead_code)]
    pub fn zint_one_to_plain(x: *const u32) -> i32;
    #[allow(dead_code)]
    pub fn poly_big_to_fp(d: u64, xlen: u64, f: *const u32, flen: u64, fstride: u64, logn: u32);
    #[allow(dead_code)]
    pub fn poly_big_to_small(d: *const i8, s: *const u32, lim: i32, logn: u32) -> i32;
    #[allow(dead_code)]
    pub fn poly_sub_scaled(F: *const u32, Flen: usize, Fstride: usize, f: *const u32, flen: usize, fstride: usize, k: *const i32, sch: u32, scl: u32, logn: u32);
    #[allow(dead_code)]
    pub fn poly_sub_scaled_ntt(F: *const u32, Flen: usize, Fstride: usize, f: *const u32, flen: usize, fstride: usize, k: *const i32, sch: u32, scl: u32, logn: u32, tmp: *const u32);
    #[allow(dead_code)]
    pub fn get_rng_u64(rng: *const InnerShake256Context) -> u64;
    #[allow(dead_code)]
    pub fn mkgauss(rng: *const InnerShake256Context, logn: u32) -> i32;
    #[allow(dead_code)]
    pub fn poly_small_sqnorm(f: *const i8, logn: u32) -> u32;
    #[allow(dead_code)]
    pub fn align_fpr(base: *const c_void, data: *const c_void) -> *const u64;
    #[allow(dead_code)]
    pub fn align_u32(base: *const c_void, data: *const c_void) -> *const u32;
    #[allow(dead_code)]
    pub fn poly_small_to_fp(x: *const u64, f: *const i8, logn: u32);
    #[allow(dead_code)]
    pub fn make_fg_step(data: *const u32, logn: u32, depth: u32, in_ntt: i32, out_ntt: i32);
    #[allow(dead_code)]
    pub fn make_fg(data: *const u32, f: *const i8, g: *const i8, logn: u32, depth: u32, out_ntt: i32);
    #[allow(dead_code)]
    pub fn solve_NTRU_deepest(logn_top: u32, f: *const i8, g: *const i8, tmp: *const u32) -> i32;
    #[allow(dead_code)]
    pub fn solve_NTRU_intermediate(logn_top: u32, f: *const i8, g: *const i8, depth: u32, tmp: *const u32) -> i32;
    #[allow(dead_code)]
    pub fn solve_NTRU_binary_depth1(logn_top: u32, f: *const i8, g: *const i8, tmp: *const u32) -> i32;
    #[allow(dead_code)]
    pub fn solve_NTRU_binary_depth0(logn_top: u32, f: *const i8, g: *const i8, tmp: *const u32) -> i32;
    #[allow(dead_code)]
    pub fn solve_NTRU(logn_top: u32, F: *const i8, G: *const i8, f: *const i8, g: *const i8, lim: i32, tmp: *const u32) -> i32;
    #[allow(dead_code)]
    pub fn poly_small_mkgauss(rng: *const InnerShake256Context, f: *const i8, logn: u32);
    #[allow(dead_code)]
    pub fn keygen(rng: *const InnerShake256Context, f: *const i8, g: *const i8, F: *const i8, G: *const i8, h: *const u16, logn: u32, tmp: *const u8);
}

#[repr(C)]
pub struct small_prime {
    pub p: u32,
    pub g: u32,
    pub s: u32,
}