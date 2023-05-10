#![allow(non_snake_case)]
#![allow(unused)]

use alloc::vec::Vec;

use rand_core::RngCore;

use falcon::{falcon_privatekey_size, falcon_sig_ct_size, falcon_sig_padded_size, falcon_tmpsize_expanded_key_size, falcon_tmpsize_expandprivate, falcon_tmpsize_keygen, falcon_tmpsize_signdyn, falcon_tmpsize_signtree};
use falcon::codec::{comp_encode, max_fg_bits, max_FG_bits, max_sig_bits, trim_i16_encode, trim_i8_decode};
use falcon::common::{hash_to_point_ct, hash_to_point_vartime};
use falcon::falcon::{FALCON_SIG_CT, fpr, shake256_extract, shake256_flip, shake256_init, shake256_inject};
use falcon::fpr::fpr_sub;
use falcon::keygen::keygen;
use falcon::shake::InnerShake256Context;
use falcon::sign::expand_privkey;
use falcon::vrfy::complete_private;
use randomness::random::RngBoth;

use crate::sign_masked::{sign_dyn, sign_tree};
use crate::sign_masked_mask_sample::sign_tree_sample;

pub fn falcon_sign_dyn_masked<const ORDER: usize, const LOGN: usize>(mut rng: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize,
                                                                     signature_type: i32, private_key: &[u8], private_len: usize,
                                                                     data: &[u8], mut rngboth: &mut RngBoth) -> (i32, usize) {
    let mut hd: InnerShake256Context = InnerShake256Context {
        st: [0; 25],
        dptr: 0,
    };
    let mut nonce = [0u8; 40];
    falcon_sign_start(rng, &mut nonce, &mut hd);
    shake256_inject(&mut hd, data);
    falcon_sign_dyn_finish::<ORDER, LOGN>(&mut rng, signature, signature_len, signature_type, private_key, private_len,
                                          &mut hd, &mut nonce, rngboth)
}

