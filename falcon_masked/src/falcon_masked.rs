use falcon::{falcon_sig_ct_size, falcon_sig_padded_size, falcon_tmpsize_signtree};
use falcon::codec::{comp_encode, max_sig_bits, trim_i16_encode};
use falcon::common::{hash_to_point_ct, hash_to_point_vartime};
use falcon::falcon::{FALCON_SIG_CT, fpr, shake256_extract, shake256_flip, shake256_init, shake256_inject};
use falcon::shake::InnerShake256Context;

use crate::sign_masked::sign_tree;
use crate::sign_masked_mask_sample::sign_tree_sample;

pub fn falcon_sign_tree_masked<const ORDER: usize, const LOGN: usize>(mut rng: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize,
                                                                      signature_type: i32, expanded_key: &[u8],
                                                                      data: &[u8]) -> (i32, usize) {
    let mut hd: InnerShake256Context = InnerShake256Context {
        st: [0; 25],
        dptr: 0,
    };
    let mut nonce = [0u8; 40];
    falcon_sign_start(rng, &mut nonce, &mut hd);
    shake256_inject(&mut hd, data);
    falcon_sign_tree_finish::<ORDER, LOGN>(&mut rng, signature, signature_len, signature_type, expanded_key,
                                           &mut hd, &mut nonce)
}

pub fn falcon_sign_tree_masked_sample<const ORDER: usize, const LOGN: usize>(mut rng: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize,
                                                                      signature_type: i32, expanded_key: &[u8],
                                                                      data: &[u8]) -> (i32, usize) {
    let mut hd: InnerShake256Context = InnerShake256Context {
        st: [0; 25],
        dptr: 0,
    };
    let mut nonce = [0u8; 40];
    falcon_sign_start(rng, &mut nonce, &mut hd);
    shake256_inject(&mut hd, data);
    falcon_sign_tree_finish_sample::<ORDER, LOGN>(&mut rng, signature, signature_len, signature_type, expanded_key,
                                           &mut hd, &mut nonce)
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
                                                                  nonce: &mut [u8]) -> (i32, usize) {

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
        let mut masked_expand = mask_expanded_key::<ORDER>(&expanded_key);
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
                                                                  nonce: &mut [u8]) -> (i32, usize) {

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
        let mut masked_expand = mask_expanded_key::<ORDER>(&expanded_key);
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

fn mask_expanded_key<const ORDER: usize>(key: &[u8]) -> Vec<[fpr; ORDER]> {
    let (_, expkey) = key.split_at(8); // alignment
    let expkey = bytemuck::cast_slice(expkey);
    let mut mkey: Vec<[fpr; ORDER]> = vec!([0; ORDER]; expkey.len());
    for i in 0..expkey.len() {
        let mut mask: [fpr; ORDER] = [0; ORDER];
        mask[0] = expkey[i];
        mkey[i] = mask;
    }
    mkey
}