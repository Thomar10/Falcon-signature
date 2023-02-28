use crate::codec::{comp_decode, comp_encode, max_fg_bits, max_FG_bits, max_sig_bits, modq_decode, modq_encode, trim_i16_decode, trim_i16_encode, trim_i8_decode, trim_i8_encode};
use crate::common::{hash_to_point_ct, hash_to_point_vartime};
use crate::keygen::keygen;
use crate::shake::{i_shake256_extract, i_shake256_flip, i_shake256_init, i_shake256_inject, InnerShake256Context};
use crate::sign::{expand_privkey, sign_dyn, sign_tree};
use crate::vrfy::{complete_private, compute_public, to_ntt_monty, verify_raw};

#[allow(non_camel_case_types)]
pub type fpr = u64;

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

pub fn shake256_inject(rng: &mut InnerShake256Context, data: &[u8]) {
    i_shake256_inject(rng, data);
}

pub fn shake256_flip(rng: &mut InnerShake256Context) {
    i_shake256_flip(rng);
}

pub fn shake256_extract(rng: &mut InnerShake256Context, out: &mut [u8], len: usize) {
    let vec = i_shake256_extract(rng, len);
    out.copy_from_slice(vec.as_slice());
}

pub fn shake_init_prng_from_seed(rng: &mut InnerShake256Context, seed: &mut [u8], _len: usize) {
    i_shake256_init(rng);
    shake256_inject(rng, seed);
}

#[allow(dead_code)]
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
#[allow(dead_code)]
pub(crate) fn set_fpu_cw(x: u32) -> u32 {
    x
}

#[allow(non_snake_case)]
pub fn falcon_keygen_make(mut rng: &mut InnerShake256Context, logn: u32, mut sk: &mut [u8],
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
    let inter = bytemuck::cast_slice_mut::<u8, i8>(tmp);
    let (f, inter) = inter.split_at_mut(n);
    let (g, inter) = inter.split_at_mut(n);
    let (F, atmp) = inter.split_at_mut(n);
    let atmp = bytemuck::cast_slice_mut::<i8, u8>(atmp);

    keygen(&mut rng, f, g, F, &mut [], &mut [], logn as u32, atmp);

    let sk_len = falcon_privatekey_size!(logn) as usize;
    sk[0] = (0x50 + logn) as u8;
    let mut u = 1;
    let mut v = trim_i8_encode(&mut sk, u, sk_len - u,
                               f, logn, max_fg_bits[logn as usize] as u32);
    if v == 0 {
        return -6;
    }
    u += v;
    v = trim_i8_encode(sk, u, sk_len - u,
                       g, logn, max_fg_bits[logn as usize] as u32);
    if v == 0 {
        return -6;
    }

    u += v;
    v = trim_i8_encode(sk, u, sk_len - u,
                       F, logn, max_FG_bits[logn as usize] as u32);

    if v == 0 {
        return -6;
    }
    u += v;
    if u != sk_len {
        return -6;
    }
    if public_key.len() > 0 {
        let inter = bytemuck::cast_slice_mut::<u8, i8>(tmp);
        let (f, inter) = inter.split_at_mut(n);
        let (g, inter) = inter.split_at_mut(n);
        let inter = bytemuck::pod_align_to_mut::<i8, u16>(inter).1;
        let (h, inter) = inter.split_at_mut(n);
        let atmp = bytemuck::cast_slice_mut(inter);
        if !compute_public(h, f, g, logn, atmp) {
            return -6;
        }
        public_key[0] = (0x00 + logn) as u8;
        let pk_len = falcon_publickey_size!(logn) as usize;
        v = modq_encode(public_key, 1, pk_len - 1, h, logn);
        if v != pk_len - 1 {
            return -6;
        }
    }

    0
}

pub fn falcon_get_logn(key: &[u8], len: usize) -> i32 {
    if len == 0 {
        return -1;
    }
    let logn: i32 = (key[0] & 0x0F) as i32;
    if logn < 1 || logn > 10 {
        return -1;
    }
    logn
}

