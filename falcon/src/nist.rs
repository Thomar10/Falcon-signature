use std::mem;
use std::ptr::{null, null_mut, slice_from_raw_parts_mut};
use crate::codec::{max_fg_bits, trim_i8_encode};
use crate::falcon_c::nist_c::randombytes_func;
use crate::falcon_c::shake_c::{falcon_inner_i_shake256_flip, falcon_inner_i_shake256_init, falcon_inner_i_shake256_inject, falcon_inner_i_shake256_inject2, InnerShake256Context as InnerShake256ContextC, St as StC};
use crate::{keygen, randombytes2};
use crate::falcon_c::codec_c::falcon_inner_trim_i8_encode;
use crate::shake::{i_shake256_flip, i_shake256_init, i_shake256_inject, InnerShake256Context, St};

pub fn crypto_sign_keypair(mut pk: &mut [u16], mut sk: &mut [u16]) -> bool {
    let mut tmp: [u8; 14336] = [0; 14336];
    let mut f: [i8; 512] = [0; 512];
    let mut g: [i8; 512] = [0; 512];
    let mut F: [i8; 512] = [0; 512];
    let mut h: [u16; 512] = [0; 512];
    let mut seed: [u8; 48] = [0; 48];
    let mut rng = InnerShake256Context {
        st: St { a: [0u64; 25] },
        dptr: 0
    };
    unsafe { randombytes2(seed.as_ptr(), 48); }
    i_shake256_init(&mut rng);
    i_shake256_inject(&mut rng, &mut seed);
    i_shake256_flip(&mut rng);

    keygen(&mut rng, f.as_mut_ptr(), g.as_mut_ptr(), F.as_mut_ptr(), null_mut(), h.as_mut_ptr(), 9, tmp.as_mut_ptr());
    println!("{:?}", f);
    let crypto_secretkey_bytes = 1281;
    sk[0] = 0x50 + 9;
    let mut u = 1;
    let mut v = unsafe {
        let out = sk.as_ptr().add(u);
        falcon_inner_trim_i8_encode(out, crypto_secretkey_bytes - u,
                                    f.as_ptr(), 9, max_fg_bits[9] as u32)
    };
    println!("v {}", v);
    return false;
    //let mut v = trim_i8_encode(&mut sk, u, crypto_secretkey_bytes - u,
    //                          &mut f, 9, max_fg_bits[9] as u32);
    if v == 0 {
        return false;
    }
    /*
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
        }*/
    false
}