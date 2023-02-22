#![allow(dead_code)]

use std::ffi::c_char;

#[link(name = "testfalcon", kind = "static")]
// Purely to access the SHA and 'nist_random' because of laziness.
extern "C" {
    pub fn sha1_init(sc: *const Sha1Context);
    pub fn nist_randombytes_init(entropy_input: *const u8);
    pub fn nist_randombytes(buf: *const u8, len: usize);
    pub fn sha1_print_line(sc: *const Sha1Context, s:  *const c_char);
    pub fn sha1_print_line_with_int(sc: *const Sha1Context, s: *const c_char, x: u32);
    pub fn sha1_print_line_with_hex(sc: *const Sha1Context, s: *const c_char, data: *const (), len: usize);
    pub fn sha1_out(sc: *const Sha1Context, dst: *const u8);
    pub fn save_drbg_state(state: *const u8);
    pub fn restore_drbg_state(state: *const u8);
}

#[repr(C)]
pub struct Sha1Context {
    pub buf: [u8; 64],
    pub val: [u32; 5],
    pub count: u64,
}
