use std::ptr::null_mut;
use crate::codec::{max_fg_bits, max_FG_bits, modq_encode, trim_i8_encode};
use crate::falcon_c::nist_c::randombytes_func;
use crate::falcon_c::katrng2::randombytes2;
use crate::keygen::keygen;
use crate::shake::{i_shake256_flip, i_shake256_init, i_shake256_inject, InnerShake256Context, St};

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
    unsafe { randombytes2(seed.as_ptr(), 48); }
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