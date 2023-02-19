#![allow(non_upper_case_globals)]

use std::slice::from_raw_parts_mut;

use crate::{falcon_privatekey_size, falcon_publickey_size, falcon_sig_compressed_maxsize, falcon_sig_ct_size, falcon_sig_padded_size, falcon_tmpsize_expanded_key_size, falcon_tmpsize_expandprivate, falcon_tmpsize_keygen, falcon_tmpsize_makepub, falcon_tmpsize_signdyn, falcon_tmpsize_signtree, falcon_tmpsize_verify};
use crate::codec::{comp_decode, comp_encode, max_fg_bits, max_FG_bits, modq_decode, modq_encode, trim_i16_decode, trim_i16_encode, trim_i8_decode, trim_i8_encode};
use crate::common::hash_to_point_vartime;
use crate::falcon::{falcon_expand_privatekey, falcon_get_logn, falcon_keygen_make, falcon_make_public, FALCON_SIG_COMPRESS, FALCON_SIG_CT, FALCON_SIG_PADDED, falcon_sign_dyn, falcon_sign_tree, falcon_verify, shake_init_prng_from_seed};
use crate::keygen::keygen;
use crate::rng::{Prng, prng_get_u64, prng_get_u8, prng_init, State};
use crate::shake::{i_shake256_extract, i_shake256_flip, i_shake256_init, i_shake256_inject, InnerShake256Context, St};
use crate::vrfy::{complete_private, compute_public, is_invertible, Q, to_ntt_monty, verify_raw, verify_recover};

// TODO REFACTOR INTO BEING ABLE TO RUN ITSELF
pub fn run_falcon_tests() {
    test_shake256();
    test_codec();
    test_vrfy();
    test_rng();
    // test_FP_block();
    // test_poly();
    // test_gaussian0_sampler();
    // test_sampler();
    // test_sign();
    test_keygen();
    test_external_api();
    test_nist_kat(9, "a57400cbaee7109358859a56c735a3cf048a9da2");
    // test_nist_KAT(10, "affdeb3aa83bf9a2039fa9c17d65fd3e3b9828e2");
}

pub(crate) fn test_nist_kat(logn: u32, srefhash: &str) {
    print!("Test NIST KAT {}: ", logn);

    let mut entropy_input = [0u8; 48];
    hash_bytes = hex::decode(srefhash);
    let sk_len = if logn == 9 { 1281 } else { 2305 };
    let pk_len = if logn == 9 { 897 } else { 1793 };
    let over_len = if logn == 9 { 690 } else { 1330 };
    let mut msg: Vec<u8> = vec![0; 3300];
    let mut sk: Vec<u8> = vec![0; sk_len];
    let mut pk: Vec<u8> = vec![0; pk_len];
    let mut sm: Vec<u8> = vec![0; 3300 + over_len];
    let mut tmp: Vec<u8> = vec![0; 84 << logn];
    let mut esk: Vec<u8> = vec![0; ((8 * logn + 40) << logn) as usize];
    for i in 0..48 {
        entropy_input[i] = i as u8;
    }
    let mut rng: InnerShake256Context = InnerShake256Context {
        st: St { a: [0; 25] },
        dptr: 0,
    };
    // nist_randombytes_init(&mut entropy_input);
    for i in 0..100 {
        let mut seed = [0u8; 48];
        let mut seed2 = [0u8; 48];
        let mut nonce = [0u8; 40];
        let mut drbg_sav = [0u8; 48];
        let fp: *mut i8 = tmp.as_mut_ptr().wrapping_add(72 << logn).cast();
        let f: &mut [i8] = unsafe { from_raw_parts_mut(fp, n) };
        let gp: *mut i8 = fp.wrapping_add(n);
        let g: &mut [i8] = unsafe { from_raw_parts_mut(gp, n) };
        let Fp: *mut i8 = fp.wrapping_add(n);
        let F: &mut [i8] = unsafe { from_raw_parts_mut(Fp, n) };
        let Gp: *mut i8 = fp.wrapping_add(n);
        let G: &mut [i8] = unsafe { from_raw_parts_mut(Gp, n) };
        let hp: *mut u16 = Gp.wrapping_add(n).cast();
        let h: &mut [u16] = unsafe { from_raw_parts_mut(hp, n) };
        let hm: *mut u16 = hp.wrapping_add(n);
        let hm: &mut [u16] = unsafe { from_raw_parts_mut(hm, n) };
        let sigp: *mut i16 = hm.wrapping_add(n).cast();
        let sig: &mut [i16] = unsafe { from_raw_parts_mut(sigp, n) };
        let sig2p: *mut i16 = sigp.wrapping_add(n).cast();
        let sig2: &mut [i16] = unsafe { from_raw_parts_mut(sig2p, n) };

        // nist_randombytes(&mut seed, 48);
        let mlen = 33 * (i + 1);
        // nist_randombytes(&mut msg, mlen);
        // Do like in katrng for random bytes
        nist_randombytes_init(&mut seed);

        nist_randombytes(&mut seed2, 48);
        i_shake256_init(&mut rng);
        i_shake256_inject(&mut rng, &mut seed2);
        i_shake256_flip(&mut rng);
        keygen(&mut rng, fp, gp, Fp, Gp, hp, logn, tmp.as_mut_ptr());
        sk[0] = (0x50 + logn) as u8;
        let mut u = 1;
        let mut v = trim_i8_encode(sk.as_mut_slice(), u, sk_len - u,
                                   f, logn, max_fg_bits[logn as usize] as u32);
        if v == 0 {
            panic!("Error encoding sk(f)");
        }
        u += v;
        let mut v = trim_i8_encode(sk.as_mut_slice(), u, sk_len - u,
                                   g, logn, max_fg_bits[logn as usize] as u32);
        if v == 0 {
            panic!("Error encoding sk(g)");
        }
        u += v;
        let mut v = trim_i8_encode(sk.as_mut_slice(), u, sk_len - u,
                                   F, logn, max_FG_bits[logn as usize] as u32);
        if v == 0 {
            panic!("Error encoding sk(F)");
        }
        u += v;
        if u != sk_len {
            panic!("Wrong private key length {}", u);
        }
        pk[0] = (0x00 + logn) as u8;
        v = modq_encode(pk.as_mut_slice(), 1, pk_len - 1, h, logn);
        if 1 + v != pk_len {
            panic!("Wrong public key length {}", u);
        }
        nist_randombytes(&mut nonce, 40);
        i_shake256_init(&mut rng);
        i_shake256_inject(&mut rng, &mut nonce);
        i_shake256_inject(&mut rng, &mut msg);
        i_shake256_flip(&mut rng);
        hash_to_point_vartime(&mut rng, hm, logn);

        nist_randombytes(&mut seed2, 48);
        i_shake256_init(&mut rng);
        i_shake256_inject(&mut rng, &mut seed2);
        i_shake256_flip(&mut rng);

        // sign_dyn(sig, &mut rng, f, g, F, G, hm, logn, tmp.as_mut_slice());
        // expand_privatekey(esk.as_mut_slice(), f,g, F, G, logn, tmp.as_mut_slice());
        i_shake256_init(&mut rng);
        i_shake256_inject(&mut rng, &mut seed2);
        i_shake256_flip(&mut rng);
        // sign_tree(sig2, &mut rng, esk.as_mut_slice(), hm, logn, tmp.as_mut_slice());
        assert_eq!(sig, sig2, "Sign dyn/tree mismatch!");

        to_ntt_monty(h, logn);
        if !verify_raw(hm, sig, h, logn, tmp.as_mut_slice()) {
            panic!("Invalid signature");
        }
        sm[2..42].copy_from_slice(&mut nonce);
        sm[42..42 + mlen].copy_from_slice(&mut msg[0..mlen]);
        sm[42 + mlen] = (0x20 + logn) as u8;
        u = comp_encode(sm.as_mut_slice(), 43 + mlen, over_len - 43, sig, logn as usize);
        if u == 0 {
            panic!("Could not encode signature");
        }
        u += 1;
        smlen = 42 + mlen + u;
        sm[0] = (u >> 8) as u8;
        sm[1] = u as u8;

        //Restore DRBG

        print!(".");
    }
}

pub(crate) fn test_external_api() {
    print!("Test external API: ");
    let mut rng: InnerShake256Context = InnerShake256Context {
        st: St { a: [0; 25] },
        dptr: 0,
    };

    let mut seed = vec![101, 120, 116, 101, 114, 110, 97, 108];
    shake_init_prng_from_seed(&mut rng, &mut seed, 8);
    for logn in 1..=10 {
        test_external_api_inner(logn, &mut rng);
    }
    println!(" done. ");
}

fn test_external_api_inner(logn: u32, mut rng: &mut InnerShake256Context) {
    print!("[{}]", logn);
    let pk_len = falcon_publickey_size!(logn as usize);
    let sk_len = falcon_privatekey_size!(logn as usize);
    let sig_len = falcon_sig_compressed_maxsize!(logn as usize);
    let sigpad_len = falcon_sig_padded_size!(logn as usize);
    let sigct_len = falcon_sig_ct_size!(logn as usize);
    let expkey_len = falcon_tmpsize_expanded_key_size!(logn as usize);

    let mut pk: Vec<u8> = vec![0; pk_len];
    let mut pk2: Vec<u8> = vec![0; pk_len];
    let mut sk: Vec<u8> = vec![0; sk_len];
    let mut sig: Vec<u8> = vec![0; sig_len];
    let mut sigpad: Vec<u8> = vec![0; sigpad_len];
    let mut sigct: Vec<u8> = vec![0; sigct_len];
    let mut expkey: Vec<u8> = vec![0; expkey_len];


    let tmpkg_len = falcon_tmpsize_keygen!(logn);
    let tmpmp_len = falcon_tmpsize_makepub!(logn);
    let tmpsd_len = falcon_tmpsize_signdyn!(logn);
    let tmpst_len = falcon_tmpsize_signtree!(logn);
    let tmpvv_len = falcon_tmpsize_verify!(logn);
    let tmpek_len = falcon_tmpsize_expandprivate!(logn);

    let mut tmpkg: Vec<u8> = vec![0; tmpkg_len];
    let mut tmpmp: Vec<u8> = vec![0; tmpmp_len];
    let mut tmpsd: Vec<u8> = vec![0; tmpsd_len];
    let mut tmpst: Vec<u8> = vec![0; tmpst_len];
    let mut tmpvv: Vec<u8> = vec![0; tmpvv_len];
    let mut tmpek: Vec<u8> = vec![0; tmpek_len];

    for _ in 0..12 {
        pk.fill(0);
        sk.fill(0);
        let mut r = falcon_keygen_make(&mut rng, logn,
                                       sk.as_mut_slice(), sk_len,
                                       pk.as_mut_slice(), pk_len,
                                       tmpkg.as_mut_slice(), tmpkg_len);
        if r != 0 {
            panic!("keygen failed: {}", r);
        }
        pk2.fill(0xFF);
        r = falcon_make_public(sk.as_mut_slice(), sk_len, pk2.as_mut_slice(), pk_len, tmpmp.as_mut_slice(), tmpmp_len);
        if r != 0 {
            panic!("makepub failed: {}", r);
        }
        assert_eq!(pk, pk2, "pub / repub");

        r = falcon_get_logn(pk.as_mut_slice(), pk_len);
        if r != logn as i32 {
            panic!("get_logn failed: {}", r);
        }
        /*
        sig.fill(0);
        let mut data = "data1";
        let data_bytes = unsafe { data.as_bytes_mut() };
        r = falcon_sign_dyn(&mut rng, sig.as_mut_slice(), sig_len,
                            FALCON_SIG_COMPRESS, sk.as_mut_slice(), sk_len,
                            data_bytes, 5, tmpsd.as_mut_slice(), tmpsd_len);
        if r != 0 {
            panic!("sign_dyn failed: {}", r);
        }
        r = falcon_verify(sig.as_mut_slice(), sig_len, FALCON_SIG_COMPRESS, pk.as_mut_slice(),
                          pk_len, data_bytes, 5, tmpvv.as_mut_slice(), tmpvv_len);
        if r != 0 {
            panic!("verify failed: {}", r);
        }
        if logn >= 5 {
            // Skip check for very low degrees as alternate data hashes to a point very close
            // to the correct point so signature matches both.
            let data2 = vec![10, 10, 10, 10, 10].as_mut_slice();
            r = falcon_verify(sig.as_mut_slice(), sig_len, FALCON_SIG_COMPRESS, pk.as_mut_slice(),
                              pk_len, data2, 5, tmpvv.as_mut_slice(), tmpvv_len);
            if r != 6 {
                panic!("wrong verify error: {}", r);
            }
        }

        sigpad.fill(0);
        r = falcon_sign_dyn(&mut rng, sigpad.as_mut_slice(), sigpad_len,
                            FALCON_SIG_PADDED, sk.as_mut_slice(), sk_len,
                            data_bytes, 5, tmpsd.as_mut_slice(), tmpsd_len);
        if r != 0 {
            panic!("sign_dyn(padded) failed: {}", r);
        }
        if sigpad_len != falcon_sig_padded_size!(logn) {
            panic!("sign_dyn(padded): wrong length {}", sigpad_len);
        }
        r = falcon_verify(sigpad.as_mut_slice(), sigpad_len, FALCON_SIG_PADDED, pk.as_mut_slice(),
                          pk_len, data_bytes, 5, tmpvv.as_mut_slice(), tmpvv_len);
        if r != 0 {
            panic!("verify(padded) failed: {}", r);
        }
        if logn >= 5 {
            // Skip check for very low degrees as alternate data hashes to a point very close
            // to the correct point so signature matches both.
            let data2 = vec![10, 10, 10, 10, 10].as_mut_slice();
            r = falcon_verify(sigpad.as_mut_slice(), sigpad_len, FALCON_SIG_PADDED, pk.as_mut_slice(),
                              pk_len, data2, 5, tmpvv.as_mut_slice(), tmpvv_len);
            if r != 6 {
                panic!("wrong verify(padded) error: {}", r);
            }
        }

        sigct.fill(0);
        r = falcon_sign_dyn(&mut rng, sigct.as_mut_slice(), sigct_len,
                            FALCON_SIG_CT, sk.as_mut_slice(), sk_len,
                            data_bytes, 5, tmpsd.as_mut_slice(), tmpsd_len);
        if r != 0 {
            panic!("sign_dyn(ct) failed: {}", r);
        }
        r = falcon_verify(sigct.as_mut_slice(), sigct_len, FALCON_SIG_CT, pk.as_mut_slice(),
                          pk_len, data_bytes, 5, tmpvv.as_mut_slice(), tmpvv_len);
        if r != 0 {
            panic!("verify(ct) failed: {}", r);
        }
        if logn >= 5 {
            // Skip check for very low degrees as alternate data hashes to a point very close
            // to the correct point so signature matches both.
            let data2 = vec![10, 10, 10, 10, 10].as_mut_slice();
            r = falcon_verify(sigct.as_mut_slice(), sigct_len, FALCON_SIG_CT, pk.as_mut_slice(),
                              pk_len, data2, 5, tmpvv.as_mut_slice(), tmpvv_len);
            if r != 6 {
                panic!("wrong verify(ct) error: {}", r);
            }
        }

        r = falcon_expand_privatekey(expkey.as_mut_slice(), expkey_len, sk.as_mut_slice(), sk_len, tmpek.as_mut_slice(), tmpek_len);
        if r != 0 {
            panic!("expand_privatekey failed: {}", r);
        }

        sig.fill(0);
        r = falcon_sign_tree(&mut rng, sig.as_mut_slice(), sig_len, FALCON_SIG_COMPRESS, expkey.as_mut_slice(), data_bytes, 5, tmpst.as_mut_slice(), tmpst_len);
        if r != 0 {
            panic!("sign_tree failed: {}", r);
        }
        r = falcon_verify(sig.as_mut_slice(), sig_len, FALCON_SIG_COMPRESS, pk.as_mut_slice(),
                          pk_len, data_bytes, 5, tmpvv.as_mut_slice(), tmpvv_len);
        if r != 0 {
            panic!("verify2 failed: {}", r);
        }
        if logn >= 5 {
            // Skip check for very low degrees as alternate data hashes to a point very close
            // to the correct point so signature matches both.
            let data2 = vec![10, 10, 10, 10, 10].as_mut_slice();
            r = falcon_verify(sig.as_mut_slice(), sig_len, FALCON_SIG_COMPRESS, pk.as_mut_slice(),
                              pk_len, data2, 5, tmpvv.as_mut_slice(), tmpvv_len);
            if r != 6 {
                panic!("wrong verify(ct) error: {}", r);
            }
        }

        sigpad.fill(0);
        r = falcon_sign_tree(&mut rng, sigpad.as_mut_slice(), sigpad_len, FALCON_SIG_PADDED, expkey.as_mut_slice(), data_bytes, 5, tmpst.as_mut_slice(), tmpst_len);
        if r != 0 {
            panic!("sign_tree(padded) failed: {}", r);
        }
        r = falcon_verify(sigpad.as_mut_slice(), sigpad_len, FALCON_SIG_PADDED, pk.as_mut_slice(),
                          pk_len, data_bytes, 5, tmpvv.as_mut_slice(), tmpvv_len);
        if r != 0 {
            panic!("verify2(padded) failed: {}", r);
        }
        if logn >= 5 {
            // Skip check for very low degrees as alternate data hashes to a point very close
            // to the correct point so signature matches both.
            let data2 = vec![10, 10, 10, 10, 10].as_mut_slice();
            r = falcon_verify(sigpad.as_mut_slice(), sigpad_len, FALCON_SIG_PADDED, pk.as_mut_slice(),
                              pk_len, data2, 5, tmpvv.as_mut_slice(), tmpvv_len);
            if r != 6 {
                panic!("wrong verify(padded) error: {}", r);
            }
        }

        sigct.fill(0);
        r = falcon_sign_tree(&mut rng, sigct.as_mut_slice(), sigct_len, FALCON_SIG_CT, expkey.as_mut_slice(), data_bytes, 5, tmpst.as_mut_slice(), tmpst_len);
        if r != 0 {
            panic!("sign_tree(ct) failed: {}", r);
        }
        r = falcon_verify(sigct.as_mut_slice(), sigct_len, FALCON_SIG_CT, pk.as_mut_slice(),
                          pk_len, data_bytes, 5, tmpvv.as_mut_slice(), tmpvv_len);
        if r != 0 {
            panic!("verify2(ct) failed: {}", r);
        }
        if logn >= 5 {
            // Skip check for very low degrees as alternate data hashes to a point very close
            // to the correct point so signature matches both.
            let data2 = vec![10, 10, 10, 10, 10].as_mut_slice();
            r = falcon_verify(sigct.as_mut_slice(), sigct_len, FALCON_SIG_CT, pk.as_mut_slice(),
                              pk_len, data2, 5, tmpvv.as_mut_slice(), tmpvv_len);
            if r != 6 {
                panic!("wrong verify(ct) error: {}", r);
            }
        } */
        print!(".");
    }
}

pub(crate) fn test_keygen() {
    print!("Test keygen: ");
    const TLEN: usize = 90112;
    let mut tmp: [u8; TLEN] = [0; TLEN];
    for logn in 1..=10 {
        test_keygen_inner(logn, &mut tmp);
    }
    println!(" done. ");
}

fn test_keygen_inner(logn: u32, tmp: &mut [u8]) {
    let mut rng: InnerShake256Context = InnerShake256Context {
        st: St { a: [0; 25] },
        dptr: 0,
    };
    print!("[{}]", logn);
    let mut string = String::from("keygen 0");
    let mut buf: &mut [u8] = unsafe {
        string.as_bytes_mut()
    };
    buf[7] = "0".as_bytes()[0] + logn as u8;

    i_shake256_init(&mut rng);
    i_shake256_inject(&mut rng, buf);
    i_shake256_flip(&mut rng);
    let n: usize = 1 << logn;
    let fp: *mut i8 = tmp.as_mut_ptr().cast();
    let f: &mut [i8] = unsafe { from_raw_parts_mut(fp, n) };
    let gp = fp.wrapping_add(n);
    let g: &mut [i8] = unsafe { from_raw_parts_mut(gp, n) };
    let Fp = gp.wrapping_add(n);
    let F: &mut [i8] = unsafe { from_raw_parts_mut(Fp, n) };
    let Gp = Fp.wrapping_add(n);
    let G: &mut [i8] = unsafe { from_raw_parts_mut(Gp, n) };
    let hp: *mut u16 = Gp.wrapping_add(n).cast();
    let h: &mut [u16] = unsafe { from_raw_parts_mut(hp, n) };
    let h2p = hp.wrapping_add(n);
    let h2: &mut [u16] = unsafe { from_raw_parts_mut(h2p, n) };
    let hmp = h2p.wrapping_add(n);
    let hm: &mut [u16] = unsafe { from_raw_parts_mut(hmp, n) };
    let sigp: *mut i16 = hmp.wrapping_add(n).cast();
    let sig: &mut [i16] = unsafe { from_raw_parts_mut(sigp, n) };
    let s1p = sigp.wrapping_add(n);
    let s1: &mut [i16] = unsafe { from_raw_parts_mut(s1p, n) };
    let mut ttp: *mut u8 = s1p.wrapping_add(n).cast();
    let tt: &mut [u8];
    if logn == 1 {
        ttp = ttp.wrapping_add(4);
        tt = unsafe { from_raw_parts_mut(ttp, n + 4) };
    } else {
        tt = unsafe { from_raw_parts_mut(ttp, n) };
    }
    for _ in 0..12 {
        let mut sc: InnerShake256Context = InnerShake256Context {
            st: St { a: [0; 25] },
            dptr: 0,
        };
        keygen(&mut rng, fp, gp, Fp, Gp, hp, logn, ttp);
        let msg = i_shake256_extract(&mut rng, 50);

        i_shake256_init(&mut sc);
        i_shake256_inject(&mut sc, msg.as_slice());
        i_shake256_flip(&mut sc);
        unsafe { hash_to_point_vartime(&mut sc, hm, logn); };

        // TODO IMPLEMENT WHEN SIGNATURES IS MADE
        /*
        //sign_dyn
        //memcpy lul
        while !is_invertible(sig, logn, tt) {
            // sign_dyn
            //memcpy (as above, so its a do-while loop
        }
        to_ntt_monty(h, logn);
        if !verify_raw(hm, sig, h, logn, tt) {
            panic!("Self signature not verified");
        }
        if verify_recover(h2, hm, s1, sig, logn, tt) {
            panic!("Self signature recovery failed");
        }
        to_ntt_monty(h2, logn);
        assert_eq!(h, h2, "Recovered public key");
        */
        print!(".");
    }
}


pub(crate) fn test_rng() {
    print!("Test RNG: ");
    let mut rng: InnerShake256Context = InnerShake256Context {
        st: St { a: [0; 25] },
        dptr: 0,
    };
    let mut prng: Prng = Prng {
        buf: [0; 512],
        ptr: 0,
        state: State {
            d: [0; 256]
        },
        typ: 0,
    };
    i_shake256_init(&mut rng);
    i_shake256_inject(&mut rng, "rng".as_bytes());
    i_shake256_flip(&mut rng);
    prng_init(&mut prng, &mut rng);
    println!("{:?}", prng.buf);
    for u in 0..KAT_RNG_1.len() {
        let value = prng_get_u64(&mut prng);
        if KAT_RNG_1[u] != value {
            println!("Fix this!");
            break;
            panic!("Error KAT_RNG_1({} != {})", KAT_RNG_1[u], value);
        }
    }
    for u in 0..KAT_RNG_2.len() {
        if KAT_RNG_2[u] != prng_get_u8(&mut prng) {
            println!("Wrongly init of prng?");
            break;
            panic!("Error KAT_RNG_2({})", u);
        }
    }
    println!(" done.");
}

pub(crate) fn test_vrfy() {
    print!("Test verify: ");

    const TLEN: usize = 8192;
    let mut tmp: [u8; TLEN] = [0; TLEN];
    test_vrfy_inner(4, &mut ntru_f_16, &mut ntru_g_16, &mut ntru_F_16, &mut ntru_G_16,
                    &mut ntru_h_16, ntru_pkey_16, &KAT_SIG_16, &mut tmp, TLEN);
    test_vrfy_inner(9, &mut ntru_f_512, &mut ntru_g_512, &mut ntru_F_512, &mut ntru_G_512,
                    &mut ntru_h_512, ntru_pkey_512, &KAT_SIG_512, &mut tmp, TLEN);
    test_vrfy_inner(10, &mut ntru_f_1024, &mut ntru_g_1024, &mut ntru_F_1024, &mut ntru_G_1024,
                    &mut ntru_h_1024, ntru_pkey_1024, &KAT_SIG_1024, &mut tmp, TLEN);

    println!(" done.");
}

// TODO Refactor to actually only use the tmp instead of the f, g, G, F, h values (removes warnings)
#[allow(non_snake_case)]
fn test_vrfy_inner(logn: u32, mut f: &mut [i8], mut g: &mut [i8],
                   mut F: &mut [i8], G: &mut [i8], mut h: &mut [u16],
                   hexpubkey: &str, kat: &[&str], tmp: &mut [u8], tlen: usize) {
    let n: usize = 1 << logn;
    let h2p: *mut u16 = tmp.as_mut_ptr().cast();
    let h2: &mut [u16];
    unsafe { h2 = from_raw_parts_mut(h2p, n); }
    if tlen < 4 * n {
        panic!("Insufficient buffer size");
    }
    if !compute_public(h2p, f.as_mut_ptr(), g.as_mut_ptr(), logn, h2p.wrapping_add(n).cast()) {
        panic!("Compute public failed!");
    }
    assert_eq!(h, h2, "compute_public");

    let G2p: *mut i8 = tmp.as_mut_ptr().cast();
    let G2: &mut [i8];
    unsafe { G2 = from_raw_parts_mut(G2p, n); }
    if tlen < 5 * n {
        panic!("Insufficient buffer size");
    }
    let g2_tmp: &mut [u8];
    unsafe { g2_tmp = from_raw_parts_mut(G2p.wrapping_add(n).cast(), tmp.len() - n); };
    if !complete_private(G2, &mut f, &mut g, &mut F, logn, g2_tmp) {
        panic!("Compute public failed!");
    }
    assert_eq!(G, G2, "complete_private");

    // Test encoding of pk.
    let pubkey_bytes = hex::decode(hexpubkey).unwrap();
    if pubkey_bytes.len() != 1 + (((n * 14) + 7) >> 3) {
        panic!("Unexpected public key length {}", pubkey_bytes.len());
    }
    if pubkey_bytes[0] != logn as u8 {
        panic!("Unexpected first byte in public key {}", pubkey_bytes[0]);
    }
    if tlen < 2 * (pubkey_bytes.len() - 1) {
        panic!("Insufficient buffer size");
    }
    tmp[0] = logn as u8;
    let len2 = modq_encode(tmp, 1, tlen - pubkey_bytes.len() - 1, &mut h, logn);
    if len2 != (pubkey_bytes.len() - 1) {
        panic!("Wrong encoded public key length {}", len2);
    }
    assert_eq!(pubkey_bytes, tmp[0..len2 + 1], "Public key encoded");


    if tlen < 8 * n {
        panic!("Insufficient buffer size");
    }
    for u in (0..kat.len()).step_by(3) {
        let mut rng: InnerShake256Context = InnerShake256Context {
            st: St { a: [0; 25] },
            dptr: 0,
        };
        let nonce = hex::decode(kat[u]).unwrap();
        i_shake256_init(&mut rng);
        i_shake256_inject(&mut rng, nonce.as_slice());
        i_shake256_inject(&mut rng, kat[u + 1].as_bytes());
        i_shake256_flip(&mut rng);

        let mut signature = hex::decode(kat[u + 2]).unwrap();
        if signature.len() == 0 || signature[0] != logn as u8 {
            panic!("Invalid sig KAT");
        }

        let len1 = signature.len() - 1;
        let sig2p: *mut i16 = tmp.as_mut_ptr().cast();
        let mut sig2: &mut [i16];
        unsafe { sig2 = from_raw_parts_mut(sig2p, len1 / 2); }
        sig2[0] = logn as i16;
        let len2 = trim_i16_decode(&mut sig2, logn, 16, signature.as_mut_slice(), 1, len1);
        if len2 != len1 {
            panic!("Invalid sig KAT {} != {}", len2, len1);
        }

        unsafe { tmp.as_mut_ptr().copy_from(sig2.as_mut_ptr().cast(), 2 * n); }
        let s2p: *mut i16 = tmp.as_mut_ptr().cast();
        let h2p: *mut u16 = s2p.wrapping_add(n).cast();
        unsafe { h2p.copy_from(h.as_mut_ptr().cast(), n); }
        let mut h2: &mut [u16];
        unsafe { h2 = from_raw_parts_mut(h2p.cast(), n); }
        to_ntt_monty(&mut h2, logn);

        let c0p = h2p.wrapping_add(2 * n);
        let c0: &mut [u16];
        unsafe { c0 = from_raw_parts_mut(c0p.cast(), n); }
        hash_to_point_vartime(&mut rng, c0, logn);

        let raw_tmp;
        unsafe { raw_tmp = from_raw_parts_mut(c0p.wrapping_add(2 * n).cast(), n); }
        if !verify_raw(c0, sig2, h2, logn, raw_tmp) {
            panic!("KAT signature failed");
        }
        print!(".");
    }
    print!(" ");
}

