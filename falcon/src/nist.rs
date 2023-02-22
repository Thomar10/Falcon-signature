use std::ptr::null_mut;

use crate::codec::{comp_decode, comp_encode, max_fg_bits, max_FG_bits, modq_decode, modq_encode, trim_i8_decode, trim_i8_encode};
use crate::common::hash_to_point_vartime;
use crate::katrng::randombytes;
use crate::keygen::keygen;
use crate::shake::{i_shake256_flip, i_shake256_init, i_shake256_inject, i_shake256_inject_length, InnerShake256Context, St};
use crate::sign::sign_dyn;
use crate::vrfy::{complete_private, to_ntt_monty, verify_raw};

const NONCE: usize = 40;


#[allow(non_snake_case)]
pub fn crypto_sign_keypair(mut pk: &mut [u8], mut sk: &mut [u8], logn: usize) -> bool {
    let crypto_secretkeybytes: usize = if logn == 9 { 1281 } else { 2305 };
    let crypto_publickeybytes: usize = if logn == 9 { 897 } else { 1793 };
    let buff_size: usize = if logn == 9 { 512 } else { 1024 };
    let mut tmp: Vec<u8> = vec![0; if logn == 9 { 14336 } else { 28672 }];
    let mut f: Vec<i8> = vec![0; buff_size];
    let mut g: Vec<i8> = vec![0; buff_size];
    let mut F: Vec<i8> = vec![0; buff_size];
    let mut h: Vec<u16> = vec![0; buff_size];
    let mut seed: [u8; 48] = [0; 48];
    let mut rng = InnerShake256Context {
        st: St { a: [0u64; 25] },
        dptr: 0,
    };
    randombytes(&mut seed);
    i_shake256_init(&mut rng);
    i_shake256_inject(&mut rng, &mut seed);
    i_shake256_flip(&mut rng);

    keygen(&mut rng, f.as_mut_ptr(), g.as_mut_ptr(), F.as_mut_ptr(), null_mut(), h.as_mut_ptr(), logn as u32, tmp.as_mut_ptr());
    sk[0] = (0x50 + logn) as u8;
    let mut u = 1;
    let mut v = trim_i8_encode(&mut sk, u, crypto_secretkeybytes - u,
                               f.as_mut_slice(), logn as u32, max_fg_bits[logn] as u32);
    if v == 0 {
        return false;
    }
    u += v;
    v = trim_i8_encode(sk, u, crypto_secretkeybytes - u,
                       g.as_mut_slice(), logn as u32, max_fg_bits[logn] as u32);
    if v == 0 {
        return false;
    }

    u += v;
    v = trim_i8_encode(sk, u, crypto_secretkeybytes - u,
                       F.as_mut_slice(), logn as u32, max_FG_bits[logn] as u32);

    if v == 0 {
        return false;
    }
    u += v;
    if u != crypto_secretkeybytes {
        return false;
    }

    pk[0] = (0x00 + logn) as u8;
    v = modq_encode(&mut pk, 1, crypto_publickeybytes - 1, h.as_mut_slice(), logn as u32);
    if v != crypto_publickeybytes - 1 {
        return false;
    }
    return true;
}

