#![allow(dead_code)]

use criterion::{black_box, Criterion, criterion_group, criterion_main};
use rand::thread_rng;

use falcon::{falcon_privatekey_size, falcon_publickey_size, falcon_sig_compressed_maxsize, falcon_sig_ct_size, falcon_tmpsize_expanded_key_size, falcon_tmpsize_expandprivate, falcon_tmpsize_keygen, falcon_tmpsize_signdyn, falcon_tmpsize_signtree, falcon_tmpsize_verify};
use falcon::falcon::{falcon_expand_privatekey, falcon_keygen_make, FALCON_SIG_COMPRESS, FALCON_SIG_CT, falcon_sign_dyn, falcon_sign_tree, falcon_verify};
use falcon::shake::InnerShake256Context;
use falcon_masked::falcon_masked::{falcon_sign_tree_masked, falcon_sign_tree_masked_sample};
use randomness::random::RngBoth;

const ORDER: usize = 2;

pub fn sign_tree_10(c: &mut Criterion) {
    let mut rng = InnerShake256Context { st: [0; 25], dptr: 0 };
    const LOGN: usize = 10;
    let pk_len = falcon_publickey_size!(LOGN);
    let sk_len = falcon_privatekey_size!(LOGN);
    let tmp_len = falcon_tmpsize_keygen!(LOGN);
    let sig_len = falcon_sig_ct_size!(LOGN);
    let tmp_sig_len = falcon_tmpsize_signtree!(LOGN);
    let exp_key_len = falcon_tmpsize_expanded_key_size!(LOGN);
    let exp_tmp_len = falcon_tmpsize_expandprivate!(LOGN);
    let tmp_vrfy_len = falcon_tmpsize_verify!(LOGN);
    let mut tmp_ver: Vec<u8> = vec![0; tmp_sig_len];
    let mut pk: Vec<u8> = vec![0; pk_len];
    let mut sk: Vec<u8> = vec![0; sk_len];
    let mut signature: Vec<u8> = vec![0; sig_len];
    let mut tmp: Vec<u8> = vec![0; tmp_len];

    let mut exp_key: Vec<u8> = vec![0; exp_key_len];
    let mut tmp_exp: Vec<u8> = vec![0; exp_tmp_len];
    falcon_keygen_make(&mut rng, LOGN as u32, sk.as_mut_slice(), sk_len,
                       pk.as_mut_slice(), pk_len, tmp.as_mut_slice(), tmp_len);
    falcon_expand_privatekey(exp_key.as_mut_slice(), exp_key_len, sk.as_mut_slice(), sk_len, tmp_exp.as_mut_slice(), exp_tmp_len);
    let mut both = RngBoth { hal_rng: None, rust_rng: Some(thread_rng()) };

    c.bench_function("Sign tree LOGN = 10 masked", |b| b.iter(||
        black_box(falcon_sign_tree_masked::<ORDER, LOGN>(&mut rng, signature.as_mut_slice(), sig_len,
                                                         FALCON_SIG_COMPRESS, exp_key.as_mut_slice(),
                                                         "data".as_bytes(), &mut both))));
    falcon_verify(signature.as_mut_slice(), sig_len, FALCON_SIG_COMPRESS, pk.as_mut_slice(), pk_len, "data".as_bytes(), tmp_ver.as_mut_slice(), tmp_vrfy_len);
}

pub fn sign_tree_9(c: &mut Criterion) {
    let mut rng = InnerShake256Context { st: [0; 25], dptr: 0 };
    const LOGN: usize = 9;
    let pk_len = falcon_publickey_size!(LOGN);
    let sk_len = falcon_privatekey_size!(LOGN);
    let tmp_len = falcon_tmpsize_keygen!(LOGN);
    let sig_len = falcon_sig_ct_size!(LOGN);
    let tmp_sig_len = falcon_tmpsize_signtree!(LOGN);
    let exp_key_len = falcon_tmpsize_expanded_key_size!(LOGN);
    let exp_tmp_len = falcon_tmpsize_expandprivate!(LOGN);
    let tmp_vrfy_len = falcon_tmpsize_verify!(LOGN);
    let mut tmp_ver: Vec<u8> = vec![0; tmp_sig_len];
    let mut pk: Vec<u8> = vec![0; pk_len];
    let mut sk: Vec<u8> = vec![0; sk_len];
    let mut signature: Vec<u8> = vec![0; sig_len];
    let mut tmp: Vec<u8> = vec![0; tmp_len];

    let mut exp_key: Vec<u8> = vec![0; exp_key_len];
    let mut tmp_exp: Vec<u8> = vec![0; exp_tmp_len];
    falcon_keygen_make(&mut rng, LOGN as u32, sk.as_mut_slice(), sk_len,
                       pk.as_mut_slice(), pk_len, tmp.as_mut_slice(), tmp_len);
    falcon_expand_privatekey(exp_key.as_mut_slice(), exp_key_len, sk.as_mut_slice(), sk_len, tmp_exp.as_mut_slice(), exp_tmp_len);
    let mut both = RngBoth { hal_rng: None, rust_rng: Some(thread_rng()) };

    c.bench_function("Sign tree LOGN = 9 masked", |b| b.iter(||
        black_box(falcon_sign_tree_masked::<ORDER, LOGN>(&mut rng, signature.as_mut_slice(), sig_len,
                                                         FALCON_SIG_COMPRESS, exp_key.as_mut_slice(),
                                                         "data".as_bytes(), &mut both))));
    falcon_verify(signature.as_mut_slice(), sig_len, FALCON_SIG_COMPRESS, pk.as_mut_slice(), pk_len, "data".as_bytes(), tmp_ver.as_mut_slice(), tmp_vrfy_len);
}



criterion_group!(benches, sign_tree_9, sign_tree_10);
criterion_main!(benches);