pub fn falcon_make_public(sk: &[u8], private_len: usize,
                          pk: &mut [u8], public_len: usize,
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
    let tmp = bytemuck::cast_slice_mut::<u8, i8>(tmp);
    let (f, inter) = tmp.split_at_mut(n);
    let (g, inter) = inter.split_at_mut(n);
    let mut u = 1;
    let mut v = trim_i8_decode(f, logn as u32, max_fg_bits[logn] as u32,
                               &sk, u, private_len - u);
    if v == 0 {
        return -6;
    }
    u += v;
    v = trim_i8_decode(g, logn as u32, max_fg_bits[logn] as u32,
                       &sk, u, private_len - u);
    if v == 0 {
        return -6;
    }
    let inter = bytemuck::pod_align_to_mut::<i8, u16>(inter).1;
    let (h, inter) = inter.split_at_mut(n);

    let atmp = bytemuck::pod_align_to_mut::<u16, u8>(inter).1;
    if !compute_public(h, f, g, logn as u32, atmp) {
        return -6;
    }


    pk[0] = (0x00 + logn) as u8;
    let pk_len = falcon_publickey_size!(logn) as usize;
    v = modq_encode(pk, 1, pk_len - 1, h, logn as u32);
    if v != pk_len - 1 {
        return -6;
    }
    0
}


pub fn falcon_sign_dyn(mut rng: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize,
                       signature_type: i32, private_key: &[u8],
                       private_len: usize, data: &[u8],
                       tmp: &mut [u8], tmp_len: usize) -> (i32, usize) {
    let mut hd: InnerShake256Context = InnerShake256Context {
        st: [0; 25],
        dptr: 0,
    };
    let mut nonce = [0u8; 40];
    falcon_sign_start(rng, &mut nonce, &mut hd);
    shake256_inject(&mut hd, data);
    falcon_sign_dyn_finish(&mut rng, signature, signature_len, signature_type,
                           private_key, private_len, &mut hd, &mut nonce, tmp, tmp_len)
}

#[allow(non_snake_case)]
pub fn falcon_expand_privatekey(expanded_key: &mut [u8], expanded_len: usize,
                                sk: &[u8], sk_len: usize,
                                tmp: &mut [u8], tmp_len: usize) -> i32 {
    if sk_len == 0 {
        return -6;
    }
    if (sk[0] & 0xF0) != 0x50 {
        return -6;
    }
    let logn: u32 = (sk[0] & 0x0F) as u32;
    if logn < 1 || logn > 10 {
        return -6;
    }
    if sk_len != falcon_privatekey_size!(logn) as usize {
        return -6;
    }
    if expanded_len < falcon_tmpsize_expanded_key_size!(logn) as usize
        || tmp_len < falcon_tmpsize_expandprivate!(logn) {
        return -6;
    }

    let n: usize = 1 << logn;
    let (f, inter) = bytemuck::cast_slice_mut::<u8, i8>(tmp).split_at_mut(n);
    let (g, inter) = inter.split_at_mut(n);
    let (F, inter) = inter.split_at_mut(n);
    let (G, inter) = inter.split_at_mut(n);
    let (_, atmp, _) = bytemuck::pod_align_to_mut::<i8, u64>(inter);
    let atmp = bytemuck::cast_slice_mut::<u64, u8>(atmp);
    let mut u = 1;
    let mut v = trim_i8_decode(f, logn, max_fg_bits[logn as usize] as u32, sk, u, sk_len - u);
    if v == 0 {
        return -6;
    }
    u += v;
    v = trim_i8_decode(g, logn, max_fg_bits[logn as usize] as u32, sk, u, sk_len - u);
    if v == 0 {
        return -6;
    }
    u += v;
    v = trim_i8_decode(F, logn, max_FG_bits[logn as usize] as u32, sk, u, sk_len - u);
    if v == 0 {
        return -6;
    }
    u += v;
    if u != sk_len {
        return -6;
    }
    if !complete_private(G, f, g, F, logn, atmp) {
        return -6;
    }
    expanded_key[0] = logn as u8;

    let (_, expkey) = expanded_key.split_at_mut(8);
    let expkey = bytemuck::cast_slice_mut(expkey);
    let atmp = bytemuck::cast_slice_mut(atmp);
    expand_privkey(expkey, f, g, F, G, logn, atmp);
    0
}

pub fn falcon_sign_tree(mut rng: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize,
                        signature_type: i32, expanded_key: &[u8],
                        data: &[u8],
                        tmp: &mut [u8], tmp_len: usize) -> (i32, usize) {
    let mut hd: InnerShake256Context = InnerShake256Context {
        st: [0; 25],
        dptr: 0,
    };
    let mut nonce = [0u8; 40];
    falcon_sign_start(rng, &mut nonce, &mut hd);
    shake256_inject(&mut hd, data);
    falcon_sign_tree_finish(&mut rng, signature, signature_len, signature_type, expanded_key,
                            &mut hd, &mut nonce, tmp, tmp_len)
}