pub(crate) fn test_codec() {
    print!("Test encode/decode: ");

    const TLEN: usize = 8192;
    let mut tmp: [u8; TLEN] = [0; TLEN];
    for logn in 1..=10 {
        test_codec_inner(logn, &mut tmp, TLEN);
        print!(".");
    }
    println!(" done.");
}

fn test_codec_inner(logn: u32, tmp: &mut [u8], tlen: usize) {
    let n: usize = 1 << logn;
    let mut rng: InnerShake256Context = InnerShake256Context {
        st: St { a: [0; 25] },
        dptr: 0,
    };
    i_shake256_init(&mut rng);
    i_shake256_inject(&mut rng, "codec".as_bytes());
    i_shake256_inject(&mut rng, &mut [logn as u8; 1]);
    i_shake256_flip(&mut rng);
    for _ in 0..10 {
        let m1p: *mut u16 = tmp.as_mut_ptr().cast();
        let mut m1: &mut [u16];
        unsafe { m1 = from_raw_parts_mut(m1p, n); }
        let m2p = m1p.wrapping_add(n);
        let mut m2: &mut [u16];
        unsafe { m2 = from_raw_parts_mut(m2p, n); }

        let mut maxlen = tlen - 4 * n;

        for u in 0..n {
            let w: u32;
            let vec = i_shake256_extract(&mut rng, 4);
            let extract = vec.as_slice();
            w = extract[0] as u32
                | ((extract[1] as u32) << 8)
                | ((extract[2] as u32) << 16)
                | ((extract[3] as u32) << 24);
            m1[u] = (w % Q) as u16;
        }
        let mut len1 = modq_encode(&mut [], 0, 0, &mut m1, logn);
        if len1 != (((n * 14) + 7) >> 3) {
            panic!("Error modq encode(0): {}", len1);
        }
        len1 = modq_encode(tmp, 4 * n, maxlen, &mut m1, logn);
        if len1 != (((n * 14) + 7) >> 3) {
            panic!("Error modq encode: {}", len1);
        }
        let mut len2 = modq_decode(&mut m2, logn, tmp, 4 * n, len1);
        if len2 != len1 {
            panic!("Error modq decode: {}", len2);
        }
        assert_eq!(m1, m2, "modq encode/decode");

        let s1p: *mut i16 = tmp.as_mut_ptr().cast();
        let mut s1: &mut [i16];
        unsafe { s1 = from_raw_parts_mut(s1p, n); }
        let s2p = s1p.wrapping_add(n);
        let s2: &mut [i16];
        unsafe { s2 = from_raw_parts_mut(s2p, n); }
        maxlen = tlen - 4 * n;
        for bits in 4..=12 {
            let mask1: u32 = 1 << (bits - 1);
            let mask2: u32 = mask1 - 1;

            for u in 0..n {
                let vec = i_shake256_extract(&mut rng, 2);
                let extracted = vec.as_slice();
                let w: u32 = (extracted[0] as u32) | ((extracted[1] as u32) << 8);
                let a = w & mask2;
                s1[u] = if (w & mask1) != 0 { (-(a as i32)) as i16 } else { a as i32 as i16 }
            }

            len1 = trim_i16_encode(&mut [], 0, 0, &mut s1, logn, bits);
            if len1 != (((n * bits as usize) + 7) >> 3) {
                panic!("Error trim_i16 encode(0) {}", len1);
            }
            len1 = trim_i16_encode(tmp, 4 * n, maxlen, &mut s1, logn, bits);
            if len1 != (((n * bits as usize) + 7) >> 3) {
                panic!("Error trim_i16 encode {}", len1);
            }
            len2 = trim_i16_decode(s2, logn, bits, tmp, 4 * n, len1);
            if len2 != len1 {
                panic!("Error trim_i16 decode {}", len2);
            }
            assert_eq!(s1, s2, "trim_i16 encode/decode");
            s2.fill(0);
            // len1 = comp_encode(tmp, 4 * n, maxlen, s1, logn as usize);
            // if len1 == 0 {
            //     panic!("Error comp encode: {}", len1);
            // }
            // len2 = comp_decode(s2, logn, tmp, 4 * n, len1);
            // if len2 != len1 {
            //     panic!("Error comp decode: {}", len2);
            // }
            // assert_eq!(s1, s2, "comp encode/decode");
        }

        let b1p: *mut i8 = tmp.as_mut_ptr().cast();
        let b1: &mut [i8];
        unsafe { b1 = from_raw_parts_mut(b1p, n); }
        let b2p = b1p.wrapping_add(n);
        let b2: &mut [i8];
        unsafe { b2 = from_raw_parts_mut(b2p, n); }
        maxlen = tlen - 2 * n;
        for bits in 4..=8 {
            let mask1: u32 = 1 << (bits - 1);
            let mask2: u32 = mask1 - 1;
            for u in 0..n {
                let vec = i_shake256_extract(&mut rng, 1);
                let tt = *vec.get(0).unwrap();
                let a: u32 = (tt as u32) & mask2;
                b1[u] = if (tt as u32) & mask1 != 0 { (-(a as i32)) as i8 } else { a as i32 as i8 }
            }

            len1 = trim_i8_encode(&mut [], 0, 0, b1, logn, bits);
            if len1 != (((n * bits as usize) + 7) >> 3) {
                panic!("Error trim_i8 encode(0): {}", len1);
            }

            len1 = trim_i8_encode(tmp, 2 * n, maxlen, b1, logn, bits);
            if len1 != (((n * bits as usize) + 7) >> 3) {
                panic!("Error trim_i8 encode(0): {}", len1);
            }

            len2 = trim_i8_decode(b2, logn, bits, tmp, 2 * n, len1);
            if len2 != len1 {
                panic!("Error trim_i8 decode: {}", len2);
            }
            assert_eq!(b1, b2, "trim_i8 encode/decode");
        }
    }
}

pub(crate) fn test_shake256() {
    print!("Test SHAKE256: ");
    test_shake256_kat("", "46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5762fd75dc4ddd8c0f200cb05019d67b592f6fc821c49479ab48640292eacb3b7c4be");
    test_shake256_kat("dc5a100fa16df1583c79722a0d72833d3bf22c109b8889dbd35213c6bfce205813edae3242695cfd9f59b9a1c203c1b72ef1a5423147cb990b5316a85266675894e2644c3f9578cebe451a09e58c53788fe77a9e850943f8a275f830354b0593a762bac55e984db3e0661eca3cb83f67a6fb348e6177f7dee2df40c4322602f094953905681be3954fe44c4c902c8f6bba565a788b38f13411ba76ce0f9f6756a2a2687424c5435a51e62df7a8934b6e141f74c6ccf539e3782d22b5955d3baf1ab2cf7b5c3f74ec2f9447344e937957fd7f0bdfec56d5d25f61cde18c0986e244ecf780d6307e313117256948d4230ebb9ea62bb302cfe80d7dfebabc4a51d7687967ed5b416a139e974c005fff507a96", "2bac5716803a9cda8f9e84365ab0a681327b5ba34fdedfb1c12e6e807f45284b");
    test_shake256_kat("8d8001e2c096f1b88e7c9224a086efd4797fbf74a8033a2d422a2b6b8f6747e4", "2e975f6a8a14f0704d51b13667d8195c219f71e6345696c49fa4b9d08e9225d3d39393425152c97e71dd24601c11abcfa0f12f53c680bd3ae757b8134a9c10d429615869217fdd5885c4db174985703a6d6de94a667eac3023443a8337ae1bc601b76d7d38ec3c34463105f0d3949d78e562a039e4469548b609395de5a4fd43c46ca9fd6ee29ada5efc07d84d553249450dab4a49c483ded250c9338f85cd937ae66bb436f3b4026e859fda1ca571432f3bfc09e7c03ca4d183b741111ca0483d0edabc03feb23b17ee48e844ba2408d9dcfd0139d2e8c7310125aee801c61ab7900d1efc47c078281766f361c5e6111346235e1dc38325666c");
    println!("done.");
}


fn test_shake256_kat(hexsrc: &str, hexout: &str) {
    let mut rng: InnerShake256Context = InnerShake256Context {
        st: St { a: [0; 25] },
        dptr: 0,
    };
    let mut inn = hex::decode(hexsrc).unwrap();
    let out = hex::decode(hexout).unwrap();

    i_shake256_init(&mut rng);
    i_shake256_inject(&mut rng, &inn);
    i_shake256_flip(&mut rng);
    let out_shake = i_shake256_extract(&mut rng, out.len());
    assert_eq!(out_shake, out, "SHAKE KAT 1");

    i_shake256_init(&mut rng);
    for u in 0..inn.len() {
        i_shake256_inject(&mut rng, &mut inn[u..u + 1]);
    }
    i_shake256_flip(&mut rng);
    let mut output: Vec<u8> = Vec::with_capacity(out.len());
    for _ in 0..out.len() {
        let vec = i_shake256_extract(&mut rng, 1);
        output.push(*vec.get(0).unwrap());
    }
    assert_eq!(output, out, "SHAKE KAT 2");
}

const KAT_RNG_1: [u64; 128] = [
    0xDB1F30843AAD694C, 0xFAD9C14E86D5B53C, 0x7F84F914F46C439F,
    0xC46A6E399A376C6D, 0x47A5CD6F8C6B1789, 0x1E85D879707DA987,
    0xC7B0CE6C2C1DB3E7, 0xA65795537B3D977C, 0x748457A98AC7F19C,
    0xD8C8F161EEB7231F, 0xE81CAE53A7E8967F, 0x27EAD55A75ED57F8,
    0x9680953F3A192413, 0x784145D6687EA318, 0x9B454489BE56BAEB,
    0xF546834B0F799C67, 0xAC8E4F657C93FB88, 0xD0E6C7610CC4028B,
    0x417296FB7E1124BD, 0xE7968F18E3221DDC, 0x1DDEC33FC7F2D5FB,
    0x76556A8C07FB48EE, 0x7910EAA4C163BC2F, 0xAAC5C6291F779D17,
    0x575B2692885C4CFA, 0x0664AA8C3E99DA19, 0xFA55C1AE9A615133,
    0x7F1DB1A620F63220, 0xE740AE9AF9CC9755, 0x8393056E1D0D81E1,
    0x556EEF4483B434AA, 0xC6D17BEF7C2FB0C3, 0x27D142BD5BBF6014,
    0x6FD90B14DB4AA0BB, 0x7ACDD6F240530D0D, 0xE980F9F9DBE6109A,
    0xA30C677211C7BF37, 0x1E41FD290B90CE8B, 0x478FCD48D5E4A9ED,
    0x10586F987EA5FA7A, 0x691891C568F5DAC7, 0x3277735ED18D9107,
    0x78FCC576E47E8D71, 0x940A2C6777E3BEBB, 0x814612E210DD9715,
    0xABBCAFCC6B54279B, 0x2550E2538A063BFC, 0x7965EFC9D3F8A5BE,
    0xAE35E74B5A0B8717, 0xD855D6ABB96EA3AF, 0xAB4689B903C01C4E,
    0x8D8018988CA554AC, 0x0BB6689524F3A2B1, 0xAC0676FCBB193A87,
    0xD0A83D30F34F65EC, 0x26D3A8C167CA09F4, 0x7D17403D2B1DD9A0,
    0x47B1C836A0224550, 0xF6ABECF6422C5A56, 0x6FB1B2FF5CDDEC25,
    0x118276B244B55F88, 0x1FB953EF9E6C2C41, 0xF351C2717ACE9BF3,
    0xDF787B64D51A5440, 0xE4B8B81149B8A70B, 0x337E5363F506228B,
    0x48948ADE314B5980, 0x7FBF7A7139004610, 0xA6CB33F6802C96C7,
    0x745888A51A99BBED, 0x49D411403BA9CFDA, 0xA547A6EA4BDD5538,
    0x2D65DCF44F045E9F, 0x734FBE9360EFCC44, 0x1131E0AD573D37A0,
    0xADF3E9199FD90113, 0x8EDF3EAF50E6E00B, 0xFE0240D04C171901,
    0x45A97204596F7C46, 0x54D1D1F962484BC5, 0xEBAC109CDB975ED4,
    0x51182BF46BD2D61C, 0xF12D0EC8A80092D3, 0x69CA22BA55B34270,
    0x5FF97BBE7A525BF7, 0xF4E19780A4149ACA, 0x2CD5AE45826309FC,
    0xF0EF1F0A309C1BCF, 0xC16AF49962FE8A87, 0x2CD2575C27761E54,
    0xD9199411E9CC816D, 0xA0C397A63D036B05, 0xF439D283DFE4C172,
    0x5DAAD309E61F2A60, 0x2E7DDC8F9CD47E91, 0x2E1BFCDDC439FD58,
    0x8E62B7C84C3C27F8, 0xECD06ED0C1938A5E, 0x0335351E644A9155,
    0x71A735982C6DBBF7, 0xD8FE9FAF2DDF9AFF, 0x06BC9F654B9814E7,
    0x2DF46A488EC46052, 0x80CB8E04CDEF7F98, 0x9B65042EE20B4DAF,
    0x203BF49ACB5B34D2, 0x54E8F69957D8903B, 0x84D63D4BA389AF36,
    0x7A2D4A2230D0DC82, 0x3052659534D82FB8, 0xC5058A8EC3716238,
    0xB8063774064F4A27, 0x2F0BE0CE382BFD5B, 0xEE4CEAD41973DA0F,
    0xFB56581EB2424A5A, 0x09F21B654D835F66, 0x1968C7264664F9CC,
    0x2CBD6BB3DD21732C, 0xA9FB1E69F446231C, 0xDBEAD8399CB25257,
    0x28FF84E3ECC86113, 0x19A3B2D11BA6E80F, 0xC3ADAE73363651E7,
    0xF33FFB4923D82396, 0x36FE16582AD8C34C, 0x728910D4AA3BB137,
    0x2F351F2EF8B05525, 0x8727C7A39A617AE4
];

const KAT_RNG_2: [u8; 1024] = [
    0xC9, 0x45, 0xBC, 0xC4, 0x5B, 0x67, 0xA3, 0x25, 0x97, 0x19,
    0x64, 0x67, 0x4A, 0x98, 0xD4, 0xB7, 0xA7, 0x83, 0x18, 0xC8,
    0x40, 0xE2, 0x7F, 0xB8, 0x25, 0x8B, 0x7E, 0x92, 0x4A, 0x8C,
    0x68, 0x1B, 0x77, 0x61, 0x1E, 0x70, 0xED, 0xC2, 0xC4, 0xA5,
    0xDF, 0x9E, 0x76, 0xED, 0x49, 0x84, 0x3D, 0x08, 0xFE, 0xFE,
    0x99, 0xE2, 0xC6, 0xEF, 0xFE, 0x2C, 0xD4, 0xC0, 0x04, 0xD8,
    0x9A, 0x51, 0x21, 0xCD, 0x5B, 0xDB, 0x9F, 0x0B, 0x9C, 0x47,
    0xCF, 0xE8, 0x38, 0x6B, 0xB4, 0x94, 0xDC, 0xCD, 0x9A, 0x9B,
    0xB7, 0xED, 0xEE, 0x82, 0x64, 0x53, 0x20, 0xA0, 0x8F, 0x59,
    0xB2, 0x4F, 0xE2, 0x5A, 0x35, 0x88, 0x39, 0x5B, 0x6C, 0x59,
    0x59, 0x8C, 0x10, 0xC5, 0x2B, 0xF3, 0x7C, 0x49, 0xFD, 0x99,
    0x0C, 0x86, 0x07, 0x9E, 0x35, 0x71, 0x8E, 0x23, 0x7B, 0x9D,
    0x23, 0x34, 0x7A, 0xC8, 0x8A, 0x17, 0xDA, 0x7B, 0xA2, 0x97,
    0x0A, 0x78, 0x2B, 0x19, 0xAD, 0xB1, 0x35, 0xBD, 0xB1, 0xE7,
    0x74, 0x4B, 0x82, 0xFB, 0x72, 0x9C, 0x8C, 0x51, 0x3B, 0xE3,
    0xF0, 0x31, 0x11, 0xAA, 0x59, 0xA4, 0x66, 0xAC, 0xAA, 0x9E,
    0x85, 0xD9, 0x2D, 0xAD, 0xCA, 0x2B, 0x69, 0x5E, 0x19, 0x9F,
    0x77, 0x15, 0x43, 0xF0, 0xC9, 0x9F, 0xBC, 0x5B, 0x66, 0x26,
    0x7F, 0x7D, 0x7C, 0x95, 0x5D, 0x60, 0xE0, 0x49, 0x15, 0xC4,
    0x56, 0x47, 0x7E, 0x8D, 0x68, 0x3C, 0x54, 0x6F, 0x20, 0xF9,
    0x00, 0x43, 0xB4, 0x52, 0xD8, 0x46, 0x51, 0xFC, 0x0B, 0x92,
    0x15, 0xEF, 0x56, 0x45, 0x49, 0x94, 0xC2, 0xD0, 0x5E, 0x95,
    0xC4, 0x6D, 0x00, 0xDD, 0x13, 0x93, 0x78, 0xC2, 0x85, 0x21,
    0x5D, 0x18, 0x92, 0xB9, 0x48, 0xD2, 0x96, 0x45, 0x89, 0x0D,
    0x69, 0x2B, 0x85, 0x5D, 0x23, 0x5D, 0x10, 0x92, 0xD7, 0xDC,
    0xDC, 0xF8, 0x60, 0x5E, 0xED, 0x1F, 0x21, 0xB2, 0x19, 0x27,
    0xB7, 0xB7, 0xCD, 0x49, 0x98, 0x29, 0x90, 0xC9, 0x81, 0xCD,
    0x4E, 0x44, 0xB5, 0x39, 0x56, 0xED, 0x2B, 0xAA, 0x53, 0x34,
    0x3B, 0xB0, 0xBA, 0x1F, 0xBC, 0xF8, 0x58, 0x5F, 0x3E, 0xD0,
    0x4D, 0xB3, 0xA8, 0x5E, 0xC9, 0xB8, 0xD2, 0x70, 0xD3, 0x30,
    0xC0, 0x3C, 0x45, 0x89, 0x9B, 0x4C, 0x5F, 0xE8, 0x05, 0x7F,
    0x78, 0x99, 0x48, 0x3A, 0xD7, 0xCB, 0x96, 0x9A, 0x33, 0x97,
    0x62, 0xE9, 0xBD, 0xCE, 0x04, 0x72, 0x4D, 0x85, 0x67, 0x51,
    0x69, 0xFB, 0xD3, 0x12, 0xBC, 0xFC, 0xB5, 0x77, 0x56, 0x3B,
    0xB9, 0xB5, 0x3D, 0x5D, 0x7D, 0x2B, 0x34, 0xB0, 0x36, 0x2D,
    0x56, 0xE9, 0x24, 0xC2, 0x5A, 0xE9, 0x2A, 0xF8, 0xEE, 0x83,
    0x74, 0xC1, 0x0C, 0x80, 0xAD, 0x43, 0x5C, 0x04, 0x49, 0xB0,
    0x41, 0xD2, 0x29, 0x32, 0x9C, 0x7D, 0x70, 0xD5, 0x3D, 0xFE,
    0x82, 0x27, 0x8A, 0x38, 0x19, 0x12, 0x14, 0x78, 0xAA, 0x2A,
    0x29, 0xE2, 0x2B, 0xBB, 0x87, 0x4F, 0x7A, 0xDC, 0xC0, 0x72,
    0x30, 0xB6, 0xDE, 0x73, 0x7C, 0x04, 0x2D, 0xB6, 0xDF, 0x5E,
    0x4C, 0x3B, 0x82, 0xF6, 0x10, 0xE4, 0x94, 0xCE, 0x90, 0xD4,
    0x23, 0x0C, 0xBD, 0xCA, 0x56, 0xB7, 0x09, 0x6C, 0xAC, 0x35,
    0xA8, 0x47, 0xF0, 0x94, 0x21, 0xBD, 0xD5, 0x09, 0x18, 0x78,
    0x7C, 0x8D, 0x1E, 0x03, 0x15, 0xB1, 0x1A, 0xE8, 0x72, 0xB7,
    0x98, 0x5F, 0x23, 0x3A, 0x91, 0xB2, 0xDF, 0xFD, 0x70, 0x69,
    0xC4, 0x3B, 0xFA, 0x73, 0x17, 0xCC, 0xFB, 0xCF, 0xA6, 0xCF,
    0xC1, 0x32, 0x3E, 0x74, 0x0C, 0xCC, 0x73, 0xB2, 0xBE, 0x73,
    0xAC, 0x8E, 0x44, 0x51, 0x45, 0xED, 0xF6, 0x60, 0x21, 0x3D,
    0x0C, 0xE3, 0x3E, 0x1B, 0x11, 0x55, 0x68, 0x1A, 0x15, 0x97,
    0x80, 0x67, 0x23, 0x4F, 0x37, 0xF5, 0x30, 0x3D, 0x05, 0x4E,
    0xCF, 0x0E, 0x03, 0xB9, 0x2F, 0xD1, 0xD5, 0xD6, 0x5F, 0x79,
    0xF6, 0x61, 0x15, 0xBC, 0x79, 0x80, 0xA4, 0xD7, 0x98, 0x5B,
    0x38, 0x7A, 0x07, 0x9B, 0x02, 0xB2, 0x47, 0x89, 0xB2, 0x25,
    0xEF, 0x7B, 0xB1, 0xB0, 0xA5, 0x35, 0x39, 0xEB, 0xA0, 0x1C,
    0x24, 0xF4, 0xDB, 0x0C, 0x6C, 0x2B, 0xA3, 0x75, 0x47, 0x00,
    0xA3, 0xC8, 0xBC, 0x1E, 0x15, 0x3A, 0xC6, 0x1D, 0x91, 0x19,
    0xBA, 0xB4, 0xCA, 0x28, 0xD2, 0x57, 0x7C, 0x0D, 0x71, 0x4A,
    0x03, 0xD5, 0xAE, 0x96, 0x6D, 0x92, 0x70, 0x27, 0x82, 0x88,
    0xB6, 0x12, 0x1A, 0x84, 0x38, 0x1B, 0x74, 0x2F, 0x74, 0x33,
    0xE0, 0xA1, 0x82, 0x93, 0x62, 0xB6, 0x5B, 0x9E, 0x4E, 0xC2,
    0xE6, 0x5B, 0x49, 0x7E, 0x4A, 0x68, 0x8D, 0x08, 0xA9, 0xD8,
    0xEA, 0x47, 0xFC, 0xD2, 0x31, 0x21, 0x38, 0xEE, 0xE4, 0xE4,
    0x97, 0xFA, 0x91, 0x90, 0xC4, 0x26, 0x4B, 0xA5, 0xB3, 0x7D,
    0x33, 0x7F, 0x5A, 0x2D, 0x54, 0xB3, 0x01, 0xCF, 0x9C, 0x0D,
    0x9E, 0x97, 0x01, 0xE8, 0x54, 0x3C, 0xC2, 0x13, 0x69, 0x0C,
    0x35, 0xCD, 0x63, 0x02, 0x70, 0xC8, 0xA1, 0x1F, 0xC2, 0xBE,
    0x8F, 0xFC, 0xCE, 0x05, 0xA7, 0x3F, 0xCC, 0x04, 0x3D, 0x18,
    0xC4, 0x13, 0x38, 0x0D, 0x4C, 0xEE, 0x81, 0xFA, 0x02, 0xF8,
    0xFC, 0x4F, 0x21, 0xD0, 0xE6, 0xF2, 0x7B, 0x92, 0x76, 0xC5,
    0x8E, 0x96, 0x6C, 0x53, 0x84, 0x3E, 0x74, 0x1D, 0xD5, 0x0F,
    0x98, 0x03, 0x0E, 0x6A, 0x9D, 0x49, 0x03, 0xAE, 0xBE, 0x70,
    0x61, 0x5B, 0x45, 0xC0, 0x1E, 0x2F, 0x94, 0x42, 0xFA, 0x16,
    0x9F, 0xFA, 0xD5, 0x9B, 0x60, 0x88, 0x92, 0x19, 0x08, 0x02,
    0x31, 0x99, 0x6D, 0xA1, 0x72, 0xCB, 0x45, 0xC6, 0x93, 0xBA,
    0xA8, 0x71, 0x42, 0xC6, 0x85, 0x28, 0x6C, 0x1B, 0x60, 0x7C,
    0x14, 0x2F, 0x9A, 0x17, 0x10, 0x34, 0x27, 0x48, 0x36, 0xB2,
    0xE8, 0xD3, 0xEA, 0xE4, 0x9D, 0x67, 0xE4, 0x46, 0x2E, 0xC6,
    0x41, 0xE1, 0x83, 0x42, 0xB8, 0x82, 0x5F, 0x79, 0x61, 0xA3,
    0x0C, 0x63, 0x00, 0xCB, 0x7C, 0xB9, 0x30, 0x53, 0xF4, 0xFC,
    0xAF, 0xAC, 0x22, 0x71, 0x87, 0x4D, 0x4B, 0x4B, 0x9E, 0xAE,
    0x69, 0xB5, 0x58, 0x04, 0x9C, 0x03, 0x57, 0x58, 0x8D, 0x2F,
    0x82, 0x95, 0x57, 0x2F, 0xC3, 0xA1, 0xC5, 0xB1, 0xF1, 0xF1,
    0x98, 0x9A, 0xF8, 0x99, 0x74, 0x5C, 0xC5, 0xAC, 0x4A, 0x32,
    0xE9, 0x24, 0xCF, 0x1D, 0x1E, 0x29, 0x18, 0x7C, 0xBF, 0x43,
    0x74, 0x23, 0x28, 0xB0, 0x3D, 0xD1, 0xB3, 0x8C, 0xE1, 0x28,
    0x02, 0x3E, 0x8F, 0x7F, 0xDD, 0xF0, 0x5B, 0x4D, 0x37, 0x96,
    0xF7, 0x73, 0x73, 0x7F, 0xBC, 0xAD, 0x6C, 0x84, 0xFC, 0x47,
    0xD2, 0x1E, 0xAB, 0xEB, 0xB6, 0xCA, 0x4E, 0x3A, 0x2C, 0x47,
    0x59, 0x61, 0x0D, 0xA0, 0x17, 0xCF, 0xDD, 0x62, 0x6F, 0xA3,
    0xF4, 0x72, 0x2D, 0xB0, 0xB2, 0x34, 0x2A, 0xE1, 0x63, 0xC3,
    0x5B, 0xAC, 0xE8, 0x6F, 0x92, 0x77, 0x78, 0xE2, 0x34, 0xAD,
    0x4F, 0x6C, 0xFF, 0x71, 0xE1, 0x92, 0xFD, 0xED, 0xA1, 0x20,
    0xCA, 0xCB, 0x80, 0x32, 0xD1, 0x78, 0x72, 0x68, 0xFE, 0xAE,
    0x73, 0x22, 0xD7, 0x60, 0x23, 0x1D, 0x3D, 0x06, 0xD6, 0x2A,
    0x81, 0xC4, 0x43, 0x98, 0xFD, 0x4E, 0xBD, 0x85, 0x09, 0x29,
    0x11, 0xE8, 0x36, 0xE1, 0xCE, 0xCF, 0x07, 0xA7, 0x45, 0x8C,
    0xCB, 0xB2, 0xDC, 0xD0, 0x98, 0xB9, 0x93, 0x33, 0x8A, 0x2A,
    0x13, 0x82, 0x36, 0x3D, 0x22, 0xB0, 0x9C, 0x74, 0x3F, 0xCE,
    0x6F, 0xCC, 0x69, 0xFF, 0x81, 0xE8, 0xAE, 0xC8, 0x57, 0x0D,
    0x98, 0xEB, 0xC5, 0x2A, 0x45, 0x55, 0xDC, 0xBB, 0x0A, 0x5B,
    0x3D, 0xB4, 0x61, 0xC4, 0xAE, 0x11, 0x68, 0x7D, 0xD4, 0x45,
    0x83, 0xAE, 0x66, 0xC8
];


