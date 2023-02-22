use std::ptr::null_mut;
use crate::codec::{comp_decode, max_fg_bits, max_FG_bits, modq_decode, modq_encode, trim_i8_encode};
use crate::common::hash_to_point_vartime;
use crate::katrng::randombytes;
use crate::keygen::keygen;
use crate::shake::{i_shake256_flip, i_shake256_init, i_shake256_inject, i_shake256_inject_length, InnerShake256Context, St};
use crate::vrfy::{to_ntt_monty, verify_raw};

const NONCE: usize = 40;
#[allow(dead_code)]
const CRYPTO_SECRETKEYBYTES: u32 = 1281;
const CRYPTO_PUBLICKEYBYTES: u32 = 897;

#[allow(non_snake_case)]
pub fn crypto_sign_keypair(mut pk: &mut [u8], mut sk: &mut [u8]) -> bool {
    let mut tmp: [u8; 14336] = [0; 14336];
    let mut f: [i8; 512] = [0; 512];
    let mut g: [i8; 512] = [0; 512];
    let mut F: [i8; 512] = [0; 512];
    let mut h: [u16; 512] = [0; 512];
    let mut seed: [u8; 48] = [0; 48];
    let mut rng = InnerShake256Context {
        st: St { a: [0u64; 25] },
        dptr: 0,
    };
    randombytes(&mut seed);
    i_shake256_init(&mut rng);
    i_shake256_inject(&mut rng, &mut seed);
    i_shake256_flip(&mut rng);

    keygen(&mut rng, f.as_mut_ptr(), g.as_mut_ptr(), F.as_mut_ptr(), null_mut(), h.as_mut_ptr(), 9, tmp.as_mut_ptr());
    let crypto_secretkey_bytes = 1281;
    sk[0] = 0x50 + 9;
    let mut u = 1;
    let mut v = trim_i8_encode(&mut sk, u, crypto_secretkey_bytes - u,
                               &mut f, 9, max_fg_bits[9] as u32);
    if v == 0 {
        return false;
    }
    u += v;
    v = trim_i8_encode(sk, u, crypto_secretkey_bytes - u,
                       &mut g, 9, max_fg_bits[9] as u32);
    if v == 0 {
        return false;
    }

    u += v;
    v = trim_i8_encode(sk, u, crypto_secretkey_bytes - u,
                       &mut F, 9, max_FG_bits[9] as u32);

    if v == 0 {
        return false;
    }
    u += v;
    if u != crypto_secretkey_bytes {
        return false;
    }

    pk[0] = 0x00 + 9;
    let crypto_publickey_bytes = 897;
    v = modq_encode(&mut pk, 1, crypto_publickey_bytes - 1, &mut h, 9);
    if v != crypto_publickey_bytes - 1 {
        return false;
    }
    return true;
}


pub fn crypto_sign_open(msg: &mut [u8], signature: &mut [u8], slen: usize, pk: &mut [u8]) -> (bool, usize) {
    let mut b: [u8; 1024] = [0; 1024];
    let mut h: [u16; 512] = [0; 512];
    let mut hm: [u16; 512] = [0; 512];
    let mut sig: [i16; 512] = [0; 512];
    let mut rng = InnerShake256Context {
        st: St { a: [0u64; 25] },
        dptr: 0,
    };
    if pk[0] != 0x00 + 9 {
        return (false, 0);
    }
    if modq_decode(&mut h, 9, pk, 1, (CRYPTO_PUBLICKEYBYTES - 1) as usize) != (CRYPTO_PUBLICKEYBYTES - 1) as usize {
        return (false, 0);
    }
    to_ntt_monty(&mut h, 9);

    if slen < 2usize + NONCE {
        return (false, 0);
    }
    let sig_len: usize = ((signature[0] as usize) << 8) | signature[1] as usize;
    if sig_len > slen - 2usize - NONCE {
        return (false, 0);
    }
    let msg_len = slen - 2 - NONCE - sig_len;
    let esig_index = 2 + NONCE + msg_len;
    if sig_len < 1 || signature[esig_index] != 0x20 + 9 {
        return (false, 0);
    }
    if comp_decode(&mut sig, 9, signature, esig_index + 1, sig_len - 1) != sig_len - 1 {
        return (false, 0);
    }

    i_shake256_init(&mut rng);
    i_shake256_inject_length(&mut rng, signature, 2, NONCE + msg_len);
    i_shake256_flip(&mut rng);

    hash_to_point_vartime(&mut rng, &mut hm, 9);

    if !verify_raw(&mut hm, &mut sig, &mut h, 9, &mut b) {
        return (false, 0);
    }
    msg.copy_from_slice(&mut signature[2 + NONCE..2 + NONCE + msg_len]);
    (true, msg_len)
}