pub fn falcon_sign_start(mut rng: &mut InnerShake256Context, nonce: &mut [u8],
                         mut hash_data: &mut InnerShake256Context) -> i32 {
    shake256_extract(&mut rng, nonce, nonce.len());
    shake256_init(&mut hash_data);
    shake256_inject(&mut hash_data, nonce);
    0
}


#[allow(non_snake_case)]
pub fn falcon_sign_dyn_finish(mut rng: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize,
                              signature_type: i32, private_key: &[u8],
                              private_len: usize,
                              mut hash_data: &mut InnerShake256Context, nonce: &mut [u8],
                              tmp: &mut [u8], tmp_len: usize) -> (i32, usize) {
    if private_len == 0 {
        return (-6, 0);
    }
    let sk = private_key;
    if (sk[0] & 0xF0) != 0x50 {
        return (-6, 0);
    }
    let logn: u32 = (sk[0] & 0x0F) as u32;
    if logn < 1 || logn > 10 {
        return (-6, 0);
    }
    if private_len != falcon_privatekey_size!(logn) as usize {
        return (-6, 0);
    }
    if tmp_len < falcon_tmpsize_signdyn!(logn) {
        return (-6, 0);
    }
    if signature_len < 41 {
        return (-6, 0);
    }
    match signature_type {
        FALCON_SIG_COMPRESS => {
            // Lul
        }
        FALCON_SIG_PADDED => {
            if signature_len < falcon_sig_padded_size!(logn) {
                return (-6, 0);
            }
        }
        FALCON_SIG_CT => {
            if signature_len < falcon_sig_ct_size!(logn) {
                return (-6, 0);
            }
        }
        _ => { return (-6, 0); }
    }

    let n: usize = 1 << logn;
    let (f, inter) = bytemuck::cast_slice_mut::<u8, i8>(tmp).split_at_mut(n);
    let (g, inter) = inter.split_at_mut(n);
    let (F, inter) = inter.split_at_mut(n);
    let (G, inter) = inter.split_at_mut(n);
    let inter = bytemuck::pod_align_to_mut::<i8, u16>(inter).1;
    let (hm, inter) = inter.split_at_mut(n);
    let atmp: &mut [u8] = bytemuck::cast_slice_mut(bytemuck::pod_align_to_mut::<u16, fpr>(inter).1);
    let mut u = 1;
    let mut v = trim_i8_decode(f, logn, max_fg_bits[logn as usize] as u32, sk, u,
                               private_len - u);
    if v == 0 {
        return (-6, 0);
    }
    u += v;
    v = trim_i8_decode(g, logn, max_fg_bits[logn as usize] as u32, sk, u,
                       private_len - u);
    if v == 0 {
        return (-6, 0);
    }
    u += v;
    v = trim_i8_decode(F, logn, max_FG_bits[logn as usize] as u32, sk, u,
                       private_len - u);
    if v == 0 {
        return (-6, 0);
    }
    u += v;
    if u != private_len {
        return (-6, 0);
    }
    if !complete_private(G, f, g, F, logn, atmp) {
        return (-6, 0);
    }
    shake256_flip(&mut hash_data);

    loop {
        let mut hash_data_restart: &mut InnerShake256Context =
            &mut InnerShake256Context {
                st: hash_data.st,
                dptr: hash_data.dptr,
            };
        if signature_type == FALCON_SIG_CT {
            hash_to_point_ct(&mut hash_data_restart, hm, logn, atmp);
        } else {
            hash_to_point_vartime(&mut hash_data_restart, hm, logn);
        }
        let mut sv = vec![0u16; hm.len()];
        sv.clone_from_slice(hm);
        let sv = bytemuck::cast_slice_mut::<u16, i16>(sv.as_mut_slice());
        sign_dyn(sv, &mut rng, f, g, F, G, hm, logn, atmp);
        signature[1..41].copy_from_slice(nonce);
        let u = 41;
        let mut v: usize;
        match signature_type {
            FALCON_SIG_COMPRESS => {
                signature[0] = 0x30 + logn as u8;
                v = comp_encode(signature, u, signature_len - u, sv, logn as usize);
                if v == 0 {
                    return (-6, 0);
                }
            }
            FALCON_SIG_PADDED => {
                signature[0] = 0x30 + logn as u8;
                let tu = falcon_sig_padded_size!(logn);
                v = comp_encode(signature, u, tu - u, sv, logn as usize);
                if v == 0 {
                    // Signature does not fit loop (idk why).
                    continue;
                }
                if (v + u) < tu {
                    signature[(u + v)..tu].fill(0);
                    v = tu - u;
                }
            }
            FALCON_SIG_CT => {
                signature[0] = 0x50 + logn as u8;
                v = trim_i16_encode(signature, u, signature_len - u, sv, logn, max_sig_bits[logn as usize] as u32);
                if v == 0 {
                    return (-6, 0);
                }
            }
            _ => { return (-6, 0); }
        }
        return (0, u + v);
    }
}