const ntru_f_16: [i8; 16] = [
    7, -7, 12, 18, 19, 6, 18, -18, 18, -17, -14, 51, 24, -17, 2, 31
];

const ntru_g_16: [i8; 16] = [
    -2, -35, 3, 28, -21, 10, 4, 20, 15, -28, 31, -26, 5, 33, 0, 5
];

const ntru_F_16: [i8; 16] = [
    16, 65, -6, 15, 26, -10, 14, -9, 22, 48, 26, -14, 15, 21, -23, 4
];

const ntru_G_16: [i8; 16] = [
    37, -57, 27, 31, -45, -49, -11, 46, -14, 26, 0, 3, -33, -33, -3, 54
];

const ntru_h_16: [u16; 16] = [
    7768, 1837, 4498, 1226, 9594, 8992, 2227, 6132,
    2850, 7612, 4314, 3834, 2585, 3954, 6198, 589
];

const ntru_pkey_16: &str = "04796072d46484ca95ea32022cd7f42c89dbc4368efa2864f7260d824d";

const ntru_f_512: [i8; 512] = [
    4, -4, 0, -6, 6, -6, 2, 1, -8, 0, -2, 0, -1, -1, -4, 8, -5, 3,
    -2, 2, 0, -5, -2, -1, 3, -4, -5, -1, 8, 1, 1, 7, 5, 1, 6, 2, -1,
    -13, 1, -4, 9, -4, -2, 4, -4, 0, -1, -1, -3, 2, 1, 1, 1, 3, -3,
    2, -1, -1, -5, 9, 4, -7, -3, -8, -3, -2, -3, -6, -6, -3, -2, -2,
    2, 1, -10, -2, -2, 4, 2, 0, -2, -2, 4, -3, 5, 2, -2, 3, 8, 1, 8,
    -3, -4, 2, 7, -5, -4, -2, -2, -3, 5, -5, 0, -3, -5, 3, -6, -2,
    3, 0, 3, 1, 2, -2, 1, 6, -1, -7, 0, -5, 3, -5, 9, 0, 1, 5, -4,
    0, 5, -1, 4, 3, 5, -6, 2, 0, -7, 1, 0, 0, 2, 4, 1, -7, -3, 4, 4,
    -2, -7, -5, 6, 3, 2, -5, 6, -1, -1, -4, 1, 2, 1, 2, -10, -9, -9,
    -1, 3, -2, -2, -6, 1, -2, -4, -1, 2, 3, 8, 2, 1, -1, 8, 0, 7, 3,
    1, 5, 0, -7, 1, -6, -4, 4, 2, 0, 0, -3, 2, 0, -3, 0, 7, -1, -1,
    -7, 2, 5, 3, 0, 1, 6, -2, -1, 2, 0, -1, -3, -6, -5, -5, -4, 0,
    1, 7, 1, -3, 2, 2, -5, 0, -4, 3, -4, 5, 3, 4, 7, -2, 15, -3, 1,
    1, 4, 5, -9, -3, 4, 2, -4, -4, -3, -1, -4, 3, -1, 1, -8, -4, -1,
    0, -3, 1, -1, 3, 3, 3, -3, -6, -7, 7, 0, -6, 2, -1, 4, 7, 1, 4,
    0, 1, 6, -1, -2, -2, 5, 0, 6, -3, -2, -5, 3, -1, 0, 5, -2, 8,
    -5, -4, 1, -3, 8, 2, -4, 1, 6, 0, 0, -1, 0, -4, -5, -2, 3, -2,
    5, 1, 4, 5, -4, 4, -1, 4, -5, -2, 1, 3, -5, 1, 2, -2, 0, -5, 1,
    8, -3, -4, 3, -2, -3, -4, 4, 3, -2, 6, -3, -2, 4, 0, -2, 0, -5,
    1, -9, 5, 6, -2, -6, 1, 5, -1, -7, 1, 2, 5, 2, 0, -1, 0, -2, -4,
    -1, -8, 5, -5, 9, -4, -4, 2, -5, -1, 0, 1, 4, 3, 1, -2, -7, -8,
    -4, -4, 4, 3, -1, 4, -1, -1, 1, 0, 6, 1, 0, -6, -2, 0, -3, 0,
    -1, -1, 0, 3, -5, -2, -5, 6, 2, -4, -3, 4, -8, 1, -1, 4, -3, 5,
    -2, 8, 7, -1, -3, -3, -2, 0, -4, 4, 0, -6, -4, -2, 5, 8, -3, 3,
    -1, 0, -5, -5, 0, 2, -5, -2, -3, 1, 6, 3, 1, -3, 4, -3, 0, -7,
    -1, -3, 1, -5, 1, -4, -2, 2, 4, 0, 1, 5, 2, 2, -3, -5, -8, 4,
    -2, -3, 2, 2, 0, 8, -5, 2, -7, 0, 3, -1, 0, 4, -3, 1, -2, -4,
    -6, -5, 0, -4, 1, -3, 9, 1, -3, -2, -3, 5, -1, -4, -7, 1, 1
];

const ntru_g_512: [i8; 512] = [
    -6, -2, 4, -8, -4, 2, 3, 4, 1, -1, 3, 0, 2, 3, -3, 1, -7, -5, 3,
    -3, -1, 3, -3, 8, -6, -6, 0, 6, 4, 7, 3, 5, 0, -5, -3, -5, 7, 3,
    -1, -4, 3, 4, -1, 1, 3, -3, -4, -4, 4, -5, -1, 3, 7, -2, -4, -2,
    -3, -1, -2, -1, -2, -6, -7, -3, -6, -3, -6, 4, -1, -5, 1, 4, -4,
    3, -1, -6, 6, -2, 2, -6, 5, -7, 8, -3, 0, -2, 0, 7, 1, 3, 6, 4,
    -5, 2, 2, 2, 4, -4, -5, -4, -3, 4, -7, 7, -6, -2, -7, 1, -2, -2,
    -3, 1, 3, 7, 0, -1, -5, 4, -8, -8, 0, 3, 6, -3, 2, 6, -1, 1, -5,
    -4, 2, -3, 8, -2, 2, 3, 0, 1, 6, 4, 4, -4, -1, -3, -2, -5, 3, 9,
    0, 4, -1, 1, -4, 0, 3, 0, -2, 8, 0, 1, 0, -1, 1, 9, -1, -4, -1,
    3, 5, -2, -2, 1, -1, 1, -1, 0, 1, 0, -6, -2, 0, 7, -3, -4, -1,
    -6, -2, 5, -2, 0, 4, -3, -5, 0, 1, -1, -3, 5, 5, -4, -4, -5, -1,
    9, -1, -5, -7, -1, -2, 4, 2, 5, -4, -1, -5, 8, -3, -6, -2, 1,
    -2, 1, 1, 4, -4, -1, 4, 1, 1, 0, -5, 1, 7, 2, -3, 3, -2, -4, 1,
    -6, -1, -3, 7, 6, 0, -2, 2, -6, -4, -3, 2, -7, 7, 0, -11, -1, 3,
    4, 0, 6, -8, -4, -1, 1, 0, -3, 7, 0, 0, -2, -1, -4, 0, -1, -3,
    7, -6, -2, -2, -1, 0, -2, 8, -6, 4, 4, 6, -2, -1, 0, -13, 1, 2,
    0, 5, -7, 3, -2, -6, -3, -4, 4, -1, 1, 3, -6, 1, -5, -8, 2, -11,
    -1, 2, -2, 0, 0, 1, 1, -4, -5, 0, 1, 0, 1, -6, -2, 2, 0, 7, 1,
    -1, 1, -2, 1, -3, 1, 2, 1, -7, -2, 2, -1, 4, 1, -2, -2, 0, 4,
    -3, -6, 2, 3, 1, 1, -4, 6, -2, -4, -3, 0, 4, -5, 0, 1, 8, 2, 2,
    -1, 1, -2, -4, -1, 4, 4, -1, 7, 2, -1, -3, -8, 3, 1, 1, 0, -1,
    1, -7, -8, 2, 1, -2, 1, 0, 4, 1, 1, -2, -1, -5, 3, -4, -1, -1,
    -8, 2, -4, 3, 2, -5, 0, 1, 5, 2, -5, -2, 3, 7, 5, 6, 5, -2, 1,
    3, -7, 7, -3, -8, -2, 2, 3, 3, 5, -2, -4, -1, 7, -2, 7, -3, -2,
    0, 3, 5, 0, 0, 4, 8, -1, -5, 3, -2, -2, -5, -5, -2, 2, 5, -8,
    -1, -2, -4, 6, 0, 6, -5, -1, -5, -6, 9, 5, -2, 4, -1, -8, -2,
    -2, 1, -8, -5, 6, -1, 0, 5, -6, -3, -3, -2, -6, -2, 0, -1, -3,
    7, -3, -1, 3, 6, 3, -2, -4, 2, 1, -1, 11, 3, 4, -1, -6, 1, 2, 3,
    3
];

const ntru_F_512: [i8; 512] = [
    -3, -27, 4, 18, 39, 7, 20, -13, 33, -29, 3, 38, 30, 26, -6, 24,
    -26, 16, 24, -48, -18, -21, 3, -14, -2, 6, -9, 42, 22, 21, 33,
    -27, -14, -14, -56, -68, -2, -33, 6, -38, -43, 21, 13, 6, 2,
    -69, -10, -30, -27, 23, -1, 41, -21, 11, -20, 15, 39, 5, 41, 15,
    -28, -34, 9, -11, 9, -1, -8, 61, 8, 13, -23, 2, 7, -23, -21,
    -54, -11, -9, -19, 40, 37, -2, -16, 19, -16, 2, -78, -35, -19,
    11, 17, -46, -16, 25, 0, 22, 13, -15, -33, 13, -15, -34, 33,
    -13, 38, 39, 37, -29, 40, 7, 63, 35, 15, 21, -24, 16, -6, 30,
    12, 18, 61, 17, -11, -15, 11, 0, -15, -2, -14, -26, -1, -42,
    -10, -52, 64, 45, 22, 6, -22, 32, -50, -16, -12, -16, -8, 34,
    -17, -18, 7, 19, 37, 41, -5, -22, -12, -7, -17, -27, -17, 4, 36,
    0, 22, -4, -50, 24, 30, 5, 1, -50, 43, 0, 0, -6, -9, 34, 0, 14,
    -27, 17, 35, -30, -13, 3, -23, -46, 17, -34, 30, 24, 47, 31, -7,
    11, 10, 16, 30, 27, -4, 11, -4, -14, -28, 49, 0, 27, -5, -10,
    53, -50, -13, -15, 13, -10, -26, 2, -3, 88, 22, -27, 40, -23, 3,
    -42, 2, -27, -12, 35, 26, -33, 38, -42, -5, 17, -24, 6, -10, 13,
    -10, -30, -35, -17, 25, 49, -29, 48, 19, 37, 48, -25, -31, -41,
    -15, -1, 19, -17, -7, -16, 2, 5, 12, 0, -15, -19, -6, -32, -4,
    -56, 14, -6, -7, 17, 24, -1, 17, -35, 5, 3, -64, -15, 4, 0, -31,
    4, -10, -18, 55, 13, -13, 23, -30, -11, -29, -21, 15, -18, 30,
    39, 16, -27, 31, 4, 31, 39, -49, 11, -25, 37, -42, -72, 28, -57,
    13, 34, 6, 10, -17, -3, -19, -43, -1, -32, 9, -11, 9, 11, -23,
    6, 28, -34, -12, -42, -7, 42, -18, -2, 22, -30, -4, -42, 10, 54,
    -16, 19, -23, -4, 18, -58, 26, -3, -38, 20, 38, 23, 20, 0, 10,
    49, 47, 18, 27, -11, -10, -14, 0, 6, 6, -18, -6, 14, -38, -16,
    12, -17, 17, -21, -52, -3, -53, 9, 9, -4, 44, 9, -4, 17, 2, 10,
    -28, -13, 28, -12, 11, -33, -2, 33, 0, -51, 2, -33, 20, -47, 23,
    42, 2, 52, -18, -17, 35, 6, 27, 3, 11, 24, -8, 0, -35, -44, -22,
    -49, 61, 3, -15, -2, -14, 46, -24, -10, -24, -24, -21, -10, -51,
    -3, 31, 20, 1, -44, 18, 9, 38, 26, -17, -8, 2, 33, 24, -8, -9,
    -20, 32, 54, 47, -11, 40, 3, -58, 13, 17, 29, -21, 27, 4, -31,
    14, 14, 17, 19, -29, 19, -86, -29, -15, -35, 18, 53, -10, 9, 13,
    -38, 9, -4, 80, 0, 6, 1, 15, -14, 0, -5, 45, 26, 50, 28, 21, 1,
    -8, -6, 12, 32, 5, -21, -1, 54, 14, 22, 27, 6, 8, -18, 33, -5
];

const ntru_G_512: [i8; 512] = [
    -58, -47, 36, 14, 2, -20, 24, 35, 38, -31, 20, -1, -17, -29, -6,
    25, 16, 4, -75, 32, 20, 17, -8, 24, 13, 7, -11, -2, 3, -2, 6,
    16, 22, 37, -25, -4, -32, -21, 57, 0, 20, 73, 20, -9, 6, -49,
    12, 14, 60, 15, 50, 15, 9, -2, 13, -8, 38, 12, -5, 9, -7, -1,
    -30, -2, -28, -6, 6, -18, 5, 1, -15, -15, -1, 15, -4, -12, -1,
    -37, -12, 33, -2, -17, -8, -57, -64, -7, 3, 3, -15, 1, 3, 15,
    -21, 67, -29, -4, 7, -21, -7, -8, 12, 38, 3, 45, -26, -37, 10,
    24, -15, -24, 23, -13, -27, 12, 14, -16, 22, -19, 15, 1, -7, 0,
    -6, 28, 2, -7, 0, -10, -19, 14, -13, -16, 22, 14, -7, -1, -17,
    31, -7, 12, 4, -8, -13, 18, -8, -38, 6, 49, 40, 1, -5, 1, 17,
    -21, -3, -9, 15, 27, 43, 60, 29, -1, -3, 2, -41, 18, -18, -26,
    29, 13, 12, 15, 38, -24, -25, 8, 17, 10, -32, -3, -39, -29, 23,
    30, 6, 3, 23, -15, -26, 34, -59, 3, -54, 37, 27, -26, 23, -40,
    -10, -15, 7, 9, -1, 24, -33, -36, 29, -7, 13, 29, 56, -13, 36,
    -37, -45, 13, -11, 43, -9, 24, 16, -13, 13, 13, 10, -18, 16, 3,
    -14, -27, -16, -5, -35, -25, -42, 51, -5, -41, 20, -27, 47, 14,
    75, 14, -74, 26, -18, 18, -12, -13, 8, -44, 6, 0, -16, 36, 32,
    -33, 6, 5, -23, -14, -32, 15, 27, 56, 10, -43, 8, 16, -63, 8,
    34, -24, -3, 15, 13, -3, -31, -14, -24, 28, 11, -41, -4, 14, 1,
    -11, 40, 32, -22, 19, -16, 27, -2, 36, 40, -11, -2, 2, -11, -2,
    35, -44, 6, 6, -40, 3, -8, -27, -28, 5, -6, -12, -2, -4, -19,
    -64, 36, -33, -16, -5, 20, -9, 10, 59, 16, -8, 27, 28, -6, 10,
    -8, 11, -35, 23, 35, 34, 47, 13, 2, 44, 8, 38, 4, 34, 41, 15,
    20, 28, 29, 35, 4, 23, 7, 8, -19, -17, 8, 5, 10, -21, -9, 15,
    -28, -4, -23, -17, 18, -15, -2, 9, -24, -7, 7, 51, -31, 40, -57,
    30, 23, -12, -18, -9, 37, -29, -4, 18, -10, 20, -54, -21, 23,
    56, 28, 30, -3, 15, -31, -41, 14, 66, 41, 15, 39, 34, -9, 4, 33,
    18, 25, 6, 38, -27, 63, -21, -24, 43, 11, -37, 16, 26, -31, -30,
    16, -20, -3, 10, 11, -58, 18, 13, 0, 22, -25, 13, -32, -14, 5,
    52, 31, 30, -3, 9, 18, -7, 4, -2, 1, 11, -8, 8, -9, -29, 28,
    -25, 51, 47, 24, -72, -4, 41, -15, 17, 50, 2, -1, 2, -41, -33,
    -20, 6, -19, -33, 23, 8, -19, 12, -19, -2, -61, -7, -19, 6, 8,
    -23, 27, 38, 12, -17, 39, 8, 23, -41, 14, 25, 16, -44, -46, 11,
    31, -6, 9, 24, 23, 37, -57, 22, 21, -22, 21, 44
];

const ntru_h_512: [u16; 512] = [
    3605, 11394, 3623, 9500, 11987, 4336, 3361, 1348, 6563, 8102,
    758, 8455, 5789, 7614, 797, 11215, 7518, 3116, 4556, 1762,
    11267, 9507, 4586, 5420, 4091, 6788, 1729, 6433, 4730, 1830,
    4200, 1416, 3705, 5380, 5767, 9261, 924, 6822, 8978, 2536, 8232,
    10530, 10137, 11653, 11704, 1887, 11653, 10218, 9207, 10699,
    3288, 1478, 7261, 10152, 3871, 10134, 7359, 9993, 9510, 8661,
    419, 1826, 978, 11037, 10899, 3311, 2064, 5939, 11072, 1748,
    9516, 5458, 7665, 4459, 5937, 5615, 7288, 3438, 6009, 3217, 264,
    3696, 608, 11576, 2774, 10976, 11146, 11188, 3237, 10913, 3541,
    11755, 9412, 5720, 4226, 1154, 9010, 9922, 3994, 11252, 11575,
    11077, 9308, 7784, 11086, 12047, 5310, 8524, 4117, 504, 3145,
    12216, 2718, 1181, 5446, 1818, 6156, 1945, 11240, 7398, 8307,
    8259, 10113, 11431, 10691, 2147, 2742, 8242, 12031, 8808, 7609,
    3657, 3567, 2485, 7669, 4388, 3255, 1395, 596, 9635, 6739,
    10284, 4910, 9410, 11788, 10978, 3877, 4006, 1860, 6225, 8834,
    11969, 11742, 9733, 8790, 7871, 10347, 2658, 4468, 947, 3384,
    9733, 6496, 382, 81, 7977, 7138, 8962, 10195, 2830, 10227, 5302,
    9974, 9157, 7442, 4931, 9761, 5759, 2115, 431, 12242, 2353,
    7529, 7822, 6343, 3370, 9369, 8491, 6742, 5681, 10973, 412,
    12105, 6913, 5565, 3760, 4378, 4454, 9070, 1289, 2596, 5355,
    12117, 2787, 3798, 4954, 9708, 2191, 2935, 4073, 7455, 11661,
    4170, 8782, 9611, 8647, 2318, 4779, 11339, 3962, 361, 9358,
    7727, 11723, 9018, 10552, 3025, 6852, 6028, 10603, 7147, 8434,
    5604, 4483, 5954, 426, 11403, 2643, 8294, 9504, 7268, 8958,
    2773, 7764, 5926, 8213, 2100, 8814, 7540, 4212, 7012, 353, 7166,
    5717, 9799, 10379, 7768, 9515, 2534, 4504, 5410, 5358, 1879,
    11581, 10692, 2614, 11002, 11667, 7333, 6932, 4254, 9503, 7386,
    2581, 4153, 6079, 6149, 5496, 2397, 11735, 6496, 9250, 11872,
    10842, 2934, 4022, 10681, 914, 4397, 7287, 9673, 4709, 4895,
    3770, 3146, 7254, 4953, 11018, 9062, 3817, 11979, 8723, 3091,
    2675, 8946, 7376, 3652, 6861, 8298, 5547, 11, 4758, 10734, 7434,
    11702, 6466, 9135, 11199, 10059, 503, 2510, 1730, 6101, 11965,
    10264, 6045, 11690, 11530, 761, 9270, 4531, 5482, 6951, 5776,
    10348, 2668, 5246, 8046, 7106, 11302, 3276, 6632, 12008, 6564,
    8465, 1953, 5904, 1036, 3109, 5020, 11945, 458, 11742, 5271,
    4474, 9918, 7963, 11786, 8318, 756, 560, 11377, 1084, 9634,
    9203, 1062, 8461, 1845, 3719, 6672, 6660, 4711, 11337, 10460,
    5367, 4072, 7043, 5567, 6356, 657, 8877, 3633, 11487, 10421,
    10877, 5052, 2174, 4711, 11853, 4461, 10942, 11619, 7591, 3424,
    3372, 4493, 11393, 7115, 9057, 7145, 2060, 9137, 707, 1968,
    7853, 645, 253, 2697, 9294, 8357, 7503, 6187, 7505, 8302, 4635,
    8899, 9258, 8559, 7988, 9571, 243, 6979, 8233, 11555, 5257,
    8361, 1836, 11185, 3771, 3517, 10585, 4756, 10212, 2035, 2778,
    6798, 11229, 11768, 8707, 7931, 3744, 10939, 5317, 6104, 11137,
    3936, 5418, 4368, 201, 3094, 8211, 6803, 2559, 3164, 6846, 8515,
    8894, 8556, 2219, 9593, 6391, 3374, 4868, 192, 2791, 4309, 62,
    20, 9968, 8831, 11185, 1365, 9722, 5623, 2398, 5049, 2241, 6060,
    998, 4233, 1455, 5324, 1053, 5626, 1726, 11569, 12033, 4897,
    859, 1676, 2097, 11147, 5155, 5187, 2026, 12050, 5615, 5450,
    260, 7526, 11923, 6346, 7221, 405, 882, 842, 4621, 4130, 3513,
    114, 3673, 4914
];

const ntru_pkey_512: &str =
    "093856c82389e51cbb4d0f03484544668dfa60bda1075a75dbe0c76bcf7578c2c47306e2b00e52347a952c3feda841b0592149e872641a058839e55045a1e42d0e71aa68c489e880a29229e66d85b6e075fb6167ea8fde9cb33605c671767a83c7e79672fe709949a1d5068c7220f4ab1daa4ccef2041733ad006d494b155277c516b5cc55ef71e0d6e5de4c910420e700982d382b5aae0ae2abb43296aa13756deb931165842084828cca6c23e6abf4b4deb459171e68ad3af0f52fa14c40541f83126fb82a7849d551871a6030799afa1ce681ce0439e06ca7a70c8632ada032bbfe26876e4e4937bc9b577d512432dc57309525a3694e82c4cba4c2b832ae23c94fa61d118518a0aec1b77a6058959ebfa1aca6245d03b334e2605658017e0145f296f8a3029f4cb0e9fcd4b69bda3c57449343988567f210c1afbf4893175a5e8e631cd2a926612b6959631ab7419cbd25b0156f4eb044691668db850928914ebbd54ae33b5935a97b088f2ddcfe9747ed8d412a24e962e1c724392abb12cf7a05a648e78bedcb8cea9382f45ac45e3296b6fae0f257911835d081aab22ca53819a52071922fe2b55e545c9a01520d226e75d10746d901616ff9655991e88b796252b279919854894ee1d5ed3da710a36abead937295b14427a51f7368a1540e57bf60155782576dd76582422b982a5a2dd8fb6a6e439244b5c7797252654c7ceba3129c564d66b0a8d98ee9bb2e213304ca738bc9cd03911acd81a95ab002d296a7b9d0ab6d99428ebebbf9d2c1f727386c25f56ebda06179db6aad0a0be643646cd56a6c9d690a1b0a6c51f9f6e6f0ac2633319e8bba19a484447a15c4040c309539cbaa41cab77949745ea6be7c6ee0a81f82f408c2c7110f25a28fcc42684347353a1da106811267b1268dc53dcfe86e0d5bf63502918ab4e31b37e8b5a9f53bc21f9267b93516daafad63769cd6034b118db205bcb8d85be920323b10b0c7b07ab428503f4a89913a0a5753d82b754606e486e2c390aa16f7cd256303cdb4380a6d2352260a91cb2bb13aecdbda5652949f907f32b69a8eaf76df8880defb3a82abb53157d8ae04f6054a91100324c16804da9327fcc5c6afa1438afa16c22ae57963dcd2e4c100c02b9d0d500f80149bc227faec455597e95f725793b923057ac0f9908916bd4cc10755fa1afad31bc053210d6c68c20c6b8b508d4431faaf1257bd54a0411d66ba4d8ca70d41950dc834a483502236e40723965332";

