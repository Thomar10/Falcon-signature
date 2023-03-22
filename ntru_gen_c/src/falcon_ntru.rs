#![allow(non_snake_case)]

use crate::prng::NtruPrngChacha8ContextC;

#[link(name = "ng_falcon", kind = "static")]
extern "C" {
    pub fn ntrugen_Falcon_keygen(logn: u32, f: *const i8, g: *const i8, F: *const i8, G: *const i8, rng: &mut NtruPrngChacha8ContextC, tmp: *const u32) -> i32;
}

