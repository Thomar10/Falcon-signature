use std::ptr::null_mut;
use std::slice::from_raw_parts_mut;

use crate::codec::{max_fg_bits, max_FG_bits, modq_encode, trim_i8_decode, trim_i8_encode};
use crate::keygen::keygen;
use crate::shake::{i_shake256_extract, i_shake256_flip, i_shake256_init, i_shake256_inject, InnerShake256Context};
use crate::vrfy::compute_public;

// type FALCON_SIG_TYPE = i32;
pub const FALCON_SIG_COMPRESS: i32 = 1;
pub const FALCON_SIG_PADDED: i32 = 2;
pub const FALCON_SIG_CT: i32 = 3;

#[macro_export]
macro_rules! falcon_tmpsize_keygen {
    ($arg:expr) => {
        if $arg <= 3 {272 + ((3 << $arg) + 7)}
        else {(28 << $arg) + ((3 << $arg )+ 7)}
    }
}

#[macro_export]
macro_rules! falcon_privatekey_size {
    ($arg:expr) => {
        if $arg <= 3 {(3 << $arg)  + 1}
        else {((10 - (($arg) >> 1)) << (($arg) - 2)) + (1 << ($arg)) + 1}
    }
}

#[macro_export]
macro_rules! falcon_publickey_size {
    ($arg:expr) => {
        if $arg <= 1 {5}
        else {(7 << ($arg - 2))  + 1}
    }
}

#[macro_export]
macro_rules! falcon_tmpsize_makepub {
    ($arg:expr) => {(6 << $arg) + 1}
}

#[macro_export]
macro_rules! falcon_tmpsize_signdyn {
    ($arg:expr) => {(78 << $arg) + 7}
}


#[macro_export]
macro_rules! falcon_tmpsize_signtree {
    ($arg:expr) => {(50 << $arg) + 7}
}

#[macro_export]
macro_rules! falcon_tmpsize_expandprivate {
    ($arg:expr) => {(52 << $arg) + 7}
}

#[macro_export]
macro_rules! falcon_tmpsize_expanded_key_size {
    ($arg:expr) => {((8 * $arg + 40) << $arg) + 8}
}

#[macro_export]
macro_rules! falcon_tmpsize_verify {
    ($arg:expr) => {(8 << $arg) + 1}
}

#[macro_export]
macro_rules! falcon_sig_compressed_maxsize {
    ($arg:expr) => {
        (((11 << $arg) + (101 >> (10 - $arg)) + 7) >> 3) + 41
    }
}

#[macro_export]
macro_rules! falcon_sig_padded_size {
    ($arg:expr) => {
        (44 + 3 * (256 >> (10 - $arg))) + 2 * ( 128 >> (10 - $arg))
        + 3 * (64 >> (10 - $arg)) + 2 * (16 >> (10 - $arg))
        - 2 * (2 >> (10 - $arg)) - 8 * (1 >> (10 - $arg))
    }
}