const ntru_f_1024: [i8; 1024] = [
    3, 2, -4, -3, 0, -5, 4, -3, -1, 1, -2, 2, 3, 0, -1, 0, 0, 0, 0,
    0, -2, -2, -3, 3, -4, -1, -1, 0, 2, -4, 0, -9, -3, 5, 3, -1, 1,
    -5, 1, -1, -6, 0, -1, -1, 5, -1, 4, -1, -2, 2, -3, -1, 1, -3, 1,
    1, -5, -2, -2, 0, 0, 5, -8, -1, -1, 0, 0, 2, 0, 4, -3, 4, 3, -3,
    -2, 6, -2, -2, 0, 3, 0, 0, -1, -2, 0, 1, -2, -1, 7, 0, -4, 1,
    -1, -2, 2, -1, -5, 5, -1, -4, -2, -1, 2, -1, 2, -1, -3, 3, 1, 2,
    1, -2, 3, 3, 1, 4, 2, 0, 3, 0, -3, 0, 7, -5, 4, -3, -1, 1, -6,
    0, -1, 0, -5, 1, -2, 2, -1, -1, -2, 3, 4, -1, 0, -1, 0, 1, 3,
    -2, 6, 4, 1, 1, 1, -1, 3, 1, -3, 0, 0, -1, 2, -3, 2, -4, 1, -1,
    -1, 1, -2, -1, -2, -6, -8, -3, 3, -2, 0, 3, -1, 1, 0, 5, -2, 0,
    -2, 1, 2, 1, -2, -5, -3, -2, 2, -1, 1, 1, 0, 1, 1, -9, 0, 1, -2,
    2, 5, 0, 3, 4, -1, -5, -2, -2, 0, 2, 0, -2, 3, 0, -1, -2, -3,
    -6, -2, 1, -7, -4, -2, 1, -1, -1, -3, -3, 2, -3, 2, 1, 2, -4,
    -2, 5, -1, 1, -2, 3, -5, 5, 1, 1, 1, 2, -4, 1, 2, -3, -5, -4, 2,
    -3, 3, -4, -4, 5, 2, 0, -4, 3, -3, -3, 3, 0, -2, -3, -2, 0, -3,
    5, 8, -2, 2, 2, -1, 10, -1, -1, 1, -4, 3, -1, 3, 1, -1, 3, -3,
    5, 0, 4, -3, 4, -5, 3, -3, -1, -4, 3, 1, 0, 2, -1, -4, -1, 4,
    -5, -6, 3, 5, 4, -2, 0, 5, 1, 1, -1, 1, -3, 6, -3, -3, 1, -6, 3,
    -3, 3, 1, 0, 0, -2, -4, -1, 1, 0, -1, -1, 4, -3, 2, 2, -5, 4, 3,
    -3, -2, -1, 1, 1, -3, 4, 6, 0, 4, -3, -1, 1, 6, 2, 3, -2, -3,
    -2, 3, -5, 3, 0, 0, -2, -1, -3, 2, 5, 0, 4, 2, 3, -2, -4, 3, 1,
    -5, -2, 2, -1, -1, -4, 5, 0, 0, 2, -2, -4, 3, 4, 1, 5, 0, 0, 1,
    0, 5, -2, -1, 1, -3, -1, -1, -1, 2, 2, -3, 0, 1, -3, 2, 0, -2,
    3, 2, 0, 3, -1, 0, 2, -4, 0, -3, 3, 3, -1, -1, -2, 3, 2, -1, -5,
    1, 2, 3, 3, 3, -2, 8, 6, -2, 1, 4, -3, 0, 4, 1, 1, 0, 0, 1, 0,
    -1, 5, 0, -5, 4, 3, 5, 2, 0, 0, 3, -2, 3, -4, 2, -3, -1, 3, -2,
    1, -1, 5, 1, 0, 5, -1, 0, -1, -2, -1, -4, -1, -2, 2, -6, -2, 5,
    0, -5, -2, 4, -1, -3, -3, -2, 2, -1, -2, 0, 0, -4, 2, -5, 1, -4,
    -1, 0, -1, -2, 5, -4, 4, -1, -2, 6, 3, 6, 3, -3, 1, 0, 1, 6, 3,
    1, -3, -1, -2, -1, 0, 0, -1, -1, -1, -2, -2, -1, -1, 2, 1, -4,
    -4, -5, -4, 3, 1, 1, 1, 4, -2, 0, 1, 2, 3, 0, 5, -5, 3, -1, 3,
    4, -3, -3, -4, -10, -1, -2, 2, -2, -3, 1, 2, 0, 1, 0, 2, -3, 2,
    1, 1, 0, -2, 1, 0, -3, -1, 0, -3, -1, -4, -5, 3, 3, -1, -5, -1,
    0, -1, 2, -3, 3, 1, 0, 0, -1, 2, 0, 6, -4, 5, -1, 1, 3, -2, -1,
    1, 0, 1, 2, -7, 2, 4, 2, 1, -3, -6, 2, -2, 0, 0, -2, 2, 2, 0,
    -2, 1, 0, 1, 0, 1, 2, -3, 2, 3, 1, 1, 2, 7, 0, 1, 0, 0, -1, 2,
    1, -1, -1, 0, 0, 3, 2, 1, 1, 0, -5, 5, 0, 3, 3, 5, 3, -3, 0, 3,
    1, 4, 0, 6, -1, 3, 4, -1, 5, 5, 4, -4, -2, 2, 3, -10, -3, 1, 1,
    4, 2, -3, 4, 2, -2, -4, -2, -4, 1, 0, 0, 2, -3, 1, -1, -4, 0,
    -1, 4, 1, -3, 0, -1, 1, 3, 2, 3, 1, 2, -3, -3, 0, 2, 4, 0, 0, 6,
    -2, -1, -2, -2, 4, -1, -1, -1, -2, -4, -6, 2, 1, 1, -6, -2, -2,
    1, 0, -3, -3, -3, 3, 3, -1, 1, -3, -1, -1, 1, -6, -1, 2, -1, -4,
    0, 0, -4, 2, 3, 0, -4, -1, 0, 2, 0, -1, 1, 3, 3, -1, -2, 5, 0,
    -1, 1, 0, 2, -5, -1, 0, 1, -5, -4, -4, 0, 1, 4, -6, -3, 1, 0, 2,
    -5, -1, 0, 1, -5, 1, 1, 0, 1, -1, 3, 4, 3, -1, -1, 1, 1, 0, -2,
    0, -1, -4, 0, 2, 1, 2, 4, 2, -1, -4, 2, 2, -1, 0, -1, 0, 0, -2,
    3, 0, -1, -4, 0, -2, -2, -2, 4, 3, 5, 4, 1, 4, -2, 3, 0, -4, -2,
    -3, 2, 2, 0, -6, 2, -7, -1, 3, 1, -2, 4, -2, 0, 1, 2, 4, 0, 1,
    1, 0, 0, 0, 1, 4, -10, -2, -3, -4, 7, -6, -2, -3, 4, 4, -4, -2,
    6, 2, -4, -1, 1, 1, 3, 0, 1, 0, -2, -4, 1, 4, 1, -5, 1, 1, 6,
    -3, 0, 1, -2, -4, 0, -1, -3, 2, 6, 5, 1, -1, 3, 1, 1, 1, -1, 4,
    -5, -3, -1, -2, -3, 0, 2, 2, 2, 2, -6, 7, 7, -1, 3, -1, -2, -2,
    5, 0, 1, 1, -3, 3, -6, 2, 1, 3, 3, 1, -1, 0, 2, 1, -5, -1, -4,
    1, -2, 5, -3, -2, 0, 3, 0, 1, -1, -1, 1, 0, 3, -1, 3, -1, 1, 2,
    0, -4, 2, -1, -3, -2, 0, -3, -2, 0, 1, 0, -5, -2, 4, 1, 7, -5,
    1, 1
];

const ntru_g_1024: [i8; 1024] = [
    3, -1, -7, -1, 4, -2, 3, -1, -3, -3, -5, 3, -1, -1, 2, -3, -5,
    0, 0, 1, -3, 3, 3, -2, -2, 0, -4, 2, 1, -1, 3, -5, 5, 0, -1, -3,
    -1, -2, 5, 3, 4, -2, -2, 0, -4, 0, 3, 3, 0, 1, 3, -1, -2, -3,
    -1, -2, 3, 1, 0, -2, 0, -2, 0, 0, 1, 3, -2, -3, 3, -3, -1, -2,
    -1, -3, -3, 1, 1, 8, 1, 4, -2, -1, -5, -4, 3, 5, 0, 0, 7, 4, 1,
    1, -4, -2, 4, 4, -3, -4, 5, 3, 2, 0, 1, -6, 2, -1, -3, -1, -1,
    1, 2, 5, 0, 3, 1, 3, 0, -4, -3, -3, -1, -1, 1, 1, 2, 0, -2, 0,
    3, 3, 3, -4, 1, -2, -4, 3, 3, 5, -4, -5, -1, -1, 3, -2, -4, -1,
    -5, 1, -1, 0, 2, -1, 1, 1, -3, -2, -1, -4, -4, -1, -1, 0, -1,
    -1, 3, -2, -2, 3, -6, 2, 2, 0, 2, -3, 0, 3, -2, -2, 2, -1, 1, 4,
    -2, 0, 0, 0, 0, 4, 0, 2, -4, -1, 5, -2, -6, 2, 1, 3, 1, 3, 0, 3,
    1, 2, 0, -4, -2, -3, 4, 2, 0, -8, -2, 2, 5, 2, 0, 1, -2, 0, -1,
    0, 3, 3, 1, 0, 0, 2, 2, -1, -3, 3, -2, 3, 1, -2, 5, 1, 4, -2, 3,
    -2, -3, -2, -1, 2, -5, -4, -2, -3, -6, -3, -2, -3, -3, -3, -1,
    1, -1, 4, -1, -5, -3, 0, 3, 6, 0, 2, -1, 5, -1, 0, 2, 7, 6, -1,
    2, -2, 4, 6, 0, 1, 4, 4, 0, 5, 1, -3, 4, 1, 3, 1, -1, -2, -4,
    -1, 0, 0, 3, -6, -1, 2, 4, -3, -1, 0, -3, 2, 1, 2, 4, 1, -1, -2,
    -1, -3, -2, -6, -1, 1, 2, -2, 0, 0, 0, -3, 1, -2, 5, 1, 1, -5,
    -4, 0, 0, -2, 0, 3, 4, 5, 2, -4, 0, -4, 3, -2, 0, -3, -4, -1, 4,
    0, -3, -2, 1, -1, 2, -3, 1, 0, 0, -3, 0, 5, 2, -2, -4, 2, 0, 0,
    -3, 0, -1, -1, -1, -1, 3, 7, -1, -2, -5, 2, 5, 1, 4, 2, -3, 0,
    -5, -4, 1, 0, 1, -3, 4, 2, -5, -1, 2, -4, 0, 1, 0, 1, 0, -1, 0,
    -2, -4, 4, -2, 0, 1, 1, 0, 2, 0, 0, -4, -5, -3, 0, -4, -5, 2, 1,
    0, -3, 4, -1, 3, -4, -5, -1, 4, -1, 0, 1, -4, 5, -1, 5, 1, -1,
    0, -3, -3, 3, -5, -3, 1, -1, 0, 4, -2, 2, 1, -2, -3, -1, -2, -3,
    -3, 1, -2, 8, -2, 0, 3, -2, 3, 3, 3, -2, 4, 0, 2, 3, 1, 3, 0,
    -1, -1, 3, 1, 1, -1, 5, 1, -1, 0, -3, 1, -4, 1, -1, -4, -1, 6,
    -4, 6, -2, 0, -2, 0, 4, -1, 1, 6, 7, 5, -1, 3, 3, 0, -1, 1, -6,
    -1, -4, 4, -1, 3, 4, 3, -3, -1, 2, 0, -3, 2, -1, -2, 1, -3, 0,
    -1, -5, -1, 4, -2, -6, 1, 0, 6, 2, 1, -3, 2, 1, -1, -1, 4, -4,
    -2, -2, 5, 1, 5, -2, -6, 1, 0, 1, 4, 4, -3, -1, -4, 1, 0, -1, 0,
    -3, 4, -2, 4, 3, 1, 4, 6, 2, 1, -1, 1, -5, -2, -3, 2, 0, -1, -1,
    -5, 3, -3, 2, 0, 1, -4, -1, 7, -1, 2, -2, 3, 0, 0, -4, 4, 2, 4,
    1, 1, 2, 3, -2, 3, 1, 0, 0, -4, 0, 2, 1, 1, -1, 4, 4, -4, -3, 5,
    -1, -1, 1, 3, 3, 1, -9, 0, -6, 5, 0, 0, -2, 3, 0, 2, -2, 3, -6,
    1, 1, -1, 3, 1, 2, 0, 3, -1, 0, -2, -3, 7, 1, -3, -1, -2, 3, -3,
    1, 4, 3, -3, -3, -4, -6, -5, -1, 6, -5, 3, 0, -1, 0, 4, -5, -2,
    1, -2, 3, 6, 2, -4, -5, -6, -3, 2, -3, -4, -2, 1, 1, 0, 1, -2,
    2, -2, -1, 1, 2, 2, -4, -3, -1, 0, -1, 2, -4, 1, -1, 0, -3, 1,
    2, -5, 4, 0, -3, 3, 5, 3, -4, -2, 2, 1, 1, -4, -2, 1, -1, 0, 0,
    3, 0, 1, 0, 1, 1, -1, -4, 3, 3, 1, 0, 1, -1, 1, -2, 3, 3, 5, -1,
    3, -3, -1, 1, -2, -1, 0, 1, 1, -2, 0, 3, -1, -3, -2, 1, 1, 2,
    -7, -2, 1, 5, 0, 0, 0, -4, -4, 0, 1, -1, 4, 0, 0, 3, 1, 3, -4,
    7, 4, -4, 0, 5, 4, -3, -1, 0, 0, -3, -4, 3, 0, -1, 2, -2, 0, 6,
    0, -2, 3, 1, 6, 3, -2, 2, 1, -1, -2, -4, -3, -2, -2, 0, 2, 0,
    -4, -3, -1, -3, 1, 0, -6, 2, 0, 4, 4, 1, -2, -1, -3, 3, -4, -4,
    -2, 1, -5, -1, 2, 1, -2, 0, -2, 2, -1, 0, 3, -2, 1, -6, -2, -1,
    0, -2, -2, 1, 0, 4, -1, 8, 3, 0, 1, 5, 1, -3, 0, 2, 1, 1, -1, 4,
    0, 4, 6, -2, 0, 0, -3, 5, -6, -3, 5, 2, -2, 1, -1, 6, 5, -3, -4,
    -3, 2, 3, -5, 2, 2, -2, -4, 6, -4, 2, 0, -4, 5, 2, -1, 1, 0, -2,
    2, 2, 0, -3, 0, -7, 0, -1, 1, 3, 3, 2, -5, -2, 0, 5, -4, 1, 2,
    2, -1, 4, 5, 2, 2, 2, 0, 0, 2, 1, 3, 2, -4, 4, -2, -1, 2, -2, 0,
    3, -2, -1, 6, 1, 3, 0, 4, 0, -2, -1, 1, 0, -3, 3, 2, 3, -1, -3,
    -3, 3, -2, 3, -2, 0, -1, 3, -3, -2, 1, 4, -4, 2, 5, -7, 1, 0,
    -5, 1, 2, -1, 3, -2, 3, -1, -2, 2, 0, 0, 0, -3, -2, 4, 7, 1
];

const ntru_F_1024: [i8; 1024] = [
    36, -13, 88, 1, 13, -66, 9, -2, -12, 30, 23, -18, -15, 8, -2, 5,
    19, 13, 14, -22, -22, 29, -18, 8, -45, 28, -4, -46, 30, 40, -26,
    -3, -1, -2, -54, 4, 34, -38, -32, 55, -25, -24, 4, -35, -9, 8,
    23, 24, 35, 17, 29, 7, 41, -3, -13, 39, 25, 24, -34, 18, -55,
    22, -61, -23, 18, 3, 1, -3, 32, -20, -2, -63, 19, 21, -13, 3,
    -7, -17, 5, -40, -29, 8, 6, -43, 27, 31, 3, -28, -46, 76, 31,
    -16, -4, 59, 38, -6, -6, -3, 34, -19, -10, -39, -20, -15, -3,
    -11, -41, 9, -90, 21, -26, 24, -2, -18, -36, 18, -2, -4, -18,
    -67, 14, 7, -22, -22, -28, 42, -57, 32, 7, 25, 7, 30, -45, -9,
    -2, 8, 28, -14, 19, -19, -47, 37, -34, 45, -32, -8, -35, 52,
    -31, 35, -14, -10, 36, -65, -16, -21, -5, 4, 21, -61, 22, 13,
    -55, -5, -22, 14, -10, -35, 8, -5, 27, -31, -32, 3, 0, -12, -25,
    27, -5, -22, 3, -6, 0, 21, -5, 45, 36, -42, 16, -2, -9, -16,
    -44, 0, 44, 3, -9, -51, -32, 11, -4, -7, 33, 15, -9, 13, -6, 15,
    15, -30, 10, 14, -8, 27, 20, -3, -10, 22, 14, 7, -15, 31, -17,
    -20, -14, 4, -2, 26, 27, -7, 32, 49, 27, -40, -4, -35, 11, 3,
    17, 35, 11, -14, 35, -6, 7, 10, -38, 12, 43, -42, 44, -20, -3,
    33, 35, 14, 1, 10, -9, -11, -20, 31, 30, -2, -2, -6, 17, -10,
    -10, -10, 49, -23, -18, -3, -1, 12, 19, -44, -21, -13, -19, 25,
    4, -23, -20, 6, -15, -2, 21, 19, -6, -4, 43, -24, 31, 22, -15,
    55, -5, -3, -8, -6, -14, 23, 0, 32, -28, 48, 55, 43, 2, 17, -4,
    56, 58, 32, 3, -6, 43, 11, -3, -9, -26, -17, -6, 23, 29, 30,
    -31, 5, 27, 11, 9, 30, 32, 66, 10, 33, -40, -26, 19, 38, -11, 4,
    12, -57, 30, -14, 29, -5, 12, -9, -17, 20, 40, -1, -19, -5, -16,
    -38, -25, -24, -1, 21, -21, -47, -1, -8, -53, 39, 7, -36, -19,
    4, 0, 2, 16, 6, 20, 8, 8, -7, -42, -1, 4, -17, -7, -10, -24, 0,
    -29, -1, -15, -2, 3, 9, -21, -2, -20, -15, -68, -42, 21, -40,
    -23, -3, -5, -18, -2, -17, -20, 17, 10, 16, -54, 27, 5, -21, 21,
    4, -54, -25, 2, 61, 39, -25, 47, -5, 20, 12, 8, -21, -12, 16,
    -13, 23, -4, -47, -12, -15, 5, 20, -4, -22, 37, -43, -14, -30,
    -21, 15, -24, 1, -13, 25, -3, -8, -4, 9, -13, -1, -14, 39, 19,
    -8, -11, 45, 32, -25, 18, 77, 13, 14, 21, 38, -42, 15, 28, -17,
    6, -2, 17, -36, 21, -33, 13, 12, -6, 36, -5, 9, 7, 1, -17, 30,
    -16, -3, -39, -12, -6, -21, -28, 3, -13, -17, 29, 23, -12, 17,
    35, -18, -10, 1, 26, -33, 69, 57, -12, -15, 43, 18, 27, -31,
    -29, 37, -5, 50, -56, -22, -57, -1, 21, -15, -27, -48, -20, -28,
    -4, -31, -20, 9, 10, 15, 12, -7, -38, 23, 0, 9, 3, 15, 28, 31,
    -8, 4, 19, 3, -7, 30, -14, 27, -7, -25, -20, -10, -20, 27, -6,
    -30, -8, 27, -5, -23, 52, -44, -33, 48, -20, 10, -21, 7, -34,
    -1, 26, 40, -11, 4, 46, -30, -13, -9, 27, 13, -13, 13, -4, 11,
    34, -32, -3, 51, 24, -45, 39, 14, 15, -55, 12, -28, -21, -14, 3,
    -16, -25, -13, 35, 18, -7, -27, 51, -16, 29, -28, 5, 1, -32, 3,
    -25, -7, -15, 33, 8, 37, -20, 26, 25, 12, 13, 15, 8, 4, 11, 8,
    -31, -1, 8, 13, -31, 22, -7, -3, 7, 12, 10, -12, -62, -49, -12,
    -5, -3, -53, -30, -7, -56, 20, 45, 6, -46, -32, -15, -13, 9,
    -18, 11, -5, 12, -6, 10, -62, 8, 11, -18, 27, 16, -5, -6, 4,
    -28, -6, -30, -58, 11, -8, -40, -51, 20, 27, 17, 12, -9, -28,
    -11, -8, -22, -18, -16, -36, 14, 17, -44, 2, 42, 16, -9, -31,
    -16, -3, -14, 41, -22, 16, -1, 32, 12, 4, -36, 18, -41, 13, 31,
    -35, 14, 8, -10, 19, -9, -4, -36, -15, -62, 0, 16, -28, -54, 4,
    20, -9, 14, 18, -26, -30, 19, 10, 10, -5, -10, 7, 23, -7, -31,
    -2, 19, -63, -14, 0, -18, -3, -21, -33, 11, -52, -21, -4, -48,
    -35, -6, -4, -14, -13, -1, 14, -16, -21, 9, -37, -31, -16, 4,
    -17, 2, -4, -17, 2, -51, -16, -16, 3, 59, 13, -16, -30, 17, 9,
    35, -11, -31, 32, -7, -26, 28, 7, 19, 28, -17, -49, -30, -9, 23,
    19, -2, 1, -3, 9, 48, 11, -54, 9, -22, 25, 8, 22, -52, 37, -14,
    -10, -30, 20, 52, 3, 10, 17, 56, 33, -3, 41, 53, 41, 4, -7, -25,
    8, -45, 29, -11, 5, 29, 60, -15, -8, 14, -7, -33, -14, 6, -12,
    -2, -2, -10, -12, -16, -21, 2, 5, -14, 53, 41, 61, 12, 31, 4,
    22, -16, 36, -4, -42, 38, -29, -10, 20, 20, 35, 66, 16, 12, -50,
    -5, 1, -16, 32, 33, 46, -3, 11, 11, -19, 28, 5, 38, 15, -35, -6,
    1, -6, -17, 2, -28, 25, 42, 8, -2, -11, 14, -33, -42, 23, 21,
    -31, 63, -8, 15, 26, -16, 13, -3, 39, -16, 58, -14, -14, -22,
    -30, -26, -51, 4, -9, -18, -5, -56, -48, -60, 10, -52, -28, -6,
    -12, -35, 31, -22, -31, -13, 8, -14, -8, 23, -20, -45, -12, 30,
    8, 7, 9, -12, -13, -2, -29, 18, -1, 30, -17, -26, -41, 58, -66,
    -6, 8, -78, 25, 29, 0, -40, -27, 16, 40, -15, -28, 18, -54, 16,
    -12, 1, -4, 17, -21, 12, 30, -7, 45, -8, -28, 8, -41, -1, 42, 3,
    -6, -47, 22, 44, -78, -45, 41, 12, -30, -23, 13
];

const ntru_G_1024: [i8; 1024] = [
    56, -4, -28, 13, 4, 26, -14, -10, -21, -32, 21, -21, 0, 59, 26,
    -6, -81, -7, -42, -1, 8, 20, 37, -33, 36, 1, -6, 14, -1, -73,
    15, 18, 7, 34, -45, 7, 19, -30, 8, 8, -56, -13, 7, 20, -20, 14,
    4, 11, -36, 21, 19, -16, -17, 10, -45, -4, -12, 18, -9, 23, 20,
    1, -27, -7, 1, 1, 33, -27, -23, 56, 35, -5, 16, 43, -1, 11, -18,
    5, 59, 14, -39, 11, -34, -59, 6, -18, 43, -25, 22, -7, 9, -28,
    -9, -40, 47, 0, -12, -22, -12, -44, -17, -12, -10, -5, 4, -20,
    -10, 25, 38, 2, 0, -17, 14, -16, 6, 9, 7, -29, 11, 48, 14, 10,
    0, 24, 4, -5, 56, 20, 1, 35, 62, 7, -31, 36, 36, 24, -19, -33,
    19, 9, -13, -16, 2, 1, -29, 3, -3, 42, 8, 27, -19, 51, 16, 14,
    28, -1, 24, 3, 1, 47, -13, -43, 9, 17, 3, 21, 26, -19, -6, 4,
    -36, 7, 26, 19, 40, -44, 27, -20, 9, -1, 19, -1, 0, -51, 5, -5,
    -45, 11, 26, -1, 1, 39, -6, 14, -14, -34, -29, -6, -15, -23, 24,
    -7, 16, -51, 55, -30, 2, 13, 1, -37, 33, -29, 11, 14, -44, 46,
    40, -45, 59, -6, 3, 18, -46, 20, 17, -2, 0, 27, -20, 1, -7, -16,
    -29, 0, -2, -19, 19, -10, 16, -15, -2, 2, -10, 4, 27, -4, 2, 0,
    15, 35, 19, 25, 21, 32, -9, 26, -28, -23, -2, 7, -24, 75, -3, 6,
    21, 45, -8, -12, 21, -12, -29, -7, 34, -13, -22, 28, 32, 20, 18,
    -22, 2, 17, -12, 37, 37, -33, 5, 28, 27, 55, -30, -9, 10, 23,
    12, -9, -26, 30, 5, 2, 24, -37, 9, -25, -50, 33, 3, -21, -38,
    -19, 44, 41, 9, 1, 34, 6, -26, -44, -26, 15, -3, -21, -24, 40,
    -43, 28, 2, -30, -13, -8, -40, 1, 31, -2, -16, -16, -25, 33, 1,
    -23, -51, -40, -37, -12, -38, 78, 15, -31, 32, 41, 26, 26, -25,
    -60, 15, -43, 27, -54, 0, 25, -7, -27, 15, -18, 20, 20, -76, -3,
    35, -39, 20, 28, 21, 10, 6, -41, -29, -31, -55, -41, 0, -8, 4,
    -17, 21, -21, -12, 11, 4, -17, 6, 26, 11, 9, -13, -41, 29, -7,
    -4, 21, 12, 2, -1, -23, 3, -5, 1, 30, -10, 9, 40, -63, -27, 51,
    -29, 4, -1, -7, 30, 30, -21, 32, -17, -51, 28, 34, 22, -5, -5,
    42, 7, -1, -16, -15, -23, 10, -9, -3, 1, 26, -12, 16, 5, 18, 13,
    -1, 28, 8, -52, -42, 12, -3, -28, 30, -3, 11, -14, -3, 34, -75,
    -13, 6, -39, 14, 72, -16, 18, -17, -36, -3, 5, -9, -38, -42, 4,
    7, 3, 74, 2, 9, -23, -32, -43, -59, -15, -4, 31, 12, -16, -24,
    35, -7, 0, -8, -3, -8, 6, 1, -57, 13, -8, 22, 31, 40, -16, 35,
    34, -32, -60, -21, 8, -1, -13, -39, 14, 17, -7, 33, -35, -6,
    -18, 0, 29, -5, -9, 40, -19, -45, 24, -41, -7, 32, 19, 28, 28,
    -17, 10, 18, 6, 29, 1, -1, 17, 28, 18, -22, -16, 20, 51, 8, 2,
    18, 16, -9, 40, -8, -20, -31, 11, -5, 34, -25, -40, -48, -9,
    -36, 4, 17, 15, 8, -18, 27, -24, -25, -13, 22, 21, -21, -25, -2,
    -32, 7, 20, -8, -26, -23, 21, -3, 34, -15, 0, -17, -21, 56, -38,
    -2, 18, -44, -14, 31, -2, -24, 3, -18, 18, 16, -24, -35, -48, 5,
    -36, 28, 15, 13, 32, -14, 14, -38, -9, -62, -11, 6, 6, -26, -46,
    72, -63, -35, 16, -4, -19, 45, -20, -28, 1, 26, 36, -35, 30,
    -13, 39, -29, 16, -30, 14, 5, 4, -11, -6, 22, -9, 5, -9, 14,
    -27, -30, 23, 23, -2, -7, 30, 0, 6, 32, -43, -17, 11, 10, -29,
    -15, 111, 15, 10, 13, 7, 16, -2, 3, -3, -6, 38, -10, -11, 5,
    -34, -2, 14, -32, -21, 4, -5, 0, -8, 27, -50, -7, 23, 8, -17,
    -63, -87, -1, 34, 5, -1, -11, 3, -6, 33, -9, -45, -34, 2, -23,
    46, 43, -11, 12, -21, -7, 17, -48, -42, -3, 7, 2, 24, 8, 71, 6,
    0, 9, -16, 6, 17, 20, -2, 6, -44, -13, 44, 23, 27, 13, 37, -10,
    58, 8, -12, -4, -15, 14, 27, -47, 18, -68, -6, -9, -16, 2, 7,
    -15, -47, 34, 1, -38, -25, -19, 1, -10, 14, 7, 9, -20, -1, -21,
    4, -13, 18, -26, -10, 31, 51, -59, 15, -3, -19, -70, -17, 46,
    -14, 24, 32, 15, 34, -23, -36, -16, -7, -10, 2, 36, 7, -12, -51,
    -25, -8, 23, -18, -17, 7, -48, 9, -26, 44, -25, 44, 32, 28, -10,
    11, 12, -17, 4, 16, 6, -19, 32, 22, -31, 16, 8, 31, -4, 17, -21,
    1, 56, 36, 21, -31, -4, 0, -46, -26, -44, 61, -61, 2, -21, 45,
    15, -14, -35, -13, 5, -38, -10, -19, -7, -7, -33, 33, -11, -12,
    29, 17, -27, -46, -48, -25, -13, 6, 25, -12, -12, -29, -58, -2,
    -29, 5, -11, -15, -19, 32, -58, 14, -35, -3, -20, -16, -32, -24,
    -45, -18, -3, 0, -1, -43, -9, 12, -29, -2, 1, -9, -26, 5, -2, 9,
    -17, 32, 27, -3, -27, 2, -7, -13, 4, 6, 46, 38, 28, -27, -3,
    -19, -38, 64, 13, 9, -16, 35, 46, 8, -80, 15, -16, -19, -26,
    -10, 48, 8, -11, -8, -2, -16, -22, 50, 9, -14, -52, 39, 11, 49,
    2, 5, -11, 13, -4, 10, 11, -23, -23, -10, 14, 31, 42, 18, 0, 49,
    34, 19, -25, 15, -41, 30, 8, 18, 29, -6, 15, -17, 5, 30, 2, -19,
    12, 43, -32, 31, -39, 8, 21, 16, -12, -8, 24, 37, 8, -13, -54,
    0, -44, -12, -26, 31, 5, -22, 51, 11, -7, -22, 27, 17, 12, 20,
    -8, 9, -11, 48, -21, 9, 24, 2, -4
];