pub fn falcon_sign_tree_finish(mut rng: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize,
                               signature_type: i32, expanded_key: &[u8],
                               mut hash_data: &mut InnerShake256Context,
                               nonce: &mut [u8],
                               tmp: &mut [u8], tmp_len: usize) -> (i32, usize) {
    let logn: u32 = expanded_key[0] as u32;
    let n: usize = 1 << logn;
    if logn < 1 || logn > 10 {
        return (-6, 0);
    }
    if tmp_len < falcon_tmpsize_signtree!(logn) {
        return (-6, 0);
    }
    if signature_len < 41 {
        return (-6, 0);
    }
    let (_, expkey) = expanded_key.split_at(8); // alignment
    let expkey = bytemuck::cast_slice(expkey);
    match signature_type {
        FALCON_SIG_COMPRESS => {}
        FALCON_SIG_PADDED => {
            if signature_len < falcon_sig_padded_size!(logn) {
                return (-6, 0);
            }
        }
        FALCON_SIG_CT => {
            if signature_len < falcon_sig_ct_size!(logn) {
                return (-6, 0);
            }
        }
        _ => { return (-6, 0); }
    }
    let (_, hm, _) = bytemuck::pod_align_to_mut::<u8, u16>(tmp);
    let (hm, atmp) = hm.split_at_mut(n);
    let (_, atmp, _) = bytemuck::pod_align_to_mut::<u16, u64>(atmp);
    let atmp = bytemuck::cast_slice_mut::<u64, u8>(atmp);
    shake256_flip(&mut hash_data);
    loop {
        let mut hash_data_restart: &mut InnerShake256Context =
            &mut InnerShake256Context {
                st: hash_data.st,
                dptr: hash_data.dptr,
            };
        if signature_type == FALCON_SIG_CT {
            hash_to_point_ct(&mut hash_data_restart, hm, logn, atmp);
        } else {
            hash_to_point_vartime(&mut hash_data_restart, hm, logn);
        }
        let mut sv = vec![0u16; hm.len()];
        sv.clone_from_slice(hm);
        let sv = bytemuck::cast_slice_mut::<u16, i16>(sv.as_mut_slice());
        sign_tree(sv, &mut rng, expkey, hm, logn, atmp);

        signature[1..41].copy_from_slice(nonce);
        let u = 41;
        let mut v: usize;
        match signature_type {
            FALCON_SIG_COMPRESS => {
                signature[0] = 0x30 + logn as u8;
                v = comp_encode(signature, u, signature_len - u, sv, logn as usize);
                if v == 0 {
                    return (-6, 0);
                }
            }
            FALCON_SIG_PADDED => {
                signature[0] = 0x30 + logn as u8;
                let tu = falcon_sig_padded_size!(logn);
                v = comp_encode(signature, u, tu - u, sv, logn as usize);
                if v == 0 {
                    // Signature does not fit loop (idk why).
                    continue;
                }
                if (v + u) < tu {
                    signature[(u + v)..tu].fill(0);
                    v = tu - u;
                }
            }
            FALCON_SIG_CT => {
                signature[0] = 0x50 + logn as u8;
                v = trim_i16_encode(signature, u, signature_len - u, sv, logn, max_sig_bits[logn as usize] as u32);
                if v == 0 {
                    return (-6, 0);
                }
            }
            _ => { return (-6, 0); }
        }
        return (0, u + v);
    }
}