#[macro_export]
macro_rules! falcon_sig_ct_size {
    ($arg:expr) => {
        if $arg == 3 {
            (3 << ($arg - 1)) + 40
        } else {
            (3 << ($arg - 1)) + 41
        }
    }
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
pub fn shake256_extract(rng: &mut InnerShake256Context, _out: &mut [u8], len: usize) {
    i_shake256_extract(rng, len);
}

pub fn shake_init_prng_from_seed(rng: &mut InnerShake256Context, seed: &mut [u8], _len: usize) {
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

#[allow(non_snake_case)]
pub fn falcon_keygen_make(rng: &mut InnerShake256Context, logn: u32, private_key: &mut [u8],
                          private_len: usize, public_key: &mut [u8], public_len: usize,
                          tmp: &mut [u8], tmp_len: usize) -> i32 {
    if logn < 1 || logn > 10 {
        return -5;
    }
    if private_len < falcon_privatekey_size!(logn) as usize
        || (public_key.len() <= 0 && public_len < falcon_publickey_size!(logn) as usize)
        || tmp_len < falcon_tmpsize_keygen!(logn) {
        return -2;
    }
    let n = 1usize << logn;
    let f: *mut u8 = tmp.as_mut_ptr().cast();
    let mut f_slice;
    unsafe { f_slice = from_raw_parts_mut(f.cast(), n); };
    let g = f.wrapping_add(n);
    let mut g_slice;
    unsafe { g_slice = from_raw_parts_mut(g.cast(), n); };
    let F = g.wrapping_add(n);
    let mut F_slice;
    unsafe { F_slice = from_raw_parts_mut(F.cast(), n); };
    let mut atmp: *mut u8 = F.wrapping_add(n);

    keygen(rng, f.cast(), g.cast(), F.cast(), null_mut(), null_mut(), logn, atmp);

    let mut sk = private_key;
    let sk_len = falcon_privatekey_size!(logn) as usize;
    sk[0] = (0x50 + logn) as u8;
    let mut u = 1;
    let mut v = trim_i8_encode(&mut sk, u, sk_len - u,
                               &mut f_slice, logn, max_fg_bits[logn as usize] as u32);
    if v == 0 {
        return -6;
    }
    u += v;
    v = trim_i8_encode(sk, u, sk_len - u,
                       &mut g_slice, logn, max_fg_bits[logn as usize] as u32);
    if v == 0 {
        return -6;
    }

    u += v;
    v = trim_i8_encode(sk, u, sk_len - u,
                       &mut F_slice, logn, max_FG_bits[logn as usize] as u32);

    if v == 0 {
        return -6;
    }
    u += v;
    if u != sk_len {
        return -6;
    }
    if public_key.len() > 0 {
        let mut h: *mut u16 = g.wrapping_add(n).cast();
        let mut h_slice = unsafe { from_raw_parts_mut(h, n) };
        let mut atmp: *mut u8 = h.wrapping_add(n).cast();
        if !compute_public(h, f.cast(), g.cast(), logn, atmp) {
            return -6;
        }
        let pk = public_key;
        pk[0] = (0x00 + logn) as u8;
        let pk_len = falcon_publickey_size!(logn) as usize;
        v = modq_encode(pk, 1, pk_len - 1, &mut h_slice, logn);
        if v != pk_len - 1 {
            return -6;
        }
    }

    0
}

pub fn falcon_get_logn(key: &mut [u8], len: usize) -> i32 {
    if len == 0 {
        return -1;
    }
    let logn: i32 = (key[0] & 0x0F) as i32;
    if logn < 1 || logn > 10 {
        return -1;
    }
    logn
}

pub fn falcon_make_public(mut sk: &mut [u8], private_len: usize,
                          mut pk: &mut [u8], public_len: usize,
                          tmp: &mut [u8], tmp_len: usize) -> i32 {
    if private_len == 0 {
        return -6;
    }
    if (sk[0] & 0xF0) != 0x50 {
        return -6;
    }
    let logn = (sk[0] & 0x0F) as usize;
    if logn < 1 || logn > 10 {
        return -6;
    }
    if private_len != falcon_privatekey_size!(logn) as usize {
        return -6;
    }
    if public_len < falcon_publickey_size!(logn) || tmp_len < falcon_tmpsize_makepub!(logn) {
        return -6;
    }
    let n: usize = 1 << logn;
    let fp: *mut i8 = tmp.as_mut_ptr().cast();
    let f: &mut [i8] = unsafe { from_raw_parts_mut(fp, n) };
    let gp = fp.wrapping_add(n);
    let g: &mut [i8] = unsafe { from_raw_parts_mut(gp, n) };
    let mut u = 1;
    let mut v = trim_i8_decode(f, logn as u32, max_fg_bits[logn] as u32,
                               &mut sk, u, private_len - u);
    if v == 0 {
        return -6;
    }
    u += v;
    v = trim_i8_decode(g, logn as u32, max_fg_bits[logn] as u32,
                       &mut sk, u, private_len - u);
    if v == 0 {
        return -6;
    }
    let mut h: *mut u16 = gp.wrapping_add(n).cast();
    let mut h_slice = unsafe { from_raw_parts_mut(h, n) };
    let mut atmp: *mut u8 = h.wrapping_add(n).cast();
    if !compute_public(h, fp, gp, logn as u32, atmp) {
        return -7;
    }


    pk[0] = (0x00 + logn) as u8;
    let pk_len = falcon_publickey_size!(logn) as usize;
    v = modq_encode(pk, 1, pk_len - 1, &mut h_slice, logn as u32);
    if v != pk_len - 1 {
        return -8;
    }
    0
}


pub fn falcon_sign_dyn(rng: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize,
                       signature_type: i32, private_key: &mut [u8],
                       private_len: usize, data: &mut [u8], data_len: usize,
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
                        data: &mut [u8], data_len: usize,
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