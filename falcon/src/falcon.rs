use std::ptr::{null, null_mut};
use crate::keygen::keygen;
use crate::shake::{i_shake256_extract, i_shake256_flip, i_shake256_init, i_shake256_inject, InnerShake256Context};
#[macro_export]
macro_rules! falcon_tmpsize_keygen {
    ($arg:expr) => {
        if $arg <= 3 {272} else {(28 << $arg) + (3 << $arg + 7)}
    }
}

#[macro_export]
macro_rules! falcon_privatekey_size {
    ($arg:expr) => {
        if $arg <= 3 {3 << $arg}
        else {((10u - (($arg) >> 1)) << ((logn) - 2)) + (1 << ($arg)) + 1}
    }
}

#[macro_export]
macro_rules! falcon_publickey_size {
    ($arg:expr) => {
        if $arg <= 1 {3 << $arg}
        else {((10u - (($arg) >> 1)) << ((logn) - 2)) + (1 << ($arg))} + 1
    }
}

#[macro_export]
macro_rules! falcon_tmpsize_makepub {
    ($arg:expr) => {(78u << $arg) + 7}
}

#[macro_export]
macro_rules! falcon_tmpsize_signtree {
    ($arg:expr) => {(50u << $arg) + 7}
}

#[macro_export]
macro_rules! falcon_tmpsize_expandprivate {
    ($arg:expr) => {(52u << $arg) + 7}
}

#[macro_export]
macro_rules! falcon_tmpsize_expanded_key_size {
    ($arg:expr) => {((8u * $arg + 40) << $arg) + 8}
}

#[macro_export]
macro_rules! falcon_tmpsize_verify {
    ($arg:expr) => {(8u << $arg) + 1}
}


pub fn shake256_init(rng: &mut InnerShake256Context) {
    i_shake256_init(rng);
}

pub fn shake256_inject(rng: &mut InnerShake256Context, data: &mut [u8]) {
    i_shake256_inject(rng, data);
}

pub fn shake256_flip(rng: &mut InnerShake256Context) {
    i_shake256_flip(rng);
}

// TODO EXTRACT MUTATE INSTEAD?
pub fn shake256_extract(rng: &mut InnerShake256Context, out: &mut [u], len: usize) {
    i_shake256_extract(rng, len);
}

pub fn shake_init_prng_from_seed(rng: &mut InnerShake256Context, seed: &mut [u], _len: usize) {
    i_shake256_init(rng);
    shake256_inject(rng, seed);
}

pub fn shake_init_prng_from_system(rng: &mut InnerShake256Context) -> i32 {
    let mut seed: [u8; 48] = [0; 48];
    // TODO ? get_seed defined in rng for Extra/c/rng
    // if !get_seed(seed, 48) {
    //     return -1;
    // }
    i_shake256_init(rng);
    shake256_inject(rng, &mut seed);
    0
}

// see inner.h for set_fpu_cw for better / more correct implementation.
pub(crate) fn set_fpu_cw(x: u32) -> u32 {
    x
}

pub fn falcon_keygen_make(rng: &mut InnerShake256Context, logn: u32, private_key: &mut [u8],
                          private_len: usize, public_key: &mut [u8], public_len: usize,
                          tmp: &mut [u8], tmp_len: usize) -> i32 {
    if logn < 1 || logn > 10 {
        return -5;
    }
    if private_len < falcon_privatekey_size!(logn)
        || (public_key.len() <= 0 && public_len < falcon_publickey_size!(logn))
        || tmp_len < falcon_tmpsize_keygen!(logn) {
        return -2;
    }
    let n = 1usize << logn;
    let f: *mut u8 = tmp.as_mut_ptr();
    let g = f.wrapping_add(n);
    let F = g.wrapping_add(n);
    // TODO alignment problems maybe?
    let atmp: *mut u8 = F.wrapping_add(n);
    let oldcw = set_fpu_cw(2);
    keygen(rng, f.cast(), g.cast(), F.cast(), null_mut(), null_mut(), logn, atmp);

    // Encode

    0
}

pub fn falcon_make_public(private_key: &mut [u8], private_len: usize,
                          public_key: &mut [u8], public_len: usize,
                          tmp: &mut [u8], tmp_len: usize) -> i32 {
    0
}

pub fn falcon_get_logn(obj: &mut [u8], len: usize) -> i32 {
    0
}

pub fn falcon_sign_dyn(rng: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize,
                       signature_type: i32, private_key: &mut [u8],
                       private_len: usize, public_key: &mut [u8], public_len: usize,
                       tmp: &mut [u8], tmp_len: usize) -> i32 {
    0
}

pub fn falcon_expand_privatekey(expanded_key: &mut [u8], expanded_len: usize,
                                private_key: &mut [u8], private_len: usize,
                                tmp: &mut [u8], tmp_len: usize) -> i32 {
    0
}

pub fn falcon_sign_tree(rng: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize,
                        signature_type: i32, expanded_key: &mut [u8],
                        expanded_len: usize, data: &mut [u8], data_len: usize,
                        tmp: &mut [u8], tmp_len: usize) -> i32 {
    0
}

pub fn falcon_sign_start(rng: &mut InnerShake256Context, nonce: &mut [u8],
                         hash_data: &mut InnerShake256Context) -> i32 {
    0
}

pub fn falcon_sign_dyn_finish(rng: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize,
                              signature_type: i32, private_key: &mut [u8],
                              private_len: usize,
                              hash_data: &mut InnerShake256Context, nonce: &mut [u8],
                              tmp: &mut [u8], tmp_len: usize) -> i32 {
    0
}

pub fn falcon_sign_tree_finish(rng: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize,
                               signature_type: i32, expanded_key: &mut [u8],
                               hash_data: &mut InnerShake256Context,
                               nonce: &mut [u8],
                               tmp: &mut [u8], tmp_len: usize) -> i32 {
    0
}

pub fn falcon_verify(signature: &mut [u8], signature_len: usize, signature_type: i32,
                     public_key: &mut [u8], public_len: usize,
                     data: &mut [u8], data_len: usize,
                     tmp: &mut [u8], tmp_len: usize) -> i32 {
    0
}

pub fn falcon_verify_start(hash_data: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize) -> i32 {
    0
}

pub fn falcon_verify_finish(signature: &mut [u8], signature_len: usize, signature_type: i32,
                            public_key: &mut [u8], public_len: usize,
                            hash_data: &mut InnerShake256Context,
                            tmp: &mut [u8], tmp_len: usize) -> i32 {
    0
}