const ntru_h_1024: [u16; 1024] = [
    6857, 4524, 6980, 4278, 8521, 9214, 399, 11461, 10346, 9318,
    2768, 1272, 396, 5635, 2424, 3623, 2071, 9145, 8766, 3391, 4900,
    10525, 6985, 3336, 5084, 11240, 5203, 4517, 7825, 974, 6450,
    11728, 3727, 1190, 1946, 2049, 10661, 1728, 1985, 8185, 9223,
    9410, 912, 7934, 2032, 377, 5915, 4515, 2076, 4073, 2109, 6749,
    7387, 4215, 11826, 2227, 4483, 10801, 6125, 5658, 4570, 11193,
    3749, 10418, 5800, 7159, 310, 2683, 453, 5616, 5109, 11698,
    1957, 11587, 11845, 11324, 3939, 3660, 6103, 1382, 6149, 6923,
    1333, 7200, 9403, 5990, 10319, 3473, 4771, 3113, 1322, 6743,
    7880, 11899, 3652, 1437, 6650, 8034, 11683, 11315, 2877, 8676,
    9741, 10509, 10783, 10635, 11008, 213, 3492, 8626, 9271, 4771,
    5099, 10209, 6359, 7738, 10860, 5853, 4632, 9862, 5942, 252,
    10959, 11867, 1685, 4189, 5995, 11443, 5328, 10337, 2968, 5209,
    8779, 4330, 7128, 11161, 9016, 12220, 4751, 4429, 7602, 5861,
    7151, 7129, 772, 4128, 8860, 4537, 4010, 1037, 1692, 11048,
    6880, 3575, 11950, 2203, 2670, 3213, 3925, 9414, 6968, 11107,
    11333, 11878, 11390, 3409, 1632, 5743, 11214, 4130, 5383, 10064,
    8173, 11054, 5112, 7029, 2850, 1905, 7673, 3131, 3383, 10728,
    8997, 4775, 6026, 206, 1602, 6698, 84, 8650, 4051, 2857, 1654,
    10234, 3560, 2220, 10072, 4755, 711, 3377, 3830, 1588, 6249,
    9108, 6301, 718, 4896, 4000, 5810, 14, 7962, 11835, 1619, 10486,
    3549, 3698, 4990, 2693, 2161, 7265, 7865, 10853, 5758, 532,
    9771, 9170, 9181, 10730, 943, 12226, 10180, 12238, 7892, 740,
    1961, 2829, 676, 5308, 8939, 9874, 11816, 8850, 2977, 2717, 535,
    11753, 2410, 4915, 8862, 3229, 87, 8181, 6423, 2900, 7322, 2728,
    11030, 9252, 660, 8255, 5084, 9638, 9905, 300, 10871, 8115,
    3921, 10583, 8542, 11888, 8610, 12287, 8303, 4353, 1249, 8437,
    4800, 11879, 11321, 226, 394, 3180, 8426, 8815, 11081, 10343,
    5780, 3187, 8139, 8663, 3271, 8829, 3961, 8000, 10424, 8944,
    8953, 8797, 11506, 9527, 5086, 3654, 2990, 2609, 9285, 10676,
    2126, 8322, 8388, 2907, 1729, 8000, 11960, 10595, 12191, 6735,
    9877, 2994, 5468, 8463, 9718, 2, 1224, 9842, 2220, 7347, 10016,
    5484, 11643, 3603, 4027, 8718, 1504, 5330, 6070, 6726, 10243,
    5581, 11371, 2288, 998, 11901, 9880, 8241, 3448, 10681, 12014,
    564, 4560, 6851, 9235, 5722, 10116, 12008, 702, 9412, 1818,
    2166, 2521, 3449, 9976, 1317, 8202, 2198, 2879, 9909, 8232,
    4358, 9168, 2723, 6954, 7861, 1599, 21, 768, 2337, 8793, 3970,
    3427, 6800, 3319, 10882, 3474, 11336, 6751, 2944, 2916, 2479,
    6692, 10943, 1129, 10958, 9778, 4105, 1160, 1789, 4091, 11799,
    9847, 4003, 7156, 6, 7913, 5539, 930, 5480, 4256, 4410, 11099,
    7276, 858, 2462, 7336, 7936, 10066, 9162, 1859, 4488, 7896,
    9090, 433, 11723, 12017, 2998, 2568, 7557, 3811, 5754, 6153,
    2684, 6011, 745, 3994, 10864, 26, 6792, 4045, 12039, 8465, 9526,
    3324, 4640, 2950, 11189, 11560, 10968, 3467, 9643, 3390, 3105,
    11058, 10171, 7104, 2221, 2882, 10809, 7349, 5094, 2277, 11876,
    5610, 9833, 11556, 9212, 1093, 4875, 1258, 6525, 1379, 6528,
    8903, 6134, 6783, 3749, 7107, 7032, 6685, 5443, 5050, 10430,
    12242, 11782, 9733, 1619, 3481, 11713, 8961, 1719, 6641, 2111,
    578, 6688, 317, 11397, 6930, 6025, 12121, 9635, 11230, 6031,
    4549, 10556, 5810, 915, 9655, 9997, 9300, 11651, 3638, 6822,
    2744, 6442, 11872, 4596, 9029, 6843, 7691, 146, 8712, 5846,
    9537, 2265, 8123, 10530, 3748, 10109, 4081, 8648, 6032, 243,
    5257, 7387, 3266, 11825, 9483, 5681, 8116, 6123, 5512, 1832,
    12174, 6471, 5779, 9994, 5717, 3438, 10887, 7041, 5482, 5776,
    5052, 1277, 113, 1592, 397, 11378, 8540, 5539, 2871, 4794, 3498,
    6522, 9072, 11037, 7679, 7192, 3190, 4453, 10689, 7319, 4307,
    9129, 611, 8733, 7051, 2177, 4032, 400, 8267, 10724, 6343,
    11699, 2709, 2348, 9276, 199, 8140, 1116, 4362, 10542, 7509,
    4463, 3631, 8311, 10476, 4188, 5615, 541, 11358, 10783, 2653,
    5769, 7483, 233, 7151, 7427, 6954, 8993, 8388, 2634, 2867, 8188,
    10115, 9728, 10919, 2716, 2602, 10405, 8148, 7446, 4372, 4018,
    7482, 9473, 6256, 743, 11460, 2574, 10848, 7890, 6186, 277,
    8438, 6377, 9923, 1538, 8021, 1912, 2915, 8575, 7640, 1224,
    6644, 7740, 8476, 7237, 8105, 8741, 8510, 2956, 7727, 5456,
    4878, 6062, 5590, 4643, 2031, 12027, 7298, 11101, 423, 11766,
    6001, 8391, 11344, 7039, 925, 12087, 4715, 7775, 2577, 12100,
    10177, 7515, 8393, 7854, 7358, 2549, 8037, 7490, 9570, 11450,
    7333, 9792, 6517, 6246, 6324, 5652, 4287, 2916, 10244, 6843,
    1032, 10596, 10106, 412, 1649, 796, 787, 9141, 8210, 11119,
    8722, 5557, 8075, 10581, 5333, 646, 11955, 8382, 10460, 249,
    10396, 1425, 10106, 9653, 2083, 9412, 9482, 10358, 5238, 4889,
    10735, 5696, 10184, 10472, 8912, 4715, 4135, 1850, 6734, 788,
    7911, 12091, 5068, 2196, 8528, 1228, 7261, 6719, 5451, 22, 7250,
    5546, 3002, 1364, 3609, 815, 9625, 6174, 11030, 1121, 5227,
    11682, 127, 9743, 2882, 2013, 6410, 3887, 5624, 6044, 6535,
    10445, 11640, 754, 806, 1728, 10444, 3467, 10796, 1342, 560,
    6412, 9877, 4231, 1004, 7563, 6241, 6650, 7678, 9513, 9268,
    5824, 756, 6364, 382, 6099, 11818, 3452, 10664, 9035, 1892,
    3627, 7790, 2766, 4747, 10766, 2582, 8247, 1343, 5404, 8999,
    1306, 2323, 1697, 3027, 3992, 10237, 12264, 2504, 12196, 285,
    3341, 1023, 11597, 65, 3054, 7012, 3354, 3802, 6381, 11090,
    7956, 8016, 5817, 3082, 7822, 10676, 10795, 9182, 3766, 1386,
    7018, 9919, 4122, 4277, 994, 957, 10047, 4963, 6036, 1829, 3699,
    5927, 2986, 11757, 1091, 8586, 4136, 5998, 8614, 738, 7930,
    9614, 1038, 3465, 9461, 8800, 448, 4790, 6902, 557, 2952, 10752,
    10914, 3678, 10642, 6378, 4775, 8587, 424, 160, 4814, 2921,
    4061, 1553, 10018, 3960, 5267, 1357, 11229, 9706, 5480, 3599,
    695, 9316, 12014, 8205, 274, 3219, 7113, 2914, 4379, 2025, 3948,
    10733, 2075, 9903, 6767, 7202, 5303, 10864, 11908, 2946, 1579,
    5946, 11135, 3629, 12006, 1620, 6884, 6120, 9640, 3085, 8987,
    6885, 126, 8477, 119, 7775, 10599, 11996, 1533, 6606, 9624,
    7770, 3292, 11468, 11115, 9034, 44, 1631, 3867, 10484, 8799,
    6863, 10298, 6792, 10913, 1051, 3927, 3297, 5283, 7269, 10315,
    8364, 7328, 2905, 9803, 8593, 1567, 7314, 934, 10166, 7739,
    2512, 8874, 11011, 10479, 7844, 3646, 6388, 7202, 9880, 4708,
    1057, 9014, 11431, 6222, 4398, 2374, 6684, 4743, 7242, 8114,
    8995, 1222, 2096, 5121, 10994, 8697, 10848, 10235, 2380, 8095,
    7857, 6687, 8380, 7399, 3751, 5398, 98, 7175, 3353, 3777, 8408,
    10568, 12088, 5008, 10713, 10027, 3345, 11243, 2602, 2825, 9460,
    2218, 5381, 9106, 12214, 844, 7720, 8328, 3552, 7008, 11397,
    6321, 1422, 2261, 251, 11402, 176, 10570
];

const ntru_pkey_1024: &str =
    "0a6b251ac6d110b685263fe063ecc5a1aa4662b404f8063160325e0e27205e3b988f8d3f4c9291d6d24d084f72be8514d1a57a443ce64cadd03a3c4a61e68801a6946c01f05ff9901e4c20e41efe1fc01795c6d1a32070fe920f5a5d736d077b8c88b3460ea315fb561a476abb93a968b25aa1bf704d8a7b07155f04fd6db21e96d43b916c3c3d8ce4c5f5c5666015b0b14d5c2092ed766a13cd914a8cc2914a9a577b22e7b391059d67e9f62b68ec332cf61e4983690da87e98bac000d536921b290dd2a34fae7e1635de3aa9b16dd48626865cd80fcab3ee5b1a5505d5daecb353428612e61459892d0ea6f62b998ce2fbc4a3d14d76c96e56fbdbd90c110208a711b93ea840d1a72b286b80df7bab889b29b8c8d3d564c66ce2b63b116e66b1f8d51198166faf39022541e7507fb6b2e4fe1b752c8877177e4c3b34de9e88c952a75e280ce1909a2a01521ca3f4cb2919da7fa37a08ac9d612930b1cd313bd863461a639462742ce4c80fa05ac800e7c6ae3b194e8f63774e724df8a8521c5c617ae6a6559f821498ae3d28f769ea0ebefc29f12fce7b502e41ea4b0d0a914bc8bae692b8a22922e84a9d085ede925a93338a78c9d015dff5645cb547268aa8ac5a4240a5203f4f725a69ac412ca9ddfb33d46957857ae70868afff81bd10113860f54b02e67b0e40e20628c6c83aa26fad268675a50c737f2e1d7331e27d3de5f40a2e22f08be625db3ca5374f78e462eb8a3191169b4213a0828310b5b1b05f40bae2963be7da4f9a54bb2557210f97d8002132267222b1cb39c8156cb5ece133eee20e17814d25ed9a46a00d5cdb1ac8f00f9ae7d9a6203135e29b9bbb82344741ac3904d65a9e12ee80afa4c41c688762764d799be052580288962cfe6b580a11068f40aa36ca9eb518fc0150c009218964f82358da9033dea82364ac48697cb802d909af6892abf11a6ace98c900912206fd3feee1799dcfa36fd00067ba55a30e89568428113aad6dc6c0d6899e72a1f009d4a3ca1d0d1887b6238206c6dcbbbc4bb62821d853b8d67a6024a7c5dec2e93e6aa700069a883f36f07844653633f12202e1abb5b4a2ad8362e5ab34f8c21acca7bb6f008ad2d0aa3972d53e62396e6457aa669b4923fc111530b13a997d158d9808b1d7f669fcea56f0db7868755434eea8bebf4ae0698146533666dc18c046b767c483f0909a2004f6c856c49789bd665a3af7978f471693c5ac839396de70d9152d8338d9aa62ae192ab9811f48d15abb782c09288216d695048d97eee9223a9277d3fc61c85e400f35225cdb330ae31942d6317ed17eb5620728be399475a4e70a5954d6eaa1db8155a96904ef04fd01c46380636c7285715a32cdd2ba36a997a8dc2b1d77fdc1831d9165a705c97434e3a9098e21d6e2c8813f00190812e9e4631edb32a5492c90f00c77f3045c442a92e755516f38be077a3b105c57bc21db17aa1f297568974ec0e96fbdd036caa3218310a4a2ccdffc9e0e600aa9ca9c28aa8a57f51d164450fb274ea50161c02e7b310a0ea981ed260a811583d98e99b0c6027d547782d8e17f77604c867d1e3c8471c457ea622584f8b8c78bd5504c397ae57592231fbeefb720ab5d069edf65dc60c7b141b7f0e76f3749ade5f2846f449f05d5b8325eae72f89f57d95d42958acba729664065d586662d161442fcb64a011abb10229649de819c19c431c0c4e3b5804ab6f88495b57e2e9555354286bace0bea3700f9a2705919dea5b5208e4c4942a87651d9319a7bd6409f228e88b4126b409c73a69383147b9ef3b4f3089485404cc7175a3f552c01671495aa2ee8554386432f966581eac5846151aeda201fe60f2d087dd6428f2f57e179c661e8cdb5e02f20c986c0a330d8ba8b053e08c190c9a550870fb1d8b61859fa77fa52990d16c00bd18dc05f97d3b8a8d7ca6a234b1d90e2b79b8ace4a2ea0e285a03714fd51c8c9c51a244c6a12f4cf989ff6fe82722fa40474d0d0ffed4d0104bee6d90d1a3b698edad49f147d416b93029e8ea6d2a2b8f78eb615a9b6a9afd01a42d43e20ef673f4d8d7941c94e735c9cbaab7b444386290285dba1a60b89efa963840e36264f589801c04ad9af608b4b88a802aa2397a99263a92a7862c1a802812ce2da4fdd18467223de14931536bdd97a9568383c2b79192eee8034112324dbc92d8911b1fa4f6ca7b481b9abda6f70894b7a9c2e842e0862b5ceab7f38b6ee61951ae45fa25a8303631b6b9407e8474077797e967bb705fd673a5987968cdcb332b6b8d2802c197cf1ba3d225f6b3e83a6a22aa1106cf5733854a3719684b82b1ca02d6664b864461f72483a69ed9e3b27422aaac0e8ef7a90e3e63d1c229a612641086336b29d84e44b894668712877129fb28c8c4c620c1401abca1f9a9827fb2531f9f7ac5a1f82f1ce73a9d5160189c073464ec18362948bce1390a76672b3446beb28a8b0993d08aa5416392bed834c78a20883781b60b2158b116388d503eec8a02c294a";

/*
 * Each KAT test consists in three strings:
 *  - nonce (hexa)
 *  - message (ASCII)
 *  - signature (hexa, uncompressed)
 */

const KAT_SIG_16: [&str; 30] = [
    "895f447be01f4cc0587f79397ecd820d752b1876db1197e653d4b28a88d4c0b7",
    "sample 0",
    "04fff00097ffd2ff100026ff5f00ed009ffeab00cdff4bffe60012ffcdff8b001a",
    "cd6a86be9c547f5e19f075f5e64068962893a94027eed40d2e8f751b266d4422",
    "sample 1",
    "04ffd1ff10000000030043ffad012c00b90023ff030095fe5c00710055ff4f002e",
    "0e8abfcaa4fbe1579231543fc504e8a3148ce60bad5606b731839ed669b15f68",
    "sample 2",
    "04002301530048feefffe50097016dffab000b00920098ffdd0063001b0077ff7f",
    "cf8f0c4c78d4f0df639645092bb454c298a8772644daff56f432f14697c56596",
    "sample 3",
    "040136ffe90035ff9cff33fef80082003cff91007c000bffd3ffe900ecff200057",
    "3f2de8a15575f0b4e8c6067ee6215e23ac49b7d1d9b4f9c7e9001e5f072e85a1",
    "sample 4",
    "04009aff4dff9dff39000a0003ff39ffd000570094ffa10086000e0076000a003d",
    "d0215b52167b13b86162d5908e3619ab72af795a7c66d2c3d3267257cdf29216",
    "sample 5",
    "040009002bffdb00580044ff6b0089006a00acff83004cff5a0083006b003cff51",
    "509a2be909412eef92ec16e6f93dfd7573f9de026fa1add893d8f9ef40511539",
    "sample 6",
    "04fff00067000a00620045ff200000ffc5ffd3007200ec0027003cffaf00e6ffa8",
    "a73711a028e954430b4aef317ea1d9f79c7e054a3e5314131f4df6c0e0bbfed3",
    "sample 7",
    "04ff6affd0ff8b00f600710028006a00560029004f005c0045ffcb0047004900e9",
    "546283756c4ca279efa5bf83c7a3a1cb0d8c68b925278f2ec27c7b7ad5b6ded7",
    "sample 8",
    "04ff7b0059ff260019feedffb9000dffb6ff87ff7f00c7fecaff5a002dffc900a5",
    "3af6b128ffc85e5694bc7a8d4809694d7644b186b31d119e1a2082d5054a4489",
    "sample 9",
    "0400e6ff98fee3ffab00b4ff5b00aa0023ffbaffa30088ffee00c7000701990023"
];

