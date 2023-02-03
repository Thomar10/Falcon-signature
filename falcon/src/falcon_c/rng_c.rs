use std::ffi::c_void;
use crate::falcon_c::shake_c::InnerShake256Context;

#[link(name = "rng", kind = "static")]
extern "C" {
    #[allow(dead_code)]
    pub fn prng_init(p: *const prng, src: *const InnerShake256Context);
    #[allow(dead_code)]
    pub fn prng_refill(p: *const prng);
    #[allow(dead_code)]
    pub fn prng_get_bytes(p: *const prng, dst: *const c_void, len: u64);
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