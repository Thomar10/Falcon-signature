use std::ffi::c_void;
use crate::falcon_c::rng_c::Prng;
use crate::falcon_c::shake_c::InnerShake256Context;

#[link(name = "sign", kind = "static")]

extern "C" {
    #[allow(dead_code)]
    pub fn ffLDL_treesize_func(logn: u32) -> u32;
    #[allow(dead_code)]
    pub fn ffLDL_fft_inner_func(tree: *const fpr, g0: *const fpr, g1: *const fpr, logn: u32, tmp: *const fpr);
    #[allow(dead_code)]
    pub fn ffLDL_fft(tree: *const fpr, g00: *const fpr, g01: *const fpr, g11: *const fpr, logn: u32, tmp: *const fpr);
    #[allow(dead_code)]
    pub fn ffLDL_binary_normalize(tree: *const fpr, orig_logn: u32, logn: u32);
    #[allow(dead_code)]
    pub fn smallints_to_fpr(r: *const fpr, t: *const i8, logn: u32);
    #[allow(dead_code)]
    pub fn falcon_inner_expand_privkey(expanded_key: *const fpr, f: *const i8, g: *const i8, F: *const i8, G: *const i8, logn: u32, tmp: *const u8);
    #[allow(dead_code)]
    pub fn ffSampling_fft_dyntree(samp: samplerZ, samp_ctx: *const c_void, t0: *const fpr, t1: *const fpr, g00: *const fpr, g01: *const fpr, g11: *const fpr, orig_logn: u32, logn: u32, tmp: *const fpr);
    #[allow(dead_code)]
    pub fn ffSampling_fft(samp: samplerZ, samp_ctx: *const c_void, z0: *const fpr, z1: *const fpr, t1: *const fpr, logn: u32, tmp: *const fpr);
    #[allow(dead_code)]
    pub fn do_sign_tree(samp: samplerZ, samp_ctx: *const c_void, s2: *const i16, expanded_key: *const fpr, hm: *const u16, logn: *const u16, tmp: *const fpr) -> i32;
    #[allow(dead_code)]
    pub fn do_sign_dyn(samt: samplerZ, samp_ctx: *const c_void, s2: *const i16, f: *const i8, g: *const i8, F: *const i8, G: *const i8, hm: *const u16, logn: u32, tmp: *const fpr) -> i32;
    #[allow(dead_code)]
    pub fn falcon_inner_gaussian0_sampler(prng: *const Prng) -> i32;
    #[allow(dead_code)]
    pub fn BerExp(prng: *const Prng, x: fpr, ccs: fpr) -> i32;
    #[allow(dead_code)]
    pub fn falcon_inner_sampler(ctx: *const c_void, mu: fpr, isigma: fpr) -> i32;
    #[allow(dead_code)]
    pub fn falcon_inner_sign_tree(sig: *const i16, rng: *const InnerShake256Context, expanded_key: *const fpr, hm: *const u16, logn: u32, tmp: *const u8);
    #[allow(dead_code)]
    pub fn falcon_inner_sign_dyn(sig: *const i16, rng: *const InnerShake256Context, f: *const i8, g: *const i8, F: *const i8, G: *const i8, hm: *const u16, logn: u32, tmp: *const u8);
}

#[allow(non_camel_case_types)]
type fpr = u64;
#[allow(non_camel_case_types)]
type samplerZ = extern fn(*const c_void, fpr, fpr) -> i32;