pub fn crypto_sign(mut sm: &mut [u8], mut m: &mut [u8], mlen: usize, mut sk: &mut [u8], logn: usize) -> (bool, usize) {
    let crypto_secretkeybytes: usize = if logn == 9 { 1281 } else { 2305 };
    let crypto_bytes: usize = if logn == 9 { 690 } else { 1330 };
    let mut tmp: Vec<u8> = vec![0; if logn == 9 { 80 * 512 } else { 80 * 1024 }];
    let buff_size: usize = if logn == 9 { 512 } else { 1024 };
    let mut f: Vec<i8> = vec![0; buff_size];
    let mut g: Vec<i8> = vec![0; buff_size];
    let mut F: Vec<i8> = vec![0; buff_size];
    let mut G: Vec<i8> = vec![0; buff_size];
    let mut h: Vec<u16> = vec![0; buff_size];
    let mut sig: Vec<i16> = vec![0; buff_size];
    let mut seed: [u8; 48] = [0; 48];
    let mut nonce: [u8; NONCE] = [0; NONCE];
    let mut esig: Vec<u8> = vec![0; crypto_bytes - 2 - NONCE];

    let mut rng = InnerShake256Context {
        st: St { a: [0u64; 25] },
        dptr: 0,
    };

    if sk[0] != (0x50 + logn) as u8 {
        return (false, 0);
    }
    let mut u = 1;
    let mut v = trim_i8_decode(f.as_mut_slice(), logn as u32, max_fg_bits[logn] as u32, sk, u, crypto_secretkeybytes - u);
    u += v;
    v = trim_i8_decode(g.as_mut_slice(), logn as u32, max_fg_bits[logn] as u32, sk, u, crypto_secretkeybytes - u);
    if v == 0 {
        return (false, 0);
    }
    u += v;
    v = trim_i8_decode(F.as_mut_slice(), logn as u32, max_FG_bits[logn] as u32, sk, u, crypto_secretkeybytes - u);
    if v == 0 {
        return (false, 0);
    }
    u += v;
    if u != crypto_secretkeybytes {
        return (false, 0);
    }
    if !complete_private(G.as_mut_slice(), f.as_mut_slice(), g.as_mut_slice(), F.as_mut_slice(), logn as u32, tmp.as_mut_slice()) {
        return (false, 0);
    }


    randombytes(&mut nonce);
    i_shake256_init(&mut rng);
    i_shake256_inject(&mut rng, &mut nonce);
    i_shake256_inject(&mut rng, &mut m[0..mlen]);
    i_shake256_flip(&mut rng);

    hash_to_point_vartime(&mut rng, &mut h, logn as u32);

    randombytes(&mut seed);
    i_shake256_init(&mut rng);
    i_shake256_inject(&mut rng, &mut seed);
    i_shake256_flip(&mut rng);

    sign_dyn(sig.as_mut_slice(), &mut rng, f.as_mut_slice(), g.as_mut_slice(), F.as_mut_slice(), G.as_mut_slice(), h.as_mut_slice(), logn as u32, tmp.as_mut_slice());

    esig[0] = (0x20 + logn) as u8;
    let esig_len = esig.len() - 1;

    let mut sig_len = comp_encode(&mut esig, 1, esig_len, sig.as_mut_slice(), logn);
    if sig_len == 0 {
        return (false, 0);
    }
    sig_len += 1;
    sm[42..42 + mlen].clone_from_slice(&mut m);
    sm[0] = (sig_len >> 8) as u8;
    sm[1] = sig_len as u8;
    sm[2..42].clone_from_slice(&mut nonce);
    sm[42 + mlen..42 + mlen + sig_len].clone_from_slice(&mut esig[..sig_len]);
    (true, NONCE + mlen + sig_len + 2)
}


pub fn crypto_sign_open(msg: &mut [u8], signature: &mut [u8], slen: usize, pk: &mut [u8], logn: usize) -> (bool, usize) {
    let crypto_publickeybytes: usize = if logn == 9 { 897 } else { 1793 };
    let buff_size: usize = if logn == 9 { 512 } else { 1024 };
    let mut b: Vec<u8> = vec![0; if logn == 9 { 1024 } else { 1024 * 2 }];
    let mut h: Vec<u16> = vec![0; buff_size];
    let mut hm: Vec<u16> = vec![0; buff_size];
    let mut sig: Vec<i16> = vec![0; buff_size];
    let mut rng = InnerShake256Context {
        st: St { a: [0u64; 25] },
        dptr: 0,
    };
    if pk[0] != (0x00 + logn) as u8 {
        return (false, 0);
    }
    if modq_decode(&mut h, logn as u32, pk, 1, crypto_publickeybytes - 1) != crypto_publickeybytes - 1 {
        return (false, 0);
    }
    to_ntt_monty(&mut h, logn as u32);

    if slen < 2usize + NONCE {
        return (false, 0);
    }
    let sig_len: usize = ((signature[0] as usize) << 8) | signature[1] as usize;
    if sig_len > slen - 2usize - NONCE {
        return (false, 0);
    }
    let msg_len = slen - 2 - NONCE - sig_len;
    let esig_index = 2 + NONCE + msg_len;
    if sig_len < 1 || signature[esig_index] != (0x20 + logn) as u8 {
        return (false, 0);
    }
    if comp_decode(sig.as_mut_slice(), logn as u32, signature, esig_index + 1, sig_len - 1) != sig_len - 1 {
        return (false, 0);
    }

    i_shake256_init(&mut rng);
    i_shake256_inject_length(&mut rng, signature, 2, NONCE + msg_len);
    i_shake256_flip(&mut rng);

    hash_to_point_vartime(&mut rng, &mut hm, logn as u32);

    if !verify_raw(hm.as_mut_slice(), sig.as_mut_slice(), h.as_mut_slice(), logn as u32, b.as_mut_slice()) {
        return (false, 0);
    }
    msg.copy_from_slice(&mut signature[2 + NONCE..2 + NONCE + msg_len]);
    (true, msg_len)
}