const KAT_SIG_512: [&str; 30] = [
    "cd3f225a65b2c6e155c2af799308af940212633fa519a4b4ddd22048ff8a7d06",
    "sample 0",
    "0900070058ffd7ff7affd9ff1e0015ff7fffce0038ff1affad0006ffceffd0ff0aff26fefcff9700c9ffec008100470071febd01960023ff58004a0150ffeeffa70150ffd6ff22ffafffef000dff43ffa3012b00eb015cff1bffeaffaa001f00120074009cff2efec0ffc10095fe96009bffc90027002c0017ffe1ffc200090007ff3f0054fefe000a009f0036002e00310128ffaefebbfff9ffc600b4ff4fff31ffa9ffdd01da00a20010fffd010aff8b013cff7dfef3feccffd3001aff92fe3f0042ff35006b0025ff9300b401db0049ff79fe24ffdc001e0061ffe10044fe98ffa2ffff0016ff3200cffeaeff6bff4dfece002effe0fe66ff3aff89ff25ff6afffc007effd4006bff02005b006900c8ff8bfe55003700290071ff5700da005affc7013a006cfff4007d006d00b3004f0004ff18ffecff2eff940070ffccff4c0086fff7ff51ffc0ff7cff1cff86005b0088ff770099ff8d006ffffa00d500abffdfff2700570076ff6c0091ffd500fe006b00ec01c2007effc90083ffeb0043ffbbfee0ff740004fe86ff8cffe700faff49002d00a4ffb3ff7600dffef1007cff6cff93ffb2005e0073ffd3009100cdff6b0022fffaffd5fffcfff9ff92ffb7001301080011ff9200ca0018012b000500f400f7ff74011500b7004dfffcff48ffd20078ff37001e008e01430086004cffc4ffdbff3fff51ff81009b0074008fffd700870031005dff19ff8d000dff87fffdff8aff63fe97015afffafe98005300e5ffb4ff4e006affe000e300f80094ff45fec600c600b400d2ffab010e0000ff32feb1002b003f005c003fff3f00be0014ff29ffd4016aff06006e000afffa0026007d00ae0047ff50ff460078ff36ffd3ff88ffd6ff0500120075009dffc50062009effddfee4ffc80150ff64018f012a007cff78ff83fffb00270071014e000cff1bff6100ff00220039ffd1ff48ffec004f00390079ff5a0067004c0020ffc7fed300c4004bffacfee4ff2000b60030ffe0fe5d00320015ffc8ff30ff20ff46001b00e50115ff3a0034002cffc2ffe200110007ffe9ff56ffdffff4ff250004ffc30028ff3400b3004000c4009f013bffe7009900350038004dff1301b5ffae008fffdb0084ff44ffad008d0134ff500003013effe1ffc2ffb6fee80034feb40179fff5fed4010bff74ff2aff38ff5500a8ffe10038ff140010fe5400ddff1afff300300014ff2b00240016ffb8ffc000b2ff6a0022007a0067ff7100caff7f0087005eff86ff200098feab0003001300d7ff860106ff96004bffb0ffc500e700f5ffaeff00004cffd8003aff8cff80ffbeff6e0066002f0032ff7afec9ff80ffc6fe770128ff4eff8800e5ff74ffeb01650048ff3d009bff95ff35ffad00dcff5302200113feedff94fec9ffc2ffcc0002fe84ffce01c6ff7dffd6ffc9",
    "a1526fcfb875c35db412013937e99fb7796360f3cccfb1a2cbf175c0cb1d6d35",
    "sample 1",
    "0900bcffd1fe860096fea6ff0ffe5bfe10ff17ffb1fff7ff3400daffe5ffa60071ffaefff4017efeea0050fefdfff5ff84fe96ffb3ff05ff91ffa90072ffff00eaff7cff56ffcefff2ffb5fe8cfff1ffd60011ff4aff9500740024ff27010c004d004801720098ffe80088ffe3ffb7ff84ff24003500c4ff1000590078008f0068ff75ff5300f90006008c0074fea400a700860119001c00300079ff6dffdb007700f700c40007013dff8bfeec0024ffaa00420199005cffc7ff25ffcd000f00ecfff9ff39fff90088ffab00b9ff66fe0fff960097fed8ff77ff8c000800100073ffb0ff4ffe29ffe7ff56ffb20007005c0012ff4bfffa0097ff7bffbcffc4000100d100290058fee000f2ff02fffc00abff2affe6ff7dff7ffede00330049fffe00a8ffd4ffc400af0035ffe6ff42fe75ff31013a001400f6012f0030002a0019fe770152000700b60030ffce0079ff69004dff930116fe57ff89ff5500dffedc0058ffbd0065ff6b00e5ff620076ff50003aff5bff4f000e00c0ff8eff3000ebffdfff3dff74ff59002bffe2ff7afffd00890099ffb4fffd00100000006dffbe0058fe85ffd4006cfef1ff4900a4ffed00a900e0ff8900740085fff5000f012b00b00014ff18ffbe00830045ffb8003f0050ff1affb50090ff27ff9500150001ffb800940007ffddff2700a7ff9500fb00b3ff6dff9300d7ff74fff7fec3fe50000e002d00adff3e00b8010c001c00440016010a00b1011eff65003500f6ffc4ff4d00e2ffabffb00056ff480047ffa3ff8bffc20066ff4affdfffe7ff3c007ffff2fe870068fffeffa40058feca0012ffed0022ffb4ff76008a005000a601f7ffdb00e400660098fe61ffd6002f00d7fe34005f000f0013ff23004e005fff2c0034ffa3ffb5ff070029ff64002a00f0ff24ffb2ff7c00920072fff9fff8ff79ff23fe5dff1c00c100b4ff44008f005c0086ff3e0050002fff3500daff93ff7effed012c008effba0195ff60ffbbffbf002fff210129ff6d001500daff66ff7b005a0095fffb009bff2eff5f0032ffa0ff1c00ddfef800d0ffaffedd0164fe8affc7ff98ff0aff20ffdaff8300b2011affa90092ff5dff7eff760050ff76ff0cff15009effeeff94fff0ffd2fff6ffac010fff6afefaff1dff16ffc60037ffb1ff9effaeff2b008700a6006dffa8fff4ffddff89ffc8ffb30041fe97ff27ff16ffb400b3ff83ff2effcaffb900b900caff5a00b7ffc000330006ff350054ffa2ff90ffcd0014000e0089ff65ffc80025ffdcff00ffe7fed0006200e00082ff34ff700093ff61ffe6fef9ffe60119ffecffb20021ffe40012013000a4ffc1ff2cffdbffc9ff7b006b0147ff91ffd5feea0040fec30015001f00bfffe6febe00d400cdffbafed6fff9fee0ff630044006100b30065007901380084ff8000da0009",
    "0c48305764bd939fcac5f86b9e40a18bdbc7c26fd28d875a24328cd2dac3a224",
    "sample 2",
    "0900060003ff940042ff8900cd0100ffd1ffe5ff64ffcf017c003500c9fea60004ffa0004aff42fe95ffb7ff3e007d0035ffb4ffa700aa004100baffd3ff280161ffd9ff940010003b00680020007000c7ff50013cffe9ff63005f004affd8fff1ffaf000bffa800280120ff8b00d2011fffe00071ffb00137ff5300b70012fea8fed20068001eff2b0014ffa4ff64008fff32ffeaff310063ff1900210029002e003c0070ffbcffadfea4007c0106006d0071ffbe007a0018fed4ff48001eff8affd1006900a8ffd2009a00edff11ff43ff01005fff42ff65ffe8ff8fffd5ff18ffc500c901180179004dffc10056002c0020ff7fff6c0032fff200f300c3009bfeff0137ffb900f1ff1effec0019fffb00bd0022007c009300ec00d6ff9cff5e0010004d00da023cffffff41004b0036ff6c00b4ffef00b1fed401440044ff1b00f60078004300cf00d00067003fff51004e008f00d5fec000e7000e0062ffbfff8cff20ff9aff9c008e007c0038ff44ffd00156018c0060ffd6ff74005d000600a4ffaf02a50096ffacffbf005b00bb00c10000fffcff54001aff54ffc2001900edffdb004a002701480135009100d8fefd007cffdfff95018200cc0039ff2a010cff81010bfefc00a0fff80096010fffdc008b0036ff7eff4cffcfffbeff810058007100d7fff2ffec0021008aff7d019dff7c00cb0013fec3ff7e0098fee1000801c3ff9800c300a500a2005a012fff7500a4001e004e00a00045fef8feb801afff0800e300f6013d0098ffff007800a8ff00fefe0105ff0fff77ff2d001bff7a00a900150095ff100011ff99fef700c1ff51ffa50141ff5affdd00b2fe34fff9ffec0013fef2ff2eff91ffdf0021004f005f005eff8b0015ff64ff44015eff4affea00ff002dfe56005dff02ffe70088ff4a00a5fef9ff5fffc50032ffc400eeff8cfee80111feb5ffb5ff9fffacffcfffbdff0700370042ffddfee70079007e009bff26fe2cff3e00e3ff73002401370056ff9700f6001b0016fefc00aaffc2ffe30005012b0059ff81ffe10005ff7200ea0026017aff92ff7a001d0055ff88ff97fffa00a1003c01a300790157003aff62000e009401460077ffbefff8003d001701bc000dfff90027ffb100a0ffa000f2ff21ffabffd80114001600b7fecfff20feb4ff33005900a0fef3fedeff9e00de007eff0aff84fff7fedc0113ffaeff9200d1ffd4ff650086ff49ff87ff57004a0065feffff82ff5bffa50143009b0097ff81fee0015000000127ffed005f004fffebffb1ffdc009bff190088ff790138fff700b30157ffe0ffbefee40083ff830015005e01560146ff4f0026001700fcffef00950069ff4dff43ff30fe51ffea001a0093ffc7ffc6ff81fffaff79ffd40077ff64001eff40ffb1ffb90042ff41ffce01490066ff9f00a00020",
    "782658797c488ce4f873262ce6a9e78561410c258c60c268c3f4be762578a8fc",
    "sample 3",
    "090065000f00c100fbffa7ffd7ff3e000e008dfff0ffc3ff4dfef3ffb400b401bbfeb4ff12ff5800fb00d3ff5eff0dfeb80095fe9600cdffccffe8fecdffa2ffa4ffe50045ffc70078ff40ffb800340063fecf003900eaffa0feed0077002800aaffcdff470010ffe000edff0800e9ffb00093004efead00e2001bffe70012ffaa006e007a011e004100adffff00aa0144ffb7018c0032005eff620069ff92ffa5ff5b0093001b00ec0113008bff1d00f80057ff6e00d30022ff32ff2d002d0013feb70136005d009200530018009dff5900d70064fed1ff96ffb900a6008bffcdffe20062ffe700effedaffe9ffa1feedffb6ffe40040ff3300530146004effec00a8ffdb003d007801000012feab005fff72ff6b00da003effde00c6ff510051ff5b0059ff1bfff3001dff7900fd011dff4ffeba009ffed7ff6d0080006dff9b00370082003f006fff37007400210029ffe5fedcffa0ff2ffff700910130003bff3100e300640062fedefff9ffef00640009ffdd00ae00a1ffe0ff61ff91000d008c009b002500b8fecdffe7ff1fffed005bff7eff73ffa5fffcffdaff9000ccff8e004a003f0052002100b3002a010e0006ff15ffc70121fee5ff7eff91ffbcffb2ff5c004c00aa00b2ff3c007fff8dffdcff830024003ffe45ffb7ff930103ff2d00aa0088004afea8fffcff30ff35019a013c004dff41ffbc002100dd0131ffd6009aff40fea500880133009bff3100830023fef100a4ff86019aff4fff77fff7ff2cff3100160059ff82ffdf001dff6affb200d5fff60094ff54ffd90029ff6500beff98ff6c005bff29fefdffb5ff58004500a2fea9fedd002400abfffd016e0007ffedffcefffcffa00086fe2efeea0108ffb30051000100680016ff01fff200aa000eff35fffe00d00199ff5e000cfef0ff17ff040061ff8800fa002b0016ff150018003a0034ffecff9eff83012d01290035ffeefff7ff76ff6e00ff0072ff53ffaafff6ff81ffa600d10030ff82ffc0ff91002affdb002500770069fed1003d00c80033ff6a00a7ff440060ffcd015bffdb015000d300b3ff94ff59008affeaff4e005b0191fff6001f00c900cdffbbff1cffa5ff670090ff230037004e006300310116ffddffbdffc800ecff59ff90ffb10024ffe2002f00b30065ffaaff3d015c009ffedd0014000bff4b006700a20063ff51ffc0014bffc00081ffaaffb6ff90005c007c0037000e0122ff0cff3bffc7fff10058013dff91ff62fffa020bff1600880074ffec005c0028ffe80044ff97ff64012a001efff400f7ff3e000e0005fe600007fee2ff4aff13009700ef010400f00064fec8004e002600920043ff1a007c003200eb0048003dfedaff4400510013ff4700580016ffa3feb201b0fef0fede0048008e0042feb2ff7ffff9000d00b7ff9f00d6ffd600f3",
    "ede0d5c1beeab1de0cf3eb392c3fa53a184ceb8e3745bb9dbf8a13ed01aeef7d",
    "sample 4",
    "0901390087ffc4001effd6006a004a0089ffb3fedeff8200360071ff7bffe4fefbff8e00d2ffc3ffc8feb7ff34ff24ff15ff44ff880042004f006afefcffd50109ffc9ffbfff76ffd1ffc4ffe8011fff400128003fff760025ff55009300a100a20018ffc60077004c016eff2bffe4018e002d00b9009b0027ffd5ff1f00de0019ffb0007cff94ffde00b1ff6fff26017cff10ff75ffc6fe62ffce010efed7fff40069ffe1001e00a6ffd40011ffbc000e013fff91ffdcff51008500a500c7ffffff4bff9f0021fff3002dffe7ffc8ffa3008500550092ff1cfff7ff60011600a8ffdfffebfff4019f00dfffb4007b016affb000900129015100b5ff7d000f00b800ebff7fff6a00c70043003a00c3ffb000e1ff78006a00fdff84ff65ff68ffd8ff22ff81ff28ffc60075ffec0040ff59ff44ffbb00a4ffe100000026fedffefdfef2ff67ffc80016ffd5004efe2e01ebffeb0012002cfe3dff46ffd4ffecffaaffdfff46005f007e001e01d5005fffd3ff4c0062fff0004501a8013c0080ff82001bff94fee4fe27ffb6ff9c00320099ffe4ffb9fe8fffee011bff01ff6c01ec006900f500830032ffcc0108fffd0101ffb60019ffe2fffbff2100cbff0800bfff01ffd9ff0dffd0ff3a00be01940084ff5d003d003900310070007c003cffe6003500f800d8ff73ff8cff85ffa900a4ff1efff800edffbb00d7011d000dffb900150041fed4008f0004ffef0035ffe6ffbb00feffc000270015002100a5fe890063ffe5ffafff6000340182ffd8ffbd00fbffa90008ff39ffda0149009600fb005e001c0008ffa60037005d00ed000effb1002e0085ff78004900b300bdffb200effffa0143ff2e000cfff2ffcbffbe003700c001deff7b006efe6f007e00b1ff69ffd8005cffb3ff94000a00c300d300bfff1e0120ff8c00e4ff2f0016002300cbffcc009900e6ffd7003c003200a10057ffdc003ffe7a000600ecfffeffda000e004800a6ff72ff590078004dffd600f4ff580089ffd70049ff15ff74ff9a002600d0007bfecdff9f0013002bffbeff5fffde00f5ffc4001a0052ffaa003eff9fffb302030091ff9201480034ffb7003a01730059007800eb00fa014f00b1002bfff9ff5cff7cff48005bfff3ff3100bb009d00340047001f0047ff59ffb3ffbbfef3ffda001b002cff1bfe71ffd20091fee100e70048002600f30124ff1000c7fec0ff15ff7600e90046ffa5ff990095febc007cfffc0042004eff7900bafff4fe83ff6effe200f4ff7300c2002bfeccff1a017e0041003301e8ffea009dffd5ff710082000dffda008100ca004f0151004c00230044001f009e0042010c00a80045ff78006400fe0042005bff4b00d7ff7c00b000ccfecf0024ff46000f0137ffc4ff6000260002ffb40004ff37005000830032ff6e0105fee1ffeaff4f",
    "fa4ba817c5ccf62fe37a5a3db2804b1d79b3475dd5df51d23a1e0505bc43ce36",
    "sample 5",
    "090110006ffef1001a004afff9002c01300092ff2bffe20174001a002effbeff4affd50114000b00cbffb5ffccff7e00eb01300018ffdcffaaff72009200eb003f01c1ff11019affe0ffbe00c3fff5ffe30063012cff2000070001002cfeaf00acffd60094ff4cff16007cff21ff74fef90007004afe610038ff46001eff9100caff6b0080ff88ff67fee0ff61ffc700320032fff70078fef30022018bfe4dffc5003effeb008bffebfef80005fffcff7cff62ffbd014400d8ffee00d1ff41ff4e011900df003800a9006200670000ff9dfefeff74ffaaff73ff4e0057ffd400e300b9015efe6b0021ffa2ffcd00c0ff0f008400d00007ff680014ff56fed0ff330083000eff8d00830062ffe10061ff2c000c006cff70005a0041fffa002e009e00bbfff500840024ff6cffa30057feb80130ff230070fe890107009dff32fffb002eff47006200d60062ffdb0092ff4bffc4ffb0009dff61ffb5fee2ff65ff34ffa90012ffabffc1006dfe17ff3b003cff30ff08003afff0ff280156fed7ffc0fecf000c00c6ff5fff37ff75ffc8013f0082fedcffb9fffe0009ffbe0045ff87ff6b0006008bffb7ff34009a000dff5100b80063ffadff8c0113fea3008afe22ff8bffcdfec0ffdeffc6ff37ff35ff230038006800d0ffdc00ba004d000100ebfe8f00230012006effbaffe5ff1000a50044ffd7ff63ff660094ff88ffdaff3d00cf00d900b4ffdb008200540012001400a800b5ff15ffd60016ffb200d4ffe200fdffdeff4b0088ff870125ffa700b50092005c002eff48ffb6006b001a00e700c4002300baff3800bffff6008effb600570047ffd4ff7c00f30037ffb80054ff9b007affd400d3006eff8e0004ffb4ff7c0031fedd005f0019ffe801230062ffeffe2000680043ff6f0045ff840069ffa3ffa50033018900580117ffd2012aff47ffe9ffc9ffd800d50010ff18fffd011eff70fffaffbcfe9dfef1ff7700bbff9cff76ff17feb80079ff870059ffe6001affb8006fffd400efff64012afe7b002f008bff32ff03ff56000effc900e90156fef30079ff3ffff500be0074008100d7ff6cff18006cffa5008600f5ffdefffc003bff1100180111ffc2ff79ffa5ff25ffca007500f00063ffae011900770044fe780076fff4ff57ff0a000d006dffee0051002bfffe00caff5efef1ffa6ffea0063009aff7bff3b00acff86004900b6fe90ffe3ff18ff700079ffabff60ffe70096fef2ff85ff4c001100d500b60052ff98ff09ff62ff8f00cdffb90120ff4a009eff4afff3ffddffcb005eff2500280045ffee00a0ff4e001cffbf002800cf004bffdcfff500abff7eff980054ff7c00470076ffb6ffce00c60056ffe9006e00140018000200d2ffd2ffa6feb2ffc3ffb6fffd003dff68ff66ff9c0062ffbf00aefee000bb0048ffdf007eff2a",
    "69e5bf07ea263f1903a46ef2f9aa7551b5be6a784a575441c6a5b59750e41717",
    "sample 6",
    "09ff8d002b00efff94ff84ffd600a3fea9ffb0ff27005bffd3ffcd019b000cff1b004f00d9014800bbffa6ff60ffa7ffde01e50065006e00e4ff76ffce00b0ffa2ff4e005f00e0002c00450073ff6a000601c3ff7cff96ff94ffa9ff460189ff9b00b400e40005007eff74ff85ff9bff110081ffd5ffadffb50026ff93000dff76ff8dfec5fdb9fec8ffc201d400c9fe32003bff0fff4c0013ffa5fe9d00fbff10009bffe9ff7b004b000dff45ffc5ff7affee0023ff540011ff6e001d001effef0123fffbffc10119ffc70042fea1006d0135fecafff1ff46ff1effda01ce002100daff900027fed7fe48ff61001cff88003400cd005d001e0112ffe2ffbe0167fff90148007e0002ff7e008100100055ff58fee400a6004400c9ff52fe7fffa1ff39ff3bffabffb8ff70ff8b00490003ff96004d006b0058ff6201170025ffa900d60008ffb700d200dc0116ffaeff5dfee7ffd6ffb10041ff59018affac00adff9dff4a0131007d011c0016ff8f0072ffc8ff6bff010103fff8002fffaaffd8ffda00c30054ffa30018ff0c00290088fdeefff6ff54ffd9ff78ff92ff52ff6c0028ffe00014ff7cff28ff8ffedcfeddfe91ff18ff8dff19ff1affa60048015effba008b0145ffeaffacffbb0043ffd100cdff9eff9600b700b8ffbcff930015004700920165ffb60077ffacff88005f004a005cff520037ffacff7600cefee5ffb900af00acffa70065ffa60010006dfff7ff950034fffc00f7ffcb00a10045000e00edff4affbeffea00adff6101a0fe3900c1ffee004cffa6ffb40098ff93ff9aff92004c002bff10ff95ffc8008c0027004dff2600a9ff74fff3ff1b00ca00b0ff23011cffe0ffc7ff530063006bffa6ffee006a0012000cfea3005a00e40085ffb5fedaffe0ff810011ff3bfea2006300d1003300eeff880070feb0003b008e0056004bffe7005c0072011e00d2ffa1ffe80102ffdfff00006fff7b0036000d00340040ff8effc20015ffd5ff9c00cf012afee3ffc30081feb9ff0d001f00500007013b000b0075006200340016fefc00e700f400b7000cffafff65ff280024ff11002400780081ffd0007fff98003effb600f3005a0054ffde005dff3bfebb014e001c01c50000ff55ff48ff8cfeac0087ff840011ff20015200a4fedf0057ffbaff82ff6a0050ff12ffc0ff99ffbaffce0129ff18ff7bffb2ff9efefa00caffc3fedcff87ffde004c0002ff4dffad001dffdb007b00e5ff9aff9e004bff83006fff6f00360009005a001d0006004f01200103ff0f019800a8007f0006ff5fffe1ffa1fff5ff8100470077004500c20005ffc6ff60ff6d012800d701370011ffa900ac0170ffadffbc006efff2007d004600920040ff58ff5effb3ff64004dfe2affcbffcc0050ffef0025fff0ff54fe6cff1c00deffa70034011aff85",
    "4bf3e3db906ad25bbe78cd425485eaa6c54983dc626fd919e11198dc01eb6f13",
    "sample 7",
    "09ff6900a500150109fffe0078fecbff390148ffe2ff03010bff6600520097007afec40050fef0012d0118ff89ff84fff200fbfee700a000b100e7ff52ffd4fffdffbfff93ff2e005f00390013017000ceff90ff80ff4dff40ffe900270010ff9effbdff930087ff73ffa300beff2cff97ff9afebdff06005a00a4ffb3ffb4000efff8fe8eff39012c00010009ffbfff7cfffaff4200e3003b0026ff7fffb500eeff76ff95ff5e001600a4019cff000029ff63ffd9000dffa6ff4cff510002002affabff2c001900a200af01080141001fff71ffca004c009700cd008900db00340050fffd0045ff96010c009200e90089ff60ff1b00bb0161fe47ffb700cbff8dff87ff7bff4dff57ffa000dfffac00b2ff55ffe9fff3ff67ff82fe5100940119fecb004e01730001ffe7ffa1ff600002ffe60183fff5ffa4001900c2ffedff4f00c4fef9001cffeeff65ff2f00bcfee2ff24009e0078ffde0034fe590088ffbdff9c0018ffca014600a7008cff7bff4effecff5300770004ffe4fef2ff8a00510047005500f0ff4f0125fe5700040038000a0043ff06006d00bb00b5ff91ff70fe7e0004ffacff7e00070024fe6dffcbffbe002200b3ff91fe6bfebd0070ffb4fff3ff570127ffdd00d10087004400310005ff4100060098ff960053fff80056ffb4ff40fff5fffe0051ffa2012a00a6004301a5ff2cffa5ff5800f6ff33ffb200b9005fffc000e7006c016a002500b300d4fff1fffdfefcffc200e1ff0fffdc003e002cff38ffdaffc700150092ffc8ff90012a0023fff0011eff74ff37000aff82ff86ffa4ffcbffc5ff9affa60134ff5afff6ff6eff7d000f00b0ff1effdc00a500e0006fff2e00b7fff900870162005f0148ff8a00ddff9b0034ff61003d0103014bff48ff0fff250193000cff4a0063ff7f001e00a4ff95fe99001f0022ff7bffcb014afffa00210088001200c4ff1bffbf00e100bd00ad00af002400f4ff670038ff30006a002f006eff6fff60ff8cfeebff6f0133000e00d5ff93ff2b00b20020ffd90121fff3fed60036001eff92ff6700b0fff7ff5fffe3ffac0095000f0102ff0e00ff0039ff5e0111ff1eff80fff2002affa9ff9100020055000100bb012f001bffaf000b0129005bffa9ff1700ccff54ff91fe5a00630075000f00f60097006100aefee10042004afdf6fed80083ff6200520002013200a60122ff6c001b003200ab00540031ffa40021ffb800280124ffe6ff8a01310020ffe800df00d1002cffddfe75ffebffe6ff5500af0092ff29ffd8006f011fffdbff7b0041ff49004effb5ff110062002affc2004f017cfe9f0023ff7fff4fff93ff2dfeafff64006b006c00ba00e8ff99ff8e0050ffa2000fff6900be000bfee50004007a00720160007dfffd00f9ffa4ff0000b0fe70ff66ff4f0029000f002dffd2",
    "857241e477e7b1d335e634874a6b61ed590180b7d9d7bccb7609c0e95b47143e",
    "sample 8",
    "09ffb90006000f011e0034ffbcfe9e0107ffc3fede0107fe55ff4dffea006dff56ffd20064ff71006b0069ffe1002b0035ffd00080009dff37ff6d010500a6ffd000c900f3ffc9fe8fffa400c7ff98ffbfffccffdd00d0006100be0086ffec0082ff8afff8ff6c0111001b003d00220053ff4a00d4014700e7005d005effcb0153ffb3ffe80000ff97007effeeffb1fff600d6ff18fff6ff74ffcbff01000c01560002fef0ff8eff1f000efedd014b0025ff8bff4400d1ff2f012800af0043ff1700b5ff9cff9bff09ff5e00ddff92004cffc9fe3600ad00c3009d004fff1efeea00aafeccffebffd4ffaeff890047ffb10115009aff04ff6600d300e30085015b000e00b60058ffcd00d3ff11ff0e00530023005b000000de00ce00b6ffe6003ffe98ff4f0020fff8000f0105ff7bfec0ffc8fffc004bfff4001cff5100bbffca014100b0ffc8ffb000d1fff4ffbe0109ff63ff7b0021ffa9ff68000c002bfedfffa8008300a7ff7bfebbffca0031005d008000550039ff2fffa1ff4a01b8ff2b0027ff37ffbf00dfff0a00e3ffc000a80064000efe39ffba00a7005d006f009e00ddffb7ffe8ffc2004a008cfffdffe6fee300ca0105016e00ffffd6007bffea001f004dffcc0175004e0123013100a8ffa6ff9b003dffc80046fe77ffb1ff6301aaffb1ffb9ffbeffee00fbfeef0027fecaffe4feaa00b4ff51004b0052ff410072012a0105ff88ffe6ff71ffa0fefe00280033ffe50043ffc40015ffc0ff7e005500caffd6ff330065018affa5000bfff600230164ffa9ffaa0058ff3d00100013ffbd004c00fe008c0029ff11fffd00890020fe84ff39ffe7ff6fff9700b2fff1fecc0027ffabfffdffeb0078ffe5ffd0005afff9ffbc008000b500a4008b0017ff7900710058005bff9b002e00b600090056ff83ffd3ff7dff060041feddff9cffc9ffa100aaff98fefcffdc013300bcfed800a1ff9200420057ff1c0007007dff8b009e003a012100b1006d00aa00ae0067fff800f5ff1ffebafebd00ba00a9ff24ffb1feec003701d0ff7eff65ffa6ff2b008efeddffa70058ffb00104ff3b00a7007100040138ffb600bcffdcffcdfe62ffd3005fff52ff6e0029fecdfdf0ff64ffe1011bff7a0042ffcfff8f015dff200069002fffeafef4002a008dffb5003c0100fec10019002f0092fe7dff9e00e7ff37ffc7ff26ffbdff56003c003bffbefff9003c00ffff3a0002001d00fafe69ff34ffe50020ff8dff7bfed80066007cffb00092ffdbffdd006dffea0047ffb2ff9100d200d80095fff6ff08ff83feb50027fff8ff6300df006e0099007aff71ff5cff9bfff800cdfe43006aff4e0040007dfff8ffbc013400ae004b0060ff92ff62ffc90071ff5cff900142fef20085ff8afff7019c0071ffc9012eff73ffb5fecc0014ff97ff58004c006c",
    "032294bf7a7bbecbb2f4f0f474f7025c9cb6b6abbedba2ae4709fa7f430454ba",
    "sample 9",
    "0900fdff3800cbffe70017fe9800b0fefe0066fedc006b00e300a1ffee0083005201720063ffdd0008002b0031010eff550029002e013cfffd0075fff6002fffb1ffe100b8ffeb0006002dffe4ff9400b7ffdafe6fff5f005300a0fe94ffe8012901e4ffe1ffbcfef0ff7dffe7ff9300530041005cff900164fe2bff9301a7fe9bff78ff0fffbb0053ffb70017ffc9fecb006fffd801430020ff66005dffb800d20029fee4fef3ffee00050052ff78ff9fffdf0076fff8ff40012d00d400c400480098fff2001afe03ffc3ffe3ffbdff23000600edffefff68ffc70103ff2bfe1100c40067005000e6ffccff8efea0ffc600ca003cffa3ff1e008a003d005d00410041ffa100fb008b00c6ff2c007900300179ffce00dfff400008ffe0ff9800e4ffd5ffa5ff42000fffe6008200a3fffb0030ff79ff6fff4effbcffbe000b005500aa00de0000ffbe00aa0000004afffa003000ecffe0ffac00b5ffe0fff8ff2c007c000001be009f00cd0044006c003600b200aeff91ffe60149fef000aa0048ffe000c7009fff4f0082001aff76003b0026013800a1fef801c9ff87ffd7003cfe98ffc4003101bbfff2008fff61006a0005005e0022ff66ff2400bbffa40196fef5ff6c004000420075ff38003b0040006cfefe00a7ff4fffcc00600070004c0064ffe9ff7fffcc002700e20082ff770047ffd2006eff970070ff880064ffe801bb00740001ffd401e00017ffffff67006c0167ff83005200aeff85ffecfe8c0078ff4b002700b00037ff8c00d50086ffa2fe550068013c017e004aff7cffeeff08ffb0008d0018003b004500e0ff70ffb4017fff1b003affa4016fffa0004e006100a2ff09012f00adffb50096fe9efee900cfffdcffcdffbf009dff87ff7bfffa00d00088007cff22ff8700bbfeda01e1ffd20107ff84ffc3002a00360097ff8c0053ffb0ff85ffb2ff25002c000a00160056ffaf00870044006600cd0009002dff2dff64001effe0ffa1ff25ff5800620049010b004e006a00bdfedcff5cffa6008400ffff5fff77ff83ffb600abff9f011dff67ff6200950074018d0094feea01a6feb2003cffb9ffb3fe67014400c4ff6eff44fef60041ffd5ffc2ffed011cfe07ff9600370114ff5eff80015bff8100c8fed3ff2efeb700a8002b00ba01070082ffc3fe90ffdfff5100580109ff6300080090ff6aff2a0071ffe200e5ffe0ffcaffe6ffb7feeefff1ff03ffb7000a003cffa6fefcff20001e0032004c00c700d8ffdbff2dffc4fea7001200440045fee8ff4d01270034ff75ffe5ff8bff3b0094010b00affebcfff9ff0400f500960055ff02002a006afeedff990021ff64ff6cffd2009affda0069ffb30012ffe5fef5ffcd0198013dff2501a2ffa3fe15ffccfff9001d009800d2ffdcfe87ffc2ff5c01bcfe8101f5ffcaff8dffa4"
];

