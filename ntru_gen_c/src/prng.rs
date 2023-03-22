#![allow(non_snake_case)]

#[link(name = "ng_prng", kind = "static")]
extern "C" {
    pub fn ntrugen_prng_chacha8_init(ctx: *const NtruPrngChacha8ContextC, seed: *const u8, seed_len: usize);
    pub fn ntrugen_prng_chacha8_out(ctx: *const NtruPrngChacha8ContextC, dst: *const u8, len: usize);

}


#[repr(C)]
pub struct NtruPrngChacha8ContextC {
    pub d: [u8; 40],
}