pub fn falcon_verify(signature: &[u8], signature_len: usize, signature_type: i32,
                     public_key: &[u8], public_len: usize,
                     data: &mut [u8],
                     tmp: &mut [u8], tmp_len: usize) -> i32 {
    let mut hd: InnerShake256Context = InnerShake256Context {
        st: [0; 25],
        dptr: 0,
    };
    let r = falcon_verify_start(&mut hd, signature, signature_len);
    if r < 0 {
        return r;
    }
    shake256_inject(&mut hd, data);
    falcon_verify_finish(signature, signature_len, signature_type, public_key, public_len, &mut hd, tmp, tmp_len)
}

pub fn falcon_verify_start(mut hash_data: &mut InnerShake256Context, signature: &[u8], signature_len: usize) -> i32 {
    if signature_len < 41 {
        return -6;
    }
    shake256_init(&mut hash_data);
    shake256_inject(&mut hash_data, &signature[1..41]);
    0
}

pub fn falcon_verify_finish(signature: &[u8], signature_len: usize, signature_type: i32,
                            public_key: &[u8], public_len: usize,
                            mut hash_data: &mut InnerShake256Context,
                            tmp: &mut [u8], tmp_len: usize) -> i32 {
    if signature_len < 41 || public_len == 0 {
        return -6;
    }
    if (public_key[0] & 0xF0) != 0x00 {
        return -6;
    }
    let logn: u32 = (public_key[0] & 0x0F) as u32;
    if logn < 1 || logn > 10 {
        return -6;
    }
    if (signature[0] & 0x0F) != logn as u8 {
        return -6;
    }
    let mut ct = 0;
    match signature_type {
        0 => {
            if (signature[0] & 0xF0) == 0x50 {
                if signature_len != falcon_sig_ct_size!(logn) {
                    return -6;
                }
                ct = 1;
            } else if (signature[0] & 0xF0) == 0x30 {
                // DO NATHING
            } else {
                return -6;
            }
        }
        FALCON_SIG_COMPRESS => {
            if (signature[0] & 0xF0) != 0x30 {
                return -6;
            }
        }
        FALCON_SIG_PADDED => {
            if (signature[0] & 0xF0) != 0x30 {
                return -6;
            }
            if signature_len != falcon_sig_padded_size!(logn) {
                return -6;
            }
        }
        FALCON_SIG_CT => {
            if (signature[0] & 0xF0) != 0x50 {
                return -6;
            }
            if signature_len != falcon_sig_ct_size!(logn) {
                return -6;
            }
            ct = 1;
        }
        _ => { return -6; }
    }
    if public_len != falcon_publickey_size!(logn) {
        return -6;
    }
    if tmp_len < falcon_tmpsize_verify!(logn) {
        return -6;
    }

    let n: usize = 1 << logn;
    let inter = bytemuck::pod_align_to_mut::<u8, u16>(tmp).1;
    let (h, inter) = inter.split_at_mut(n);
    let (hm, inter) = inter.split_at_mut(n);
    let inter = bytemuck::cast_slice_mut::<u16, i16>(inter);
    let (sv, inter) = inter.split_at_mut(n);
    let atmp = bytemuck::pod_align_to_mut::<i16, u8>(inter).1;

    if modq_decode(h, logn as u32, public_key, 1, public_len - 1) != public_len - 1 {
        return -6;
    }

    let u = 41;
    let mut v;
    if ct == 1 {
        v = trim_i16_decode(sv, logn, max_sig_bits[logn as usize] as u32, signature, u, signature_len - u);
    } else {
        v = comp_decode(sv, logn, signature, u, signature_len - u);
    }
    if v == 0 {
        return -6;
    }
    if (u + v) != signature_len {
        if (signature_type == 0 && signature_len == falcon_sig_padded_size!(logn)) || signature_type == FALCON_SIG_PADDED {
            while u + v < signature_len {
                if signature[u + v] != 0 {
                    return -6;
                }
                v += 1;
            }
        } else {
            return -6;
        }
    }

    shake256_flip(&mut hash_data);
    if ct == 1 {
        hash_to_point_ct(&mut hash_data, hm, logn, atmp);
    } else {
        hash_to_point_vartime(&mut hash_data, hm, logn);
    }
    to_ntt_monty(h, logn);
    if !verify_raw(hm, sv, h, logn, atmp) {
        return -6;
    }
    0
}