const KAT_SIG_1024: [&str; 30] = [
    "af0228b7e30f8c0a6620c8419cd181acfe6c76d134020a9fedb3839ca732f775",
    "sample 0",
    "0affe7ff90ff90ffc0ff42000f0151ff18ffaeffa0ff9effaaff72006a005affaffe4fffdeff80ffe00078007800b5fff8ffe4000400b600550106007d0133ff31007effb500730035ff93ffc0ffbc00ca0024ffd7ff46fffe0083009effee0029004800260034ff36ff1bffccffbf0000ffd0005500bcffbe0036ff6900bf0024ff0c007cff59ff15ff470060ffcc00380069ff33ff8000cfff62005cfffb003c004700d4fff7ffd80123005b00bdff020045ff66ff1d010400adffb100bafff80101ffeaff5bfedbfff9ff85002efe2dff45ff85ffb700f000ceffe3fef2002d000000b300e0ffcfff030178fee2ff0dffb4feb8ff930040ff93ff8aff4a00560226fec9ff53ff50fedcffc5fe6aff8fffacff3dffd9004ffeca0089ffc7003dff92ffffff89ff64fff1ffdb0030003bff75ff1500bfff64ffaaff65ff690029fec1ffff00ea009cff0fffa1018101ad00e90070ff470081ff9c00950087ff230032002bffd0ff3f00c2fe9700aaffb300caffceff69ffadff96fefbff0cffa2003a0084ff61ff42ff97fefdff58ff81001001bc006dfff700020195ff33ffa0ff63ff6a0097012effd7ffadff32003b00320006ff960026ffeeffa8ffdfff4bffd20014001300220068fee900510059ffa6008600c2007fffc9ffe5ffd7ffecffb400160109ffd8002e0006ff3dfee6ff83ffdfff62ff3e00200011fef900cf003fffcfff70ff090026ff87ff25ff63007a001700e5ffda004b00b8ff4dff990095ff37ff5b0135ffc0ff9b0159ffab0091010701d3005cfffaffb0ffec0061fed400e000b4ff940079008ffeba00cbffc0ff070091ff90ffd3ff3600240061ff33ffe2ff7900e3ff3900400038ffa500d000fd0080ff69ffe2004a00c2fff2ff7300dfff07ffcafe4600110113ffd20085003e00afff9d00230049ff03ff1a002a00150041ff75fffcff6200550067003700bb002b0092ffd9fff300020168ff90000800070011ffb800910095ff10003aff8eff38ff93008f00c50117002cff2f01400152ffd2ffadff9b005900a80036ffe5fef2fff4febdfff6fff6ff96002e00d3ffc6ffaf00bcff0cffa9ffc9ff3400e20015ff8ffea900ceff470060ffa3ff62ff73001dff8ffff3004eff54fe6f000dff3300af0028ff62007e0063ff19011fffe3ffb3ff1dff95ffc8ffa40042ff97ffc2ff39ff7dff9bffda008b000bff6bfebe00540021ffea00320118ffbefee4004eff3c00d3fff700a9001e001d0091015e009200bd00e7ffaffff0ff45012aff49ff010135ffffffc60164ff6cffcfff25ff63fff10026002000e3fffd00650133ffc600b4ffdaff40ff0700e20043003d00210001012500b6ffb4008bffd9ffa2ffd2fefcffaaff7f00ceff0b0063fffc0019000bfee5ff76ffe50026ff92fffeff6f00d0ffcdff26ff9cffe000210090ff720003ffa7ffe2000000e8ff8500afff3100b800a700d30008008201880089007a0055ff500022ffe0ffe8fedaff0eff2d0012fe4cff0400140068fdec004cfff6ffdaff31ff63ffb0ffa3012500ba005300cc0005fff0fff5013d00bdff50fe3400a40024ff95ff2800c6fed100f1010d001f00ee00d3ff96ff83ffc40010009f0144002a0051ff36ff8b015b001800800067ffabfecd0050000cffc500aaffa600580071fff4007e005fff37ffbffff6ff1ffeb20056008a004801570051ffafff27ffb400f0ff3eff7200ef00f2ff50ff99ffacff49ff7affd70068fff1ff1c00b6feffffceffd9ffe7ffff00edff95007cff7a00f9ffaf0080ffc6fff1ff71ff1401a4ff190054fec6ffe5fea6ff6100eeff89fedf009600550024ffe4ffba00520080ffffff75ffb1004aff33002f005700e30097fe2a00d50074fea6ff89001d00120007ff0fff38fef70052ff9d001800790072008a01920116ffee00a0ffca008d006b002affb6ff61ffc90002001800ab009300940054000bff120058ffccff26ff96ffd0ff18ffcbff4fffa9ffc0011301aeff8eff6bffdd00830181004800b30088fff900fd0041ffffff83fee2001dffa8ffc00124ff89010a0050011500a7ff3200d0ff63005aff9900470005ff32ffe2ff7300ec009e00e700f6ff20ff6f013200a9ffb7009cff4cfff4febaffd00009ff08ffb0ff570093fef9ff75ff610016011ffec5ffd2ff84ffc600d6fff1ffcbff0400e8fe88ff730086003bff82ff23ff7efea8000bfeb8ffce0027ff17ffe8001cff840092ff24ffac00440035012dffb2ff950054006effc4ff67005d003bff2fff520007fff4ff7dff68ffe8ff5d00b6ffce0046ff02fe73000b0132ff2b0035fffd0085015d00a900570031008bffe5ff2b0130006effca0043ffdf0068004c00ac00f6ffe8007affa30060002e0077ff47011400d8ff4effd30058003a0176ffd80033ffa9fe8dfe57ffb9ff31ff2600a2ff78012800590072ff7effd4004dff48fe8affe6ffc6ff4b00b70079002001450052002c009c01ed00ccff7b00e3ff34fffe0085ff7f011900b5016d001dff7e007affa600baffe0ff6effe1ff810008feacffb3ff8aff84003efeecffccffde013fffe900230059006c0006ff94ff8b00a60089fff7ff70ff0dff1efef200a9006600810071003bfee9ff350059ff04ffe5008a002c0035ff8c002a005a006b004bffdd003101f0002900100006ffecff8f00f9ff16012effc4ffd7ffba0020001effd50082ff4fffef0083ff74ffabff8dff6f00680097010c004cff2d00a0ff83ff7b012f0000ffc90121ff73009effcfff5bffd5ffa6ff82ff9dffc80071008bffd1008dffaaff14fe7a00d30089fec3fee6ff6f0041ff7600170104ffceff620026003700430056ff92002c",
    "0deb2593f9773204e70b1cac2898e54d8c75b8c86918a711120622feaf80994c",
    "sample 1",
    "0afedbffe8fed8ffc7ff83ff5effd4fffbff71ffca01e9fe37ff2f00be00bc003a00cfff9300b1ffaeff13010d004bffcc00c50038ff1700abffda0048fef90080ffc7ffbd00150097ffc3ffa8ff500116fee2003bfe5bffa800afff5200330076fe46ffb2ff98007c0043ffdc00a60046009bff8aff99ffbd0008009bffc7ffe5ff2b003cff65ff52ffd80031ffa2ffbc010bff81005c008800fb00dd00380085001900b2fee1006fff79006dffa0febcffc4ff5600ef00d0ff8aff64ff87ffb60027ff540056ff9effbe005b0114ff6aff37004fffb9fe5400e101cfff65007600d100fe0068000eff25ff5900b9ffeaffbeff11fff600d3fffc0008ff5400e80035ff5bffb1ff6eff9300f0009000c3ff9c003c00d4001fff2afeb6001a00bdfe6500caff880050ff8effb1ff5100a2ffbe0007ff49fe51ff4900f4ff89fff4001c006e00060017ff35009300c7ffd90031ffc200ba011f016d0008ffe5ffdf00e4feb0ff5f00a8ffbcff27ff86ff16015cff6d008e00c900c30055fefb003f00c300c8005bff540022fece0009ff42ff2e00dfff8dffdefe75008bffc4ffedff6100c601110010ffbc003a00310057ff4dffd8ff7efff5ff35fe6cfffc00a1ff9801510038ff89ff8b008cff78002f004b003e003d006c0152ff57ff9dff27002800d1fffe0134001800660037ff39ff95ff5bff5a0144ffa700d90140ff94fe56ff14ffcf0074ffd600360189003affe2ffe4017f001b0074ff5300f800edfea1fee70033fe1a00f4ff6700250010ffddfed5ffae0094ffd4013dffbaffaa011f00ab0055000c001cffbf004b0031ffa4fff800420102ffa5001b007400a7fffffedcffadff63ff89fff7ffe00039ff9fff1fff7e005dfffdffbbff51ffbb0112003200b80093ffd6ffdeffcd003e000cfffc011aff42ff18ff46ff9fffa90149ff80012dffa000bcff65ffd400560111ff6cffb4ffa3004eff4affa2ff59ff52ffd5001a0089ffb900b6ffaa0150ff93fff300ee0066ffb8ff8a000bfea90033fff4ff4cff8fffc50081fefd00540108008fffceff330114fffeff99fe9cfffcff5400d1fefdff9eff5bfff600afff7a0090016800aefff8ff4500ad004fff5afe83012affb5ff18ff40010700a50150ffcafec0014d0016ff58fecf0004fea0ffa7feb6ff88fea6ff45ffdd00ea0012ff86002300c00166fffcffd801bcff6a01a6ffba004fff7c00fe00eb0012ff75ff50003b00d5ffccfee3009affff0051018200c2ff3fff48ff7efffdffee002500dfff7e005b0107ffc20054fff9000800acffbdffb9feb8ff45ffd7ff91ff94fe91016a0097006a00350017ff69ff5c00740033ff32003a008aff4f006f0061003b00b9ff6eff05ffe8ff3fffa9fff800bbff5c0065ffd7feb90019fffdff8fff65feb9ffaefe8b00a8ffe40017ff6c00ee0090ffa700c2006dfec30058003eff17ff6d00f6005600abff18ffc9ffb0ff48fff7ffcaff25018a00140038ff5b005bff8f0037ffcc00a4ffdaffcdff880083ff2bffb1ffc3ffa700bd00a5004f004d014a002100970017fe7f00640104009dffddfecbffed0087ffc1003dfff20061ffa6ff380020001cff21ffa20102005effccfefafe7c00acfff2ffcd0033005f00b9ff6f01600015ffd4ff87ffdbffdbff34ffcaff620012ff4fffbaffe20064fefd0021007100b600a7ff5bfff0fff7ff51ff6100cb004dff6f0077fff90089ff7c00c3ff08ff79016b011f017200ad0023013e00ae014b00a2ff8efffaff4bff50fe6c006cff85ff9bff4afffd000a006effe201070030fead00dd0033ffe9000efee200cc00bb0077ffee0028001eff39ff4200b900b70019001e003dffcaffdcffd3ff5cffa300d7ffe8007400320036ffb00028fed0ffdf003afffd007efff10061fff50119ffd8ff8dfefb00b900f7ff94fef300cf00610059007700ad0085ffcfffcb009fff4e00e3005a01600034005cff93ff8eff0c0080ffe2014501200090fe76feb80105ff5d018a015fffc6ffbcff63ffce00aeffadff240018feeeffae01250055ffbf0100008b002a006fffb800d5ff1efe8dfff2ff870100012800d8ffd000d3ff72000aff7aff3bff0900b000ff00ebffb8ff7c0016007200cafee80089ff7a00f8ff94ffe0ff2dfff3017bff85ff6c004f003affc100dbff22fef5001c0075ff7c0024ffacffee013c00deff3200280064007200e3ffa3ff77ffde006aff2a0004013000e80050ff690043ffa40164002f013000bd00abfeb6ffae012cff00ffd4ffec003efff00143ffcbff89000a00bdff8200bafff7005e0097ff5b0146ffe1ffdc00a7ffc5ff38004affb5ff6dff7ffed4ffbc0031001c0026003d00a20004009cff73ffdbfeeaffcdffb9008a002000410079009affe300a9ffb1fffaff7b00b7ff70ff6dff08ff03febcff9f002eff9601660045ffd2011a00540075006e000bffd0ff870008ffbeffe8ff0700bd0084009d001bffd90068004e00ffff37ffd300e9fef5ff290003ffe000b0001affde003200ed0016ff6a0106ffb4ff7afebaff84ffebffea00a901630075ff11ff84001a0134ff96fffb0057007efeab00f9ffeb002c0063ffb4ffd50069ffaa005f00170020005600090048ffdc00e3ff000020005f0035008f0008ff59feef00a8ff630068ffb6007d00370026ff7d00240095ffbcffa4ffaaffe000940086ffb6ff9a003cffcf003200c5ff08ffe5002700eefe490018002d002300b200c8008aff5bff3bff07ff7000160043ff150082ff2a00f1006a00f1001800280151ff1bfff7ffe30211ffc6ffbf0005ffd400a700ac005a0079001500f70020ffb8ffdeffa9ffb2ffc8005f00f3ff76fff5ffe2",
    "d6f42222c7d600a0168c614d04927d74c7dc3b3eeab9c97d18637f7d658c8e0e",
    "sample 2",
    "0a012d005500d3002700b0001cffc1ffe20005012800a9ffd8feda004a0047ff140040ffe1002700d7006d0088ffdefeb80043ffbe00acff56010d002d0016004a0048fffa004700b0ff99ff83009a00910058000bff81ff230033ff4500cc007effa9ffb100b1ff7aff50ff62ffd4000dfebeff9fff5eff84015e00d1fee800160114ffe10269002efffffefeff45ffd30021ffeefe84ff6f00470029ffa0ff43ffed00590170ff530069ff9b010b00b3ff65ffe0fedb00c9ff89fffd0066fffaff4c00710001fed9ff870014ff89009effdb0059ffe9ff3000b9ff03002b002c0016ffc7fece0110ffe0ffd3013c0040fe6d0029fedc0010fecb0000ffb8ff6cff760079015a0162fffbff6aff490116ffe70037ff79ffd60022ff0a010c00a8013500280014ff1fff96009fffdcff860026ffb600a300acff8b0032005effa8ffea00ed0140ffc7009c006e01180040004eff43015700dcffa4ff3b0050ffc6004cffbc0075ff1200f2ffed007e00e100c10012007200b9ffd70013ff52ffecfe80ff2dffed0061013afff2ff93ff72012c0051fec6fe78ffddffb4005aff6cffc2ff9d0026001d0024006a0023ffc2003cfff50018ffe4ff5dffafff6dff1effbbffc7010cff6aff3e00b9fee6ff4b002bffecffbd00d800fcfe2eff53ffdb009ffefc00a1ff48ff56ff83ffb9004c000c0024003f01660056012a0013ff8e000400460025ff75000c001100e200fd0040fff5004f003a00d2002affbdfeb8ff32ffc9ff8d0178ff840091ffba0106006600a200080085ff3fff5900d600b701330032ff66ff980044005f0014ff5f009b00b40032ff570098fee3ffefff96ffebfefffef100acffa70109ff3effc50055ff41ff740090016500acffabffbaff0b0077ffc700ef0076fe6eff94ff93002f006d0038febd00da01340184ff9b015fff66ff79003d0117ff6b00760013ff2700360064ff8700a10047002fffbbff6d00d0ff60006000e0fff9013b002300ea01c8010aff07002400d7ffe6ffdf01030098ffa0ffdeff380001ff44003d002d007e003cffb1004800da0067009a008f008b003100170031000e0016ff8f00ff00a60063fff4ff9fffc9004d00640128013e002fffd800f00046000effb5008c005b002a00360063ffb7009c005efffeff750021ffdaff78000cff16ffce00e4ffe7ffecff250078fffbfff500440015fed90009009f001e0067ff42ff7700cf00cb007f0024ffd9ffcfffe50099fe85fe6a0164007500320007011d0009ffd8ffdc004dffed00c30005005afe880038ffb500b6ff0bfffeff27ffc2ff90ff6800180043ffaa000cff2c0022ff9bffd8ff10ff9a0083ff50fffc007bffc900f90054fe860005ff8cffab0080ff7100adff5e00ed0069ff390140ff5800290065ffc7ffc00105fff200940040ff80ffb200c7007fff280020001b01bbff47ff9dffa800b30014ffb6fe3e0028ffd20077ff560069ffac00e9ffba0017fec0001300b2007800570161ffff0025ff6a01930037004d0040ffa0ff6f0040fff100670077ffdfff12ff430099ffaf00f7004e00c500010032fe8c015b0096ff08fed4ffe7ff9e009c0049fef5ffa8ffc0004900edffee0035001500a9015dff9d00dd0061ff8200110074009801350003ff85ffa3ffbb001d0165014b00c400f2ffb8ff5e0054fe63fff1ff90ffcb010a00aaff36005b01feffdc002effa5007700a7ffc10059ffd7ff53ffbfff72ffcaffb6ffac00bdffee0108ff9dffc900c0001c0123ff6900180073ffebff9300140021010f0002fe7fff7b00af004a001bffa7ff24ff3aff0cffd3fe7f0037000affd400b800d8003d00daffa700a5ffbcffb000a7ff7900980063ff7d004a00a3ffcc0007005c00acfedfffd60035fffc001f00feff5effc5011dff42008c012cfea5ff97ff2d00b700c0fe2dff6e00cffe9d002e01760040ff98ffd400d200f9ffec0064ff3cffe2ff6900c200e5ffaeff80ffe9ffbfffa4ffd2ffa0ffc5ffc4ff84010000c900b9016a00ddffae00b30080fec9fefdffd60014012bffa8ff80ffb20051febb01130082000800b1ffd1fef700d5ff2efedc00b8004cffe3ffe0ff79004a00aa005aff8dff9d002ffed2ffbf0048003100af003200150029fe9e00e500150059ffc4ff750040ffb0ff63ffeeffffff6c00fd008400dcffe90092ff7fffec016d00ccff8aff8a00d7ff47ff210024012dfff90015006e0092ffa30097ff34006c002f0073ff3d0015008bfee1ff0bfe7eff040090ff8affd5ffb200a8ff70ffd3fed0015eff69ffa5ffc7fff3002effdc0096ffdf00a0ffd90100fff8ffb8ffca0036001cfed5012aff84ff80ffaf0078ffcaffd0ffd7ff7bff68ffa10022006bfffb011aff69000d005b006f00a20128ff9dff83ffbdffa10053ff47011f0060fefa001fff6dff6e0046ff94ff4d0180ff90004600ab0015fffaffe60065004c003bffb4ffb6ff98ffaeffed01b200aa00fdfee2ff73ffaaffbe0081ffa6ff810012ffe1febbff89001a000eff60002700b20060fe5affcd00ff00acff1dffe100440081ff0500c700bf0090007400b20016ff8dffa10036ff6500fc0043ffb1ffaaff72011efe4200a500090136ff4bff6cffe9ff9cff6eff75fff000c3000a00d1ff85ff8d01d00130ff6fff0d0078009100f8fe690034ffdbff220086ff1dff59004b00da00c0ff37ffd8006400620100ffedfffd004effa300ed0085002cfedfffcdff78001500c700a0ff6dff71ff6b00b4009f00940133011d009a007dff76005ffee701030017ffb10097006cff4e002c010fff9d014e0054ffb6fe33008affb6feffff3000a900adff8cff1d00de005dffb000250131",
    "67b099b366cf45c33e4b0b642d85f59f005333191606e3eee1a1b5a1d1f4d969",
    "sample 3",
    "0affcbfeee00dc00220058ffc0011effac00a000ff01290005008fff2fff51fff801260067fefe006eff7efea9004aff87010fffe1009e0047fee60072ff93ffdbffb60022ffdaff75002600f100a900bcffb7ff43008eff5d0017fff4ff510006feb6ffa2ff9a0136feb1ffd80165fff10050ff98009a004300dfffa1ffe9ffeffffbffcdffd0001efff4ffb5001c00a9ff33ffeaffec00720117fea8ffb8006e00190031ffa8fe9d0101ffea002c0010ff16018d002dffa9ffa7fe38006401440058008e004b0083004effcf00a4ffd900940051007600abff9dffa6014200470006ffd1000d001b0031000fffbaffa1fe88007bffa3007cffdbff5900a6ffd601130082ff36ffb2ffe8ff23003dffbaff6fffba0094ffcb003600b3ff71ff8eff360001008800c6ffc20032011dffb60050ffe1ffedffb7ff4fffa7ff700096fe43ff490019ffcdffa4002400360102ffc7ff0fff780024ff22002eff570117004affa50145ffa6005effb3ffe00213ffdc000cffe8ff7b000c00cb000b0093009300d5ff0bffadfe8d002cffd5ff36ffb9ffde00c900ce00f50028ff87ff3effcdff9efee70016ff23ffceff2e01a9ff49006d0094ffb90051000cff9cff0d0067ffe4fede0128ff12ff90fffa00c0fff0ff22ff33004efec800deff6500eaff8b001ffff5010b002dff900104010a002b007effed000b00130022ffcc0131ffc1ffcfffc4ffc7007700e6ffe0ffd800350042fffb0051ff93ff8afff8ff4c0179ff08ffb9fff600acffd7001cffe4fff8feb70037ff79ffebfef5ffd6fff0ff9e01c90052004bffc501630085ffddffd800a7fec1ffb0ffac0058ff25fff8ff50ffb10085ff4efff60091ffbaff77009bff9f00af002900daffe6009dffceff7bffdfff330055ff7c0046ff750124005b00db00db001e0053ffd9fff800230012ff70ff68001e00e0ffb4ff360088ffb20036fff5ff05ff50ff94ffdffe93fec0000dff5affc7004d00c30038fef700dafff6ffca002600a60022ffe600e70168ffa8fffaff0cffe5ffedff24007400140075ff380049ff14008dff1a02010083008cffd80061ffb0fea7fe7800f6ffb1fee5018e0031ffd8ffde00520077ffe400600021ffeaffec0131ff540076004eff94002c003eff310050ff60ff4c009cfffb0052014aff0c00e000bbff8c00710052005fffe8ff48ffc8009dffaeff3a00850030ff1eff6500c00067ffe0ffbb01d9ffe6006cff4f0089ff22fff4ffd7ff54ffb7ff80ff53005d0083006efff3002eff1a00680011ff73ff99ff66001600dc004b005c00b600640040ff9dfebaff300111ff60ffc600dd0040ff2d00a0ff9b00540026ff350011fe8200e20105fff0007d011a000a01ac00a5ff3400340098fef80080feec00560080008600b7ff9fff7b00f7000b00b2ffacffc8005b013aff5f009a00c3002dffd5fecc0177ff7c00ac004bfe5000120002010a009afec90052009affa900980089ff74ff39ff1cffb50000004effc4ff2eff50ffd500f101beff4500950066ff8d00e8ff7f00320229ffdfffbeffdd001200ebfef200760117ffed00f40032001effbbffe1ffeb00130079ffdeff73ff5affba00790032ffd7007900c0fedd01bb00c8002cffb5000b0080ffc2ff4500baff3ffed10017ffbb010600edff6dff34ffcc005a008dffc8014cffbc005f0128ffb2ff6d0096ff9600a200730036ff62007dffbcffc500d5ffafffca00a900990055ffffffd100410024017d0085001000ffff6f002aff86ff870031fec9000001420024ffa200a3febdff14ff76fffdffe1ff73fea4ffbe001b0086ff41000d0034001d0050ff95001bff7ffedfff9aff40ffe300360059fffd002affa7ffe90044fef2ffda0079ff2f004200700060ffbefebafeb8fe6500e5fff9001b006400dbff8efee600ba0100ff230010fe7900a700410036ff0700ea0098006300ebffb800ff0007002c006c0108fe5500f1ff470059ff84003e008e00e4ffc2ff6eff9fffadffd9ffa70034ffe60034ffcc00fb002bff55000dffc9000effee004e0121ffb201d8ffd9ffb9002fff15fed3ffbc0040009dff7e0050ffd0ff06003e001400150010ff13006cffa4001cfefe00d5ffda00af0041ff9501440154ffbeffa7feb701ab00edffe5ff9d00b5ff640021ff6e00b8ffb700a9000cff040067ff69ffd20109016f0085ffbf0133ffbb000700a1ffc9ff82ffa8001a000a0115fef4005dffd8007d0047ff370051ffcd003800a3ff540160ff4aff8300cc00a9ffff002dff7aff870008fff5ff93ff67fff40022ff350110ffd40083ffd9ff250128ffe700a0ff6affbdffca008dff38ff49ffaa008dff2affff0048ff25000a009cff40fff1ff9d0072fd76ffdeff37000fff660008ffe7ff7d00250014ffb3ff84ff63ffd8ffe8016dff380094007dffae00450082ffeeffd2ff91ffd001270006ff4300e1000cff22ff9b0037005e00dfffbd004000e2ffab0090fff600f500dcffd1ffd0ffddff3bffd30094ff5a0063ff9fff4c00970020ffe3ff2dfff500fbff8cff74ff35ff9ffffaff8a00a7fffaffc1ff540003ffcbff690056ffdf00780050ff50008400abff4600d1005eff81fffd007a0007ff9600af0029fff5ff18ff8affb90052ff1900010015fe61ffd6ff19fffdffdb0070ff7cff46fff1ffdeffdbffee0041010cff36ff91004f0079ff6cffe10091ffedff9d002a00b200afffbf000cff9a0151ff72ff7a00f4ff4effdd007eff9100a80011ff9b00c1ff21ffa7ffc90096ff6dffb4ffd000a2ffa2ffd10068ffd200600120ff1aff0eff71012d003a007e005f00bf0008008d00420051ff500057ffde0099fffb01330085",
    "036f0921b89631213dc33f05707a0a3fece5aa9e507f6646b6f8443c27ccc843",
    "sample 4",
    "0a0051ff8c003fff05008bffa30078000eff18ffc000560016001500daffeb0060ff5effa6014900bc007affb600aaffe00013feff01130055ffa2004700cb005f0003004000cd001a002b0092008d014bfed800c9ff2cffa3fef901c1014d006efeb4ffa9fff701310072fffbffc0ff62ffea002e003d01400029fff3ffab00ee00730069ffedfeed0061ff69ff4d00f7ff30ffda0017ff600014009f00d5ffd0fff000b6ffe8ffefffee00290005ffc4fffcff9b00bf00bcff41ff8eff7affb5ff97ff59ff36ffbd007effeefeb8005e0017ffcb003dffc801dc0096fed2ff3eff4effad0021ffbd00baffeb0070ffee007dff79fff6009b007dffc6ff77fff3ff7afe2500a1012effabffc8ff95ffe3ff79004d0060000900c3ff4bff4f000aff20ffc8ff4f004cfff900c0ff9a00280037ff3bff18ff7b0050ff2b0189ff8cff2c003d0026ff53006aff180042008dffbd007bff56008c00f9ff15fed6ff3f011afffc005effcbffc2ff30ffaeff65ff3dfff1ffabfffdff1dff870041fea00095ff5bffee009200c400c6ffbaffd400b9ff4d00a9007500c70010fffb004b0121ffdb009d006000c8ff5100a4ff02ffbd00bf006e0120015f00910051fe8d012a00f0ff0fffe1ff3d00a300deffbbff780044009e00670189002b0001ff65ff520093ff9f00bbffe6ff71002cff15fec000cf0086005cfef20084ff9cfeadffd4003400ff01080016ff98007700a8ff96001f00740126ff4700e200360028000c01170007007b0020ff88000100e9ff820005ffdcff4fff6500620101005200a2fffeffb0ff3affbf0051ffd2ffba00b5ffe6fff20078fffcffb20088feb100550068002f00ebff32ff25005b00300025fe4500ac00120004ff950095ff8c0021ff8aff2b0067009c0003010c00f4000afff100610088ffec00e900fd019c00c1feec013b0059003cff22feccff16005600ce0069005400f600bb00030111ffb9fe5bff6901850089008fffdb0138000d004dff5bffdd00a4ff59ff9efe6300720197004e003600780130ffb000170172ffb40078ff8cffd8ffeaff9c000c0049ff62ff0400590048fefc0081ff87ff6600ec002500a6005e0197ff3e007fff87ffe9ff45008dffe600a600a5ffd800fb009aff7d00180115fea50073009f0019ff22ffceff200030ff3200a6ffaaff5bff7dff54ff03ff6200db0093ff2fff94ffceff87ffda00ad002a009200ca0182ffb0fff6ff8d015000c100fdff20ff34ff45ff0dfef7018600900019007bffb5ffe8ff8a002bffb700e5ffebfec8ff8a00e4ff7c00150013000d0092ffc6000b009dff9aff47ff1c00a3005d000dffcc0090ffcafe9f0058ff6d0027002dff9000d20027ffd9ff5b0119fedafebe006500e80129fea2ffb300b60169fffeffc100510058ff190055001bff9bffcdff81ff820101ff00ffd80136ff7effa8ffeafe8bff340096ff7bffba00f1ff75008a00bc007fff5cffcf0060ff3e00280110008200e9ff7b005200c1ffbffff800d90049fef0ff9e0155ff79ffb9ff72ff27ff4f00160003ffd600caffbaff9effd3005b000e00380009ff1b0198fffa00feff3c007fff42fff9ffd7ff80fea3003bff2bff2400cdfefa003bff8f00220026005e00b1ffc7ffeafeceffe40116002b0027ffa1ff8bffa2ffc2fec4001dfefcff8600c100b50049001fff68000600dffeec0116ff4dffc6ff95ff20ffedff5effd0004d00360035000cffc7ff42006700f9ff69ffec015e0088ffadff41ffda001bfffeff86ffa20031ffa90023ff9efe5f010d00a5011300b1000d0056ffffff59ff11001d0003006e00d0feedff4cff76ffefff26ff83013800b8ffdbffc7ff71feed00ff006200d8002400690017001100b5fff600fcff230073ff4800a300e60035ffd3ffae003001130116ff59ffa2ff45001d00590014ff2b00ebff66003fff4a007900e6ff810187001dfff4fedc00dafefdffe1fef500f9ff8ffff700c90088ffb4009d008effc5ff07ff71ff7affb30001fff500c5ff4e00050025003cffad00c3ff28fef10035fff4ff78fffcff5c0002001aff0b00c100a400d1012dffbf001afefd0008ffacff3ffeb1ff1affad00dafff6ff94003c00bbff4401360037ff88ffb4007b00430005ff1c0095ff2f0040007b00f2005500b6ffd6fe7e0068fe73ffb5ffd000b7003fff5efff0fe98002700a2ffe6fffb00bb0167ff0200b5002800fb027a0048ff55ff950008007cff91ffabff1a000aff9bfe5100d8001f004400abffda002cffc600d5ff35001bff9d005bffb3ffcdff9ffffc001f00bc0136ffffff2cff22016cff77ffc1ff99007800830027002cff0d00b1006fff43ff76003e012b00c7000eff32ffd300c6ff7effaaffbcff23ff6fff0f0007008800bc0044fea8ff1400c10058ff6dfff500b8016200ebff930063ff2d0078ff9dff5bff9d011cff9a00e60073ffd7ff59ffd40038007efee4ff22ffb80059ff80ffe8003f00930026006600c2ffa3fea3002cff2dff650026ff8f0137004300fdff60015f0056ff8900c90037007700ca005ffee00008ff8e0097ffd0ff6bff6500d9ffccff69ff69ffbd00040048ff3400b5018a0099ff9cffe0ff4dffc2ffe3ff89ff65ff7d007e007200b90094ff560061ffe4ffdd005e004000a6ffc600e6018900180009fe8800f9008d0092001f00d8007ffff001120001009fff99ff18003fff49ffd1ff68ffae0059ffffffbfff5dffb0ff62ffc5fe83003e0083ffe9ffe200c5007300bbffac004400a5ffd2fe680064008dffc1002a001500b6001affbeff82ffa401d50006fff40077ffe3000d006eff6cfff3ffe100cdfef0003f004e012ffefb0058fffa",
    "7cf467172d3c90505ee0ca12051777b73954d6ed8c82515ba2e4e6098f4d9cb3",
    "sample 5",
    "0affb4ff43fffeff7afff7ffc800b5ff74023000490021fed0ff1f0099ffcfff05ff28ff90fffc013dff9fffd4ffaa00fcfecf0008fffbff9bfe33000300b200bbff2b00f50082000afe9f011500a3ff6fff640091ffaa008a0009fef1ffbb0025ffca001b0024ff99003b0036ffc4009ffe76ff85fed80066ff7e00e50050004800defecfffc40069ffb8fea9001bff24ffa9ff3a0045ff0d0013ffccff83ff1d002dff9e01db002100d30061ff2c003dff780150ff19ff8a00b100750013fe3b0105ffda008cffb3ff93ff1c0058ff93ff1cffc10043ff910043ffa7ffecff5dff5f001cffb3ff540056ff5afee7ff01005500940030ff70ffd300a4fff50016ff39ff9dff6e00a8000300d1007900a1002cffe3008e01000063000effa70040003a00b50063feb4002a0055ff5fffc4018c01250082fec50075ffccff1bfff7fffe009fffcd001cffb8003e007900df00dd00f50015ff0c001300520020007dff750016ff5cffb400a4fe13ff640004fff700fa0017ffde000dff68ffbb00c90107002d00ea01cc009c00a5ff9cffd4003bffa8005effdefffcff9eff6aff7200440116ff0b00e10114ff65006bff86003cff57fff5ff9a00a3006dffb7ffeffeed002700200158fff9ff7afee20032002ffef8ff2c0003ff2d003dffc100a4ffc9009f007bfff5ffd400ecffa6ff79ffedffa000eeff52ffb0ffe5ffbb0160002c01a5fff3017cfee5ffa6ff96004aff6a0041000cffe9ff6b002e00d40035fff30084ff6eff7effb2fffd009a0071ff270046009b00baff8cff6701e5ffeeffee0109ffbefe8affa6ff2b004d00930032001a00fb0080ff39000f008500a8ffd6001c00670096ffa7fe90ffb90098ff02005100c900350051ffc4ffb8009affd7005400520015ffd000cc00bb0044008effbaff510001ffcdffc100ce0158ff210079fef7fee1fee100a9000100960150ffc100d3006a00390090ffbdff22007000a5ff8a009eff58ffca006500b7ffe90058ffedff450076ff0700d300f7fffc003bffe0ff0cfee1fff5fe66feb0ff96ff02ffd000b7ff4f0071ffcc00090054ffa5ff97000c01c9007c00f400abffee0038ffd4fffbff2bffd80034ffab01bd00a300b500e1ff9d0025feffff58ff89ffe0fe7bffb3ff42feaefef8006dffa3ff93ffd4feeffff500650095ffc3ffb7ffa00049007dff9c002aff7ffffbff540020ffb30064fffaffed00edffc1006fffe1ffd10012ff1fff9b0028fff00057ff5cffbf00cdff8d0043ffd900c0ff7fff87feb80074ff46006f006a0073004fffd3ffbcffb7ff2401ba001f01d2fff00011003aff68ff74013cff51ffe800f800310034ff9fff1f0031ff2c012700d700a20093ffc4000000300055ffac00e60079ffd5ff13ff4efe6e00c50128fff0ffbdff82002100560021ffa6ff9b00020008fff1008d00bc005cfe4c0055fecdff05007bffaf003affcffeaf0010ffa0ffafffa7002d004e008600ffffe9ffd30036006a0034004cff34ff90ff2e003fff78fff0008ffffbfed90075ffbe00a500990047ffcf000dff8bff77ff9dffd7007200800080ff6800f1ffc3002efffa0023ffb2ff650113ff9a0036010aff7effb5ffe7ffccff8b00db002bff56ffd400b3ffda008900ba0052fffbff710012ff7d0034ff9200f5ff5100b7ff0fffeefed3ffce00ff00fd002b0083ff98ffed00790082fedc0093003fffedffa2ff8b002dfffbff4800260089ffe5ff71fec6ffd6ffa90082002800dbff6ffef3ffac0025ffa30088ff180054ffc90062002d0054fff700e2fea30026ff29010f00cbff920032ff8f00b0fef4fff1ffc0ffaefe9a0079ff64ff81000a00a9000cff60fff3002dfffa007d00340032fe930008fff6ff1cff530150ff9c005cff66007101e5004afeccff6dfd9f00b50084ff89003aff33ff8b008dfecf019f008aff1fffacff75fec201e30050ff6ffff5fec300c700e2fed9ffe8ff84001800c6023000a30030ffe40010ff6300c9004f001dff5300b1006300a6ffd80099ffd50127ffae0050ff650086006b0046ff32002a001400e1ff990048ff41001bff5600590011017eff99ffe5ffd3fe6f0073fdf3002d005cff2b006a010d002cff7cfe7cffc4ff56ff940042ffa1ffd9ff6f001efef1feed003900610029ff520050ffd4001cffd700c9004eff70ff540020008e007aff51ffbf009dff35feb9ffb7011f009f00d300e200a3ffe6ff5600e8fe82fed7ffa6ffea01ee008dfefe001b005500beffac005efff9ffdbffe2ff72ff1e0009ff690034fedbff9700920090013c00a400650055fed8ff92ff38ffedffaafdf8ffae013d0019001ffff40076ffc0000800fbffd6ff1500f400bfff71ffb5001f0076ffd9ffd1ff8fff660121ff92007bff37006ffffbffab00ce0070ff2b00bdff16ff3c0086ff45ff69fdc1ff9800830089ffa4ff4700c8ff8b006f00d7ff49ff32ff33fea9ff74006e0089ffaeff7300c0ff36ffabffb7ffc2ff9bffd3ffd1fed101370161feebfff9001bfff9ffd3ff9f00590007fff0002eff4e0124ffec0016ffc8003bffbb006200dd00c6ffd6009fffd3fddeff2dff4effd2ff8affc40128002a00c3ffac0084ff40016b00800087001e0042ffb000c7ff0b017cff1cfffbff9c0045ff31005ffff8fff10136012affec0068001aff88ffd5fffeffd3ffe2001bffaeff78ff750135ff7e00bcff39ffbbffdefeecfff9fe95ff57ff73fe480013ff610063ff220054ff0cffa7ff7200a2ffebffebffc7002c000d006affbfff3f00d500660023fed1ffe2ff94ff09ff1900860010ffd0015effe800760038ff7a0097000800c6ffeb001dffdffffbffdeff4b004c004cffde",
    "acc78539c8b82aa74f435eab9a974b55b80b4258bf480c103a7e6575e31ec822",
    "sample 6",
    "0affaffee8ff30ff720158008400ccffbdff960148ffee00abff4bff8f00e1005afea10046001900390053009c015200d0013200d6fffa0031febe000b00d60113ff2200b600d4ff5bfebb00c3ff56ff55ffecfffdffef005bff650089000effcafff6002000daff31ff68ff6b0026000fffda0066ff05ff820083ffb50026ff40ff2600cbfff200c6ff02fff1ffb2000500d6ff670092ff54000b01180105ff9cff65ff9b00d700b00068ff43ffdbffe6003efefc0070ffc9ff7affda00460042007eff9600fbfeabfe66feddffb6004c0017ffe7001ffffe009301040021ff3d0084000c0161006ffe9c00db0026ffd8003d00e9fee70103006d00bc0027ff5500faff8efe9e00670052ff2fff36003bffce004affe2000d00db00040037009aff9affd100b1fe73ffd1ffa6fffd00aeff43ffdaff44005a00b9ff9f00f0007ffea500bfff9e00bfff9f0033fee9fec8006d005000a4004200d5ffa6ffc40064005e0010002f000e0035ffc8010fff9a00c700670005004eff84ff82ff5b00c500a6003cff880032ff4f0036ffccfffb00680047ff2b002c007500cbffa400b10048feb7ff710002fed4ff91fead0019ffe5ffa5fe9b00f0ffdeff6f00bd0074ff07ff91ffb3004200b0ff8dff6d001ffefa008e0013ffaf003500a600cf0073ffbefff800b9ff54ffb50016ffa7ffad00410045fec8006e01e00189001600850034ffd0fffcffd2002e0074ff6dff9c00ed00c200e1002f0061ffefffe20058ff92ffa9012cffc70036ffadfeed0023fed0ffb5002f005100c5ff16015700d4ffbdff6d00850012ffe7ff0800a6ffc9ff33ffcdff24ffc0ff7e002cfe7c01240090fffdff4f00a1ff000065004000160090ff75ffbfffc3ff00014800faff690047006affeeff8affe8ff65ffc900140061ff0c0030ffc7ffc7ffb4ff7c007b00ee00d4ff54ff800044ff8400ac007d0014006400130018ff0e00a000f80089008000610016fe41fff70081ff9fff92014fff82001601200035ff3200b100860108012f00d601d3ff490069ffc7ffaafed7008dffe200270048ff48fffaff5d003e00a4000affd0ffe0006eff7c00e60088ffab002bff3c0080005dffaaffe3ff47ff780134015000fafebb00310014ff1c0006ff900070010dffc6ff6300830167ffc1ffa2ffd8ff81ffd7ff8f00d100d5fe2fff47ffc2fecc0077ff83ff7bff6000230107ff8fffecffbc002b00aeffe6ffb400fd00beff05ff090045ffc7ff920005ffbeff6200fcff94ff1400a2004affc9ffe4ff14fe91ff0fff6b0001ff3dff64ffd70009ff7d02380004008f002000e0001f007bff5eff02ff2e005bff8affca00cd0047002a0109ffcdfffaffc300a5008c0032ff9e0074000dfef4ffaeffb3005bff61fec40074ff27feda0117ff4900a6fe3d005000080018ffd6feb200970082002800bb0015ffb900fdff7bff94ff68ffecffbeff3901d9ffd2ff85ff8bff540003ffceffa3ffa1fee00031ffd100a3007bffc10013ffee0039fe5500c30034008affb4fe45fffeff5b000fff86ffd600ad013e00a4ff7effe200330049ffb4ff9fff3bfff2ff22006eff930019ff2c0045ffa7009eff48007b00cc014400c6ffdc007e000b008d00b700bc0035ffc0ff57ffc8ff58ffee00d7fff0ffb200d9ff28ffc20011ffae0003fffc00edffd1fed2005ffffb00e9003f0086ff3900110101005f00350014000a00ab0098fff3fe20008b00b8ffe000bcfeef005200adffd0ffc7ffb7fff301a900d2003dfff5ffd80095ff2b0037ffa90045ffa3fe6800b80003ffa6ff31fff9ffa6ff1300b8ff80014cffedff95000bff2cfeecffaaff60ffd2fea0fff3ffff001aff5f00610068ffbcfef90061ff70ffb500a1ff1c0110ff6b005f00b10195ff94ffa4ff8effe5007b00a4ff68ff9800b0009800a500d0ff74006bff5cff47ff82ff16ffcfff3a01b5fee6fff0fde7fffe003400070006ffcb00b6009d001f0086fdebfff3ffc3ff6b0129ff3b007f00f6ff9cfe3bff8a0003ff7a0062007effddffb90091ffe0ff9afe30007e0018ff550064ff22000100cffff9fff1ff440047ffe0ff91004f0027ff4b003d00ee00040118000aff2ffebb007500dbff140027ffdc0018ff86fe5a00610024008dffaa008500ce001fffe2ff5dff9ffe7aff6f0012fffd00360041ffb0ff610066ffd00041002c017eff79ff44ff1b007f0034fff40036ff9700860083ff16011d00a10024011dff6b00c5ffa8ff36008eff92009c00c8ffb400550048001fffd0ffabfff0ffc9ff0e0053ffd1ffd0ffd2ff6dffac00500059ffa600b8ff42ffd200daff9d003b00b3ffe3ff89005eff0cff3fff31fefaffcbfffdff90011affd000ea0088ffd0ff02004a0031ffa00077005cff00000d0082ffbd006affb5ffb6ff9a000dff58004efe570034ff90004b0103ff6f013d005cff79ffdcff110022ffe90088ff5a00770108ff76006e01a20018fdd600870021005effa5002600bb0096fff1ff91002f0103002f00140038001cffaf0120ffee0024ff8f013cffbdffc9006bff67ff76ff5cffb80076fff4ff8cffbafee3006aff21004a0058feae0051ffeeff5fffdc00e5fe03fef40004ffdc00940018005d0045ffb6000400b3ffaaffc7007300e4ffa6ff14ffb20052ff6aff6d0045ff91001afefbffc4ffca0030ff92ffc70043ff8700b7003dfed20067ff18ffcbffa300a100910045ff2cff4c004dfff5ff9900ac008c001800ee003f000c0101008d008b00240095ffc2ff380098007bff550127fe710086ffdcfffa005001150010005afe90011bff5eff69ffd9012effc5ff07fffbffc60091ff5fff75ffd200d1007afffeff8f0053",
    "519bf3fa8785ccb4b95f2ef0d3f0168245218323450be56a4a5660b9fe097d27",
    "sample 7",
    "0affc1ff420024fec7000bfea1004e00560023fff40121005eff45ffdbffeaff32ff100024ffaffe8affff012fff4dffe600a0ff6d0016ff6aff24ff1cffc2002ffebdffe2ffbc012eff45ff95fee7009dfec7ff0b00b9ff24ff03014f0070fff3ffc10053ff8cff8800fa007a0004ff5300bf002aff41ffbc00590009004dff45ffea004efec6007eff8a011201740091ff850121002b018a00050016ff00fe650055002cffa9febe002bff84ff570105ff9f002f00d6007e00780057fedfff020033ff14ffdafff0fee0ff83ff5b0009002c00cdff99ffa9ffceff8f000600bb000a006bffa500ed00a0ffd600bbff3e005501bb0054ffaf00ecff92ff23ffecff9600bdff26fe72fed3ffd00051ffb300030070ff60fff5ffef0052004100900151002bff97007dfff5ff5c0024ff7eff90ff58ff900138001c000cffec007b016700dc00ca0034ffce0163ffe6ffc3ff400019008f0063ffaa00f700a400b7ffc70035ff6a006bfedfff0dff69fff5007e006000c10007ff60ff9100c7ffa2fff200ccffa80073ffcfffdd00e40000003a006300c4002d009ffee8ff2bffb3fff8fefb012100e2ffc800ef007dff05000affbaff3000b5ff52ff17feceffbeff85ffadff65008bff4affdaff8bfee0ff270014ffb90141ffb5ff210049008f004d002cfedfff86fed80011ffecff42ffc300bcff44ffd800a0fee5ffd6fffcffb3fffa0005ff650091009bfef1ff61ff170023ff83ff4800d5ffc1ffe80045ff20ff9800b9ff7e012fff7100e1ff4bfff7fefcffb1ffd0ffbe0035ffd8ff9700a6ffbeff62fff900a2ff2f008eff3a00090084ffb30021ffefff6bff720049ff3f0009001cffe1ffc0ff3ffea5ffa2ff30ff15ff6b0047ff6c00deffefff2bffaa0022fff100f2004d00cdffcdff91ff6c00a200a30030ffca002dff810049ffe2ff550149ff3e0050ff20ff17002400700103000fff57ffd7ffe300af00810154ffd9ff2000a8ffbc00310041009dfee1ff0b011dfed70012ffbb0041ffb7ff2e001e006fffd700cffea7fffe00ccffda00440063ff3cff03ffcb003900c300f400b5fec1ff78ffdb007500b3ff9f0012ff8bffc80089ff51ff76004aff0eff7eff1e001e009d004a006affa8fff10093ff97febb0075ffdaffbc0095fff0ff7cffa80027ffa1005e00150052fee4001a00380018ff6800fd0095007f0089ff94ff98009cffb1ffc6ffe60059ff2affb1ff45ffb20178fecfff39ffb0ff49ffd4ff30001001bc00caffff002efe70ffdefea80048ff6d004effc8ff43009c0157ffa700c9ff660087003000c1ffa30099ff63ffb9008ffec3fefdfecaff3c0078fff3000bffac000d00610002ffec006400150066ffd700f6ff74ffbeffc30015ff8a009dffc9ffb800ed00cbffcfff3d003500d4ff920058007e005900a9ffd4ffb800a1001600b5001b000dff740137000fffa10002ff540023008fff6b0082000affc50039fefd00bffeddfff7004800a1001e00e70019ff5b0068ffd0ffe9ffe7000c003dff86ffa8fedb0050004a0011002afe7affcefe7d002bffb9ff6800b1ff47ffd10167ffe0ff0eff620095ffe3ff5afe10ffdd00740156005900890111ffe8ff23ff80ffeeffd7ffc9ffdb000a00ae000c003b00e7ffc1ffb300afff9aff4affbbff70006501580044ff9a006a005affbdff8dff4cffadffaeffbcff91ff4b010500ddfff7015f0004ffbbff7f000fffbd018dfeedff1b00a7fef9ff93ffeefe40003bffc90013ff99ffc400ddff41feac0085ff9c009bff2fffc90007ff5301b90021000c000fff6cfe2effd30020002b004dfedafff1004700c100aeffad00c6ff50ff9300370095015eff25005dffebff1cfff900b60011ff08004800c1004a00a1002e0015fef9ffffff7bffdf005100090056ff5dff11ff76fe6aff60fff5ff76007600d600080030ff7b00d8ffef008fff00ffebff37ff5f0046002fff85003f00b0fdd5ffb9010b0057ff6cff41fea800bb005bff11ffa10033002500190014007b00810075ffca00970048009d0115fff9ffceff040040008dff6f003e000a00c700e5ffb2fe8100080037ffd8ffddffc3ffcb0010ff530010007400d70058003cff980032ffc9ffd8ff540081ffb20047ffeefef2ff310064005a0072ff86ff9d006eff4eff59fead00e400b2feca01a0ff83ff20002b004ffff10078004600c6ffa90078fff20008fee1ff8c0072002ffeb5ffe5ffe3ff7c0079ffdc012d0019fffbff5bffacff60ffa000690082fff9fe6afed7ff95008e002dffac009bffe90092ffb8ff4e0042fefeff9c006201b60112feda01950035feaf00a9006e00b8fe14ffa40095004aff4100d7001700ed012000710026fefdff750049010bff5cff32009500040027009dff3a0051ff3cff94fffffee3ffceff60000fff7effa20018008bff7a007a0033ffbb011dfee2fff1fead002e00840105ff3bff66ff7e008dfed5fffd00affe79010701740015ff8e00f2002200640024feccfff500da0049fffdffa200c00058003dffccffc0ff330090ff5000cdff72ff9cff3fffb90020ffcaffe300f70075ff6bff98ff4700a70020007f00b7ff8bff3b01020099ffe0ff5800bc006affda000fff4a0077ff9affb7ffe800d6fffb0064009a0021fffbff83ffc30023ff2c017900380051ff9bfeaf006a007c007200c100ca006cffa8ff9e00890006ffe2fed6009d011d01030047002dffbbff10ffcefeb7009400a4002cff9a004b00d0000b00fdff7affc4013e0024008fffc3003200090093ffcbff8a0049fffa0053ff9effd10093010eff57ff88005e00f3009a0158ff3dff9000050064ffb6005d0027ffc700a6ffe9002e",
    "b4c3bb96b3614ca047ee15ce94f3ecce0f0451dfb00877611170e56017e76269",
    "sample 8",
    "0a0077ffcb00020072ffc2007400c4fea600ec002c00280076002e0075000c0045fffe00c5ffc2ffdeff9b001fff6600dfff0900e6ffae011d005dff840022ff36ffb1ff97ffd5ff0dff9f00a0ff6200b00070ffbcffab0055002dfeac0021009a001f0095ffdf00c2ff2bff5d00f7ffde0003fff00100ffbc008900a2008700290165fef6ffa400a5ff18ffcb006c00af001b0089ff99ffccff9f012700c1ffaa0074ff9affcdfeecffd9fe64003a000aff840085005c00cc0039ff4fffd9ffc900a50144fee700db00a900d1015b003bffbfff80fff0ff2eff0f003300090160006bffbbffd5001400ffffec0000fef5000100050033007aff90feaeff0f0047ff320028ffbb00d2ff9c003300bc00e3ffd200060061fe5d00f2ffa6ff8a003eff9affa6fff9ff6aff270037000cff94ff87000e0040ff8eff16000c0069007fffb2ff2b00b8fef8000f007dff3c003f00780028000cffe00079fe66ffd5001eff40fe1fffc50107ffc7008cfe54ffe4ffcd006c0084ff49006fffe5ff670024fef000a4ffe0ff90fed00051fec8ffc2fff30017ff1affd10097ffe6ffbb00afff8b00870046ff050011008efe9e003ffffdff75ff5bffbefff9ffb100c3ffdb004dfeb800270001ff5d006700fb010cffe700aeff990060002eff0900a1fe63fff600c3ff130043009b0177ff4d0024ff50ff45ffafff72fef2fe8fffe7ffc600d7ffb000c20031ffcaffbe002e010bfffbff1e0082fef2fff6ff460031ff650087ff520093011400b600de0007000eff4aff30ffea00540083008fffb0009300b80057018dff1bffdeffda00c6ff87005eff3dfed7ffc90093ff67ff09ff69001c00e0004cff93007affe0ffaeffe4ff140042ffb6ff0f006f00690057ffd0fecafff4ff38fe2f003fffa800930033fed3ffa0ffcaffcd00060032008fff67ffebffa6ffdcffc2ff87000bff1dff9800f900a6ff2400d6fffa0050fecbff74ff86003a010400ddfe5cffbc006a00cf003c0029fffa00dc0075003f000e0054ffa0ffbf010effbeff1dff480002ff87ffaeff3a00c8ffb6ff92008eff850096ffaaffe8ff95ff47001200fe01020084fea4ffd0fe8a00c0009f0079ffefff4a00efffc7ff5efeecfe9cffba015a0030ff2bffbd0005fffbffefffd30072feb2ffc100bfffd6ff76008f0065ff55012a003b00aaffc7fee1ffb2feaaff2eff7f00c5fff00044ff7cff9e00a90034006bff48ffff00e700610036fea200cffeb600bcffdbff8b001bff870012008d0019002fff2bff2dffd3ffbcff59002e0101fe8efdad00b3ff98ff3effe201140130ff53ff9a0116007aff29ffc8000a003b00ba005bffcb010bff7a0052004700e5ff7dff830099003c0036ff70ff700000005a010cffbfff8e0037ffc8ff2600c0ff54001d00abffa00024fffffff600c1feffff87fedbff88ffde000200be0005003cff5a004700f100deffe9ffe300a4fe97ffaf00560097fed30062fed500ddffdb006300dc00670148ff13fee7ffeefe96ff59ff5bff47ff79ffc100e1ff61ff33007e00afff51fff7ff93ff900017ffec008701bdffaf003f00db01660078fff70000ff8aff260090005fffc4ff670039ffa0ff62009300f3ffa701b0ff5fff26ff81fff900a8004aff13ffb80086ffd1006f00170074ff440152003a0008ff36ffe600d1ffc9ff20ffe6fefe000601ce0000006dff7affb9006fff37ff20ffe2004fff9d0182fefdff240019ff44ffb300b6ff19ffd8ffd000180051007f0016ff46ff2100760135ff9401100021ff00ffda000200e5ffc2fef3ff8000e3ffab014800d700a6fff9feb200a2ff9e005c004f011fff9cffe3ff91009dfff6002ffed6fed2005100c3fff2ffcdff6f00a400a4ff8f002a0137fe4c0030fead0050ffce005400010009ff4900be0100ffad00550047ff1500c600ac000b0061016b00ccfff701b6fed80025ffd20093ff19ff34011300aeff6dffe7ffc60002feebff7bfe6aff6fff8aff4c005dffde000500430039fe6a012eff83fea1ff9cff40ff7cfef0001e004aff89010400afffb2007f000c014e00b200e0fe3bfe3eff6fffb100fc0052ffee0028ff6c008effe5ffa8ffc7ff4bff20009aff1aff870121feddffc100d200c6ffd4ff30fff4fed1ff28ffd000fd00f0ff67002e002700de0057ff9efff200acfff5ffc5ffc500920116ffccffbcffdbff6cfeccfe970088000100b4ff97003ffff10075ffbc00010012ffc60097ff1f00dfffd4ff94ff9efe6400350035ffc6ffccffb000bb00280008003f00f6008ffee6ff3cffa7ff97fff20004004bff170032fec30054ffeffee0003cff4800d7feba0000ffe7005201820022ff96005400890011001aff380057fed9ff11011c0022009b002c000efe8eff31ff3f006b00e70037ff230075ffb3ffc600a1fee70051ff2dfffa003f00f9005d0045009bffd0ffc7ff92000c00b300e1ff6f006a0040ffc6ff5aff8f002d009dfefafe15ffe6fe92009a0077ff82ff73ff0cfee8014cff9000e4ff75ff520058006200dcffabff47fffc004c005affb00004ffe6ff2f00250046ff20fefaff6c006c00370049ffb3ffe2ff89ffa6ff7fff37fec7003a0059ffe800b70072ffe9fffcffe800ee007700edffbdff9500abfff2fef200670004fecaffd0fef50107ff3aff79ff97ff0e003fff8300680036006f019f0021ff6dffc1ff0fff45ffa4fee2003f005c008500b40151006bff5dffbbff8b008ffec60003ffac003c00f600430002016b00ec0039011a0159ff1100b7fff80094009700480096ff27ffea00460163ffef00a400fdff9cfe71ffcfffffff4cffc6ffe9ffb7ff99005a007f005bfebb0089ff50fff9",
    "2b982d5b2aef3932ad1c2aa72cdc2bd8d5badd60d19508d5b7832f12867a41ce",
    "sample 9",
    "0aff9a001f0058006f008dff280022ff090011010b006e006bffee0009000f0067ffa2ff44ff8000f4005600ff0014ffc80008ff920049001400f4003e00610054ffdfff47ff4dff91ffa800e40065fdee0075002e0044ff40006f0001ff5e004900b6009bff5c00fcffa4ffd100a6fec80015ff68003300f1ffc3ffc3ffe5ff56002dfec30030ff22ff0bff77ff5700f0ff130066ff4bffa8003fffd8008ffe5c001700a70172008f007bffcb00b50007ffe30050002f0075ff6eff69009300eb0039ffbcfff9ffe0010200250051004a0011ffc5003afefc0026ff7bff88ff8e00d3003cffb9ffbaff87ff9e00fd001bffe5ff31ffef0012ff21002b0099ffcfffa700550003ff6bffe30037ff20ff800073ffca0065ffc0ffa10073ff0f00600048018700b10109ff8cffbbff9e010afffc0022ffc1ff0affaaff80ff4fffe400fa001b0075ff87ffc7ff8fff61001d0019ffea0194002200bd0135ffc8ff6bff73ffcaffb9ffb400e700b20065ffedffbbffe200b20057ff32ff8dff7b00260029ff510076ffb2002100f1ff4ffe3b0027002eff6800d5ff0300910016ff0400ec005cff45ff7f001e0021ff8900420086011f003dfff5002a008200980000ffcf0035ffaf0065ffb5004fffe5ff5eff990063ffc1000dffa9fff9000bff5900f30131ffd6fff20058ff9d0056ffdcffc501f200cf0106002cff50ff1000f0ff170025febeff9500750022feb70082ff48ff2200bbff18006f006fff75ffab009600790060005b0012004300cdffcc00b8007f0125fffeff8d000cfff4ff97ff880070ff83ff26012fffd5ffb2ffeb002d00c900d1ff8900a1ffa60133feffff64ff79001c0053ffb80161ffd6ffb7fff900cbff84ffbb008affa201ad00d00073ff01ff6a0017ff5b0016ff8d0019feec003dfffb01cb0094ff3f000cff92ffd8ff650044004b0003ffc600080080ff8c00f1ff2600a1ffd0ff58ffecff48001a00dbff4dffd6001cff5a0157ff690177009eff2cffadff62fee60027ffb00034010100780127ffbc0052003e000c00e1ff62008bff250016fee3ffe200a8001effad00efff19ff66005d00b2fed600700112ff5eff690073003901a5ffe9ffb60013ff4bfeb2ffe8ff2cffefff14ffa4004affdf006c016efed900b5fef9ff5effbd006c0068ffedff6100bf0138002bff7f0016ffb90066012100ef0101fee601490024ffb4005e0038ff1700c3ff5f00d0002e001800ec00850049ffd8ffe8ff90ffb6fff5febc0078fed2ff5dff95000500920084ffe60194004dfec7ffb300e2ff97ff06ffff00caffa3fed00012ffddffe0002700b3ff4400cbfeaf001dff170064ffd60166010b0019001500050085ff0cffc50022ffa4008f0071ff5c0037ff23ffc10060fe8c01070075ffe0ffc90066ff04ff47ff9d00630068001700d3ff7bffce0028fedbff26ffffffdaffe2005f0033fef0fffcfec9ffb3ffe6025100660056ff83ff3e00300097fedc008fff940098feafff81ffb1ffa5005cff98013a00260024007f0031003900cd00ad0075ff69ff860016ff2300a6ffabff9fff8100480056ff97003b003cffc20084001c01090057feffff610051fea6ff8effe3ff840072fffd00a5005eff770160ff320118fedefffffede007b0006ff80000f00c50119ffc2fef9008bff5e022aff6200caff03fff3fed2ff550007ffd5ffb2ff11001600f6ffd2ffc1ffad006a003aff280035ff49fff3ffa0ff0e00baffea0095fed700a9001aff1200b3010fff3a006801170042ff480069ff610109005f002d00220055001fff400068008cffb60070fffdfff60010ff8f0009ffdcff3e0051fff2ff4bff4000c5ff9d00b7009b00f200aa004fff4900fe012c00be000e0060003b000c005c00e0ff2eff6bfefe01b6ff9d00c100a0006aff51ffb3ffde002f00a200c7002bfebe0016ffe0ffbb0093ff2200abff8cff17ff8d002fff0afff8feda006eff040082ffb50058ff73005e00cb0091ff7500e2004a007b002cffacfffcffa001b5009300d300700002ff740009ff9dffe500e400770033006bff7cfea0ff48007dffec002cff8300f6ff34006a011afffc00eeff1000570061006a0028ff92003cffbd001a001100a7ff5b002effcf00bcff7dfff8ffce0030ffc6ff3700bf00070006ff7b00acff53004bff54007c00680134fff80138fe7cff9e002fff700091fff701ad003a007efde8fff30031fffaff6dfffafee10086ffdcffc1ffc3ff37ffe500b00090ff82ff4d00920053ff880180fefefffdff4100c8004401440066ff49ff73000bff8eff50ffa8008b00dcff5effb0002dffd00024ff3bfefcffcb00e3003800f5ffadff57ffa3ffd40120ff63ffbfffebffd0ffc7ffe0006f0050009d00ce001e00a6ff13ff53001f006efed50085ffabfedfffa9ff46ffd500280092000c005cff6a0078ff50ff630037ffe9ff34fff9008effb500e1005e00af014800a8ff83005cff98ff6300a9ff2f0074ff770105ffd1fe6bffc3ffb6ff3100b4003b01030077009b011bffb900380040ff7600b60059ffc70092feee010dffd3ff70ffec0002004eff93ff86ff6f00d60016ffd90149ff7900440004ff3affb1ff76004dff67ffa4008b002b003d0190ff5301990022ff2800f00001fe6dfe90005f007bff280030ff65000b0160017f004fff6fff47ff97fe9d009100fc0009003bff86008500faff7dff4b009700320044ffd4ff13005200b2ff41011a002e002bffcffe95ffe2000effeefffa0035ff3c00b6009bffd0009a00d6feaeffb4ff17003cfec7ff2b0026ffdd0038fefa011fff400191ff0d00e6ffedff71001bff9e00a10000ff56fed1ffe10157fffa"
];