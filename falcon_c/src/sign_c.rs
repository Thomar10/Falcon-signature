#![allow(dead_code)]
#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::rng_c::Prng;
use crate::shake_c::InnerShake256Context;

#[link(name = "sign", kind = "static")]

extern "C" {
    pub fn ffLDL_treesize_func(logn: u32) -> u32;
    pub fn ffLDL_fft_inner_func(tree: *const fpr, g0: *const fpr, g1: *const fpr, logn: u32, tmp: *const fpr);
    pub fn ffLDL_fft_func(tree: *const fpr, g00: *const fpr, g01: *const fpr, g11: *const fpr, logn: u32, tmp: *const fpr);
    pub fn ffLDL_binary_normalize_func(tree: *const fpr, orig_logn: u32, logn: u32);
    pub fn skoff_b00_func(logn: u32) -> usize;
    pub fn skoff_b01_func(logn: u32) -> usize;
    pub fn skoff_b10_func(logn: u32) -> usize;
    pub fn skoff_b11_func(logn: u32) -> usize;
    pub fn skoff_tree_func(logn: u32) -> usize;
    pub fn smallints_to_fpr_func(r: *const fpr, t: *const i8, logn: u32);
    pub fn falcon_inner_expand_privkey(expanded_key: *const fpr, f: *const i8, g: *const i8, F: *const i8, G: *const i8, logn: u32, tmp: *const u8);
    pub fn ffSampling_fft_dyntree_func(samp: samplerZ, samp_ctx: *const c_void, t0: *const fpr, t1: *const fpr, g00: *const fpr, g01: *const fpr, g11: *const fpr, orig_logn: u32, logn: u32, tmp: *const fpr);
    pub fn ffSampling_fft_func(samp: samplerZ, samp_ctx: *const c_void, z0: *const fpr, z1: *const fpr, t1: *const fpr, logn: u32, tmp: *const fpr);
    pub fn do_sign_tree_func(samp: samplerZ, samp_ctx: *const c_void, s2: *const i16, expanded_key: *const fpr, hm: *const u16, logn: *const u16, tmp: *const fpr) -> i32;
    pub fn do_sign_dyn_func(samt: samplerZ, samp_ctx: *const c_void, s2: *const i16, f: *const i8, g: *const i8, F: *const i8, G: *const i8, hm: *const u16, logn: u32, tmp: *const fpr) -> i32;
    pub fn falcon_inner_gaussian0_sampler(prng: *const Prng) -> i32;
    pub fn BerExp_func(prng: *const Prng, x: fpr, ccs: fpr) -> i32;
    pub fn falcon_inner_sampler(ctx: &SamplerContext, mu: fpr, isigma: fpr) -> i32;
    pub fn falcon_inner_sign_tree(sig: *const i16, rng: *const InnerShake256Context, expanded_key: *const fpr, hm: *const u16, logn: u32, tmp: *const u8);
    pub fn falcon_inner_sign_dyn(sig: *const i16, rng: *const InnerShake256Context, f: *const i8, g: *const i8, F: *const i8, G: *const i8, hm: *const u16, logn: u32, tmp: *const u8);
}

#[allow(non_camel_case_types)]
type fpr = u64;
#[allow(non_camel_case_types)]
type samplerZ = unsafe extern fn(&SamplerContext, fpr, fpr) -> i32;

#[repr(C)]
pub struct SamplerContext {
    pub p: Prng,
    pub sigma_min: fpr,
}