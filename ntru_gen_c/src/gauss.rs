#![allow(non_snake_case)]

use crate::prng::NtruPrngChacha8ContextC;

#[link(name = "ng_gauss", kind = "static")]
extern "C" {
    pub fn ntrugen_gauss_sample_poly(logn: u32, f: *const i8, tab: *const u16, rng: rng_f, ctx: &NtruPrngChacha8ContextC);
}

#[allow(non_camel_case_types)]
type rng_f = unsafe extern fn(*const NtruPrngChacha8ContextC, *const u8, usize);


