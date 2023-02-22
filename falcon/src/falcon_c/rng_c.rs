use std::ffi::c_void;
use crate::falcon_c::shake_c::InnerShake256Context;

#[link(name = "rng", kind = "static")]
extern "C" {
    pub fn prng_init(p: *const Prng, src: *const InnerShake256Context);
    pub fn prng_refill(p: *const Prng);
    pub fn prng_get_bytes(p: *const Prng, dst: *const c_void, len: u64);
    pub fn prng_get_u64_func(p: *const Prng) -> u64;
    pub fn prng_get_u8_func(p: *const Prng) -> u8;
}

#[repr(C)]
pub struct Prng {
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