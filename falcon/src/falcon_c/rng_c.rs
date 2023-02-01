use std::ffi::c_void;
use crate::falcon_c::shake_c::inner_shake256_context;

#[link(name = "rng", kind = "static")]
extern "C" {
    pub fn prng_init(p: *const prng, src: *const inner_shake256_context);
    pub fn prng_refill(p: *const prng);
    pub fn prng_get_bytes(p: *const prng, dst: *c_void, len: u64);
}

#[repr(C)]
pub struct prng {
    pub buf: Buf,
    pub ptr: u64,
    pub state: State,
    pub typ: i32,
}


#[repr(C)]
pub union Buf {
    pub d: [u8; 512],
    pub dummy_u64: u64,
}

#[repr(C)]
pub union State {
    pub d: [u8; 256],
    pub dummy_u64: u64,
}