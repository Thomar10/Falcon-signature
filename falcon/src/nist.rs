use std::ptr::{null, null_mut, slice_from_raw_parts_mut};
use crate::{i_shake256_init, i_shake256_inject, InnerShake256Context, keygen, St};
use crate::codec::{max_fg_bits, trim_i8_encode};
use crate::falcon_c::nist_c::randombytes_func;
use crate::shake::i_shake256_flip;

pub fn crypto_sign_keypair(pk: &mut [u16], sk: &mut [u16]) -> bool {
    let mut tmp: [u8; 14336] = [0; 14336];
    let mut f: [i8; 512] = [0; 512];
    let mut g: [i8; 512] = [0; 512];
    let mut F: [i8; 512] = [0; 512];
    let mut h: [u16; 512] = [0; 512];
    let mut seed: [u16; 48] = [0; 48];
    let mut rng = InnerShake256Context {
        st: St { a: [0u64; 25] },
        dptr: 0,
    };
    unsafe { randombytes_func(seed.as_ptr(), 48) }
    i_shake256_init(&mut rng);
    let seedu8: *mut u8 = seed.as_mut_ptr().cast();
    let seedu8_slice = slice_from_raw_parts_mut(seedu8, seed.len() * 2);
    unsafe { i_shake256_inject(&mut rng, &mut *seedu8_slice); }
    i_shake256_flip(&mut rng);
    keygen(&mut rng, f.as_mut_ptr(), g.as_mut_ptr(), F.as_mut_ptr(), null_mut(), h.as_mut_ptr(), 9, tmp.as_mut_ptr());

    let crypto_secretkey_bytes = 1281;
    sk[0] = 0x50 + 9;
    let mut u = 1;
    let mut v = trim_i8_encode(sk, u, crypto_secretkey_bytes - u,
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
                       &mut F, 9, max_fg_bits[9] as u32);
    if v == 0 {
        return false;
    }
    u += v;
    if u != crypto_secretkey_bytes {
        return false;
    }
    true
}