#[allow(non_snake_case)]
fn falcon_sign_dyn_finish<const ORDER: usize, const LOGN: usize>(mut rng: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize,
                                                                 signature_type: i32, private_key: &[u8],
                                                                 private_len: usize,
                                                                 mut hash_data: &mut InnerShake256Context, nonce: &mut [u8], mut rngboth: &mut RngBoth) -> (i32, usize) {
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
    let tmp_size: usize = falcon_tmpsize_signdyn!(LOGN);
    let mut tmp = vec![0u8; tmp_size];
    let n: usize = 1 << logn;
    let (f, inter) = bytemuck::cast_slice_mut::<u8, i8>(tmp.as_mut_slice()).split_at_mut(n);
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
        //let (ff, gg, FF, GG) = mask_all_secret_polynomials(&f, &g, &F, &G, logn as u32);
        let mut ff = mask_polynomials(&f, LOGN as u32, &mut rngboth);
        let mut gg = mask_polynomials(&g, LOGN as u32, &mut rngboth);
        let mut FF = mask_polynomials(&F, LOGN as u32, &mut rngboth);
        let mut GG = mask_polynomials(&G, LOGN as u32, &mut rngboth);
        sign_dyn::<ORDER, LOGN>(sv, &mut rng, ff.as_slice(), gg.as_slice(), FF.as_slice(), GG.as_slice(), hm, logn, rngboth);
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

fn mask_all_secret_polynomials<const ORDER: usize>(f: &[i8], g: &[i8], F: &[i8], G: &[i8], logn: u32) -> (Vec<[i8; ORDER]>, Vec<[i8; ORDER]>, Vec<[i8; ORDER]>, Vec<[i8; ORDER]>) {
    let mut rng: InnerShake256Context = InnerShake256Context {
        st: [0; 25],
        dptr: 0,
    };
    shake256_init(&mut rng);
    shake256_flip(&mut rng);
    let n: usize = 1 << logn as usize;
    let mut ff = vec![0; n];
    let mut gg = vec![0; n];
    let mut FF = vec![0; n];
    let mut GG = vec![0; n];
    let mut hh = vec![0; n];
    let tmp_size: usize = falcon_tmpsize_keygen!(logn);
    let mut tmp = vec![0; tmp_size];
    keygen(&mut rng, ff.as_mut_slice(), gg.as_mut_slice(), FF.as_mut_slice(), GG.as_mut_slice(), hh.as_mut_slice(), logn, tmp.as_mut_slice());
    let mut fkey: Vec<[i8; ORDER]> = vec!([0; ORDER]; n);
    let mut gkey: Vec<[i8; ORDER]> = vec!([0; ORDER]; n);
    let mut Fkey: Vec<[i8; ORDER]> = vec!([0; ORDER]; n);
    let mut Gkey: Vec<[i8; ORDER]> = vec!([0; ORDER]; n);
    for i in 0..n {
        let mut fmask: [i8; ORDER] = [0; ORDER];
        let mut gmask: [i8; ORDER] = [0; ORDER];
        let mut Fmask: [i8; ORDER] = [0; ORDER];
        let mut Gmask: [i8; ORDER] = [0; ORDER];
        fmask[0] = ff[i];
        fmask[1] = f[i] - ff[i];
        fkey[i] = fmask;

        gmask[0] = gg[i];
        gmask[1] = g[i] - gg[i];
        gkey[i] = gmask;

        Fmask[0] = FF[i];
        Fmask[1] = F[i] - FF[i];
        Fkey[i] = Fmask;

        Gmask[0] = GG[i];
        Gmask[1] = G[i] - GG[i];
        Gkey[i] = Gmask;
    }
    (fkey, gkey, Fkey, Gkey)
}

fn mask_polynomials<const ORDER: usize>(polynomial: &[i8], logn: u32, rng: &mut RngBoth) -> Vec<[i8; ORDER]> {
    let n: usize = 1 << logn;
    let mut mkey: Vec<[i8; ORDER]> = vec!([0; ORDER]; n);
    for i in 0..n {
        let mut mask: [i8; ORDER] = [0; ORDER];
        let random: i8 = rng.next_u64() as i8;
        mask[1] = random;
        mask[0] = polynomial[i].wrapping_sub(random);
        mkey[i] = mask;
    }
    mkey
}


pub fn falcon_sign_tree_masked<const ORDER: usize, const LOGN: usize>(mut rng: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize,
                                                                      signature_type: i32, expanded_key: &[u8],
                                                                      data: &[u8], mut rngboth: &mut RngBoth) -> (i32, usize) {
    let mut hd: InnerShake256Context = InnerShake256Context {
        st: [0; 25],
        dptr: 0,
    };
    let mut nonce = [0u8; 40];
    falcon_sign_start(rng, &mut nonce, &mut hd);
    shake256_inject(&mut hd, data);
    falcon_sign_tree_finish::<ORDER, LOGN>(&mut rng, signature, signature_len, signature_type, expanded_key,
                                           &mut hd, &mut nonce, rngboth)
}

pub fn falcon_sign_tree_masked_sample<const ORDER: usize, const LOGN: usize>(mut rng: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize,
                                                                             signature_type: i32, expanded_key: &[u8],
                                                                             data: &[u8], mut rngboth: &mut RngBoth) -> (i32, usize) {
    let mut hd: InnerShake256Context = InnerShake256Context {
        st: [0; 25],
        dptr: 0,
    };
    let mut nonce = [0u8; 40];
    falcon_sign_start(rng, &mut nonce, &mut hd);
    shake256_inject(&mut hd, data);
    falcon_sign_tree_finish_sample::<ORDER, LOGN>(&mut rng, signature, signature_len, signature_type, expanded_key,
                                                  &mut hd, &mut nonce, rngboth)
}

fn falcon_sign_start(mut rng: &mut InnerShake256Context, nonce: &mut [u8],
                     mut hash_data: &mut InnerShake256Context) {
    shake256_extract(&mut rng, nonce, nonce.len());
    shake256_init(&mut hash_data);
    shake256_inject(&mut hash_data, nonce);
}

fn falcon_sign_tree_finish_sample<const ORDER: usize, const LOGN: usize>(mut rng: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize,
                                                                         signature_type: i32, expanded_key: &[u8],
                                                                         mut hash_data: &mut InnerShake256Context,
                                                                         nonce: &mut [u8], mut rngboth: &mut RngBoth) -> (i32, usize) {
    let length = 1 << LOGN;
    let logn: u32 = expanded_key[0] as u32;
    if logn != LOGN as u32 {
        return (-6, 0);
    }

    if signature_len < 41 {
        return (-6, 0);
    }
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

    let mut hm = vec![0u16; length];
    let mut atmp = vec![0u8; falcon_tmpsize_signtree!(logn) - length];

    shake256_flip(&mut hash_data);
    loop {
        let mut hash_data_restart: &mut InnerShake256Context =
            &mut InnerShake256Context {
                st: hash_data.st,
                dptr: hash_data.dptr,
            };
        if signature_type == FALCON_SIG_CT {
            hash_to_point_ct(&mut hash_data_restart, hm.as_mut_slice(), logn, atmp.as_mut_slice());
        } else {
            hash_to_point_vartime(&mut hash_data_restart, hm.as_mut_slice(), logn);
        }
        let mut sv = vec![0u16; hm.len()];
        sv.clone_from_slice(hm.as_mut_slice());
        let sv = bytemuck::cast_slice_mut::<u16, i16>(sv.as_mut_slice());
        let mut masked_expand = mask_expanded_key::<ORDER>(&expanded_key, LOGN as u32, rngboth);
        sign_tree_sample::<ORDER, LOGN>(sv, &mut rng, masked_expand.as_mut_slice(), hm.as_mut_slice(), logn);

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

fn falcon_sign_tree_finish<const ORDER: usize, const LOGN: usize>(mut rng: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize,
                                                                  signature_type: i32, expanded_key: &[u8],
                                                                  mut hash_data: &mut InnerShake256Context,
                                                                  nonce: &mut [u8], mut rngboth: &mut RngBoth) -> (i32, usize) {
    let length = 1 << LOGN;
    let logn: u32 = expanded_key[0] as u32;
    if logn != LOGN as u32 {
        return (-6, 0);
    }

    if signature_len < 41 {
        return (-6, 0);
    }
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

    let mut hm = vec![0u16; length];
    let mut atmp = vec![0u8; falcon_tmpsize_signtree!(logn) - length];

    shake256_flip(&mut hash_data);
    loop {
        let mut hash_data_restart: &mut InnerShake256Context =
            &mut InnerShake256Context {
                st: hash_data.st,
                dptr: hash_data.dptr,
            };
        if signature_type == FALCON_SIG_CT {
            hash_to_point_ct(&mut hash_data_restart, hm.as_mut_slice(), logn, atmp.as_mut_slice());
        } else {
            hash_to_point_vartime(&mut hash_data_restart, hm.as_mut_slice(), logn);
        }
        let mut sv = vec![0u16; hm.len()];
        sv.clone_from_slice(hm.as_mut_slice());
        let sv = bytemuck::cast_slice_mut::<u16, i16>(sv.as_mut_slice());
        let mut masked_expand = mask_expanded_key::<ORDER>(&expanded_key, LOGN as u32, &mut rngboth);
        sign_tree::<ORDER, LOGN>(sv, &mut rng, masked_expand.as_mut_slice(), hm.as_mut_slice(), logn);

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

fn random_expanded_key(mut exp_key: &mut [fpr], rng: &mut RngBoth) {
    const LOGN: usize = 10;
    const exp_tmp_len: usize = falcon_tmpsize_expandprivate!(LOGN);

    let mut exp_tmp = [0; exp_tmp_len];

    let mut f: [i8; 1024] = [0; 1024];
    let mut g: [i8; 1024] = [0; 1024];
    let mut F: [i8; 1024] = [0; 1024];
    let mut G: [i8; 1024] = [0; 1024];
    for i in 0..1024 {
        f[i] = rng.next_u32() as i8;
        g[i] = rng.next_u32() as i8;
        F[i] = rng.next_u32() as i8;
        G[i] = rng.next_u32() as i8;
    }

    expand_privkey(&mut exp_key, &f, &g, &F, &G, LOGN as u32, &mut exp_tmp);
}


fn mask_expanded_key<const ORDER: usize>(key: &[u8], logn: u32, mut rng: &mut RngBoth) -> Vec<[fpr; ORDER]> {
    const LOGN: usize = 10;
    const exp_key_len: usize = falcon_tmpsize_expanded_key_size!(LOGN);
    let mut random_key2 = [0u8; exp_key_len];
    let (_, random_key3) = random_key2.split_at_mut(8); // alignment

    let mut random_key = bytemuck::cast_slice_mut(random_key3);
    random_expanded_key(&mut random_key, &mut rng);
    let (_, expkey) = key.split_at(8); // alignment
    let expkey: &[fpr] = bytemuck::cast_slice(expkey);
    let mut mkey: Vec<[fpr; ORDER]> = vec!([0; ORDER]; expkey.len());
    for i in 0..expkey.len() {
        let mut mask: [fpr; ORDER] = [0; ORDER];
        let random_fpr = random_key[i];
        mask[0] = fpr_sub(expkey[i], random_fpr);
        mask[1] = random_fpr;
        mkey[i] = mask;
    }
    mkey
}