#![allow(dead_code)]

use criterion::{black_box, Criterion, criterion_group, criterion_main};

use falcon::{falcon_privatekey_size, falcon_publickey_size, falcon_sig_compressed_maxsize, falcon_sig_ct_size, falcon_tmpsize_expanded_key_size, falcon_tmpsize_expandprivate, falcon_tmpsize_keygen, falcon_tmpsize_signdyn, falcon_tmpsize_signtree, falcon_tmpsize_verify};
use falcon::falcon::{falcon_expand_privatekey, falcon_keygen_make, FALCON_SIG_COMPRESS, FALCON_SIG_CT, falcon_sign_dyn, falcon_sign_tree, falcon_verify};
use falcon::shake::InnerShake256Context;

pub fn sign_dyn_9(c: &mut Criterion) {
    let mut rng = InnerShake256Context { st: [0; 25], dptr: 0 };
    let logn: usize = 9;
    let pk_len = falcon_publickey_size!(logn);
    let sk_len = falcon_privatekey_size!(logn);
    let tmp_len = falcon_tmpsize_keygen!(logn);
    let sig_len = falcon_sig_compressed_maxsize!(logn);
    let tmp_sig_len = falcon_tmpsize_signdyn!(logn);
    let mut pk: Vec<u8> = vec![0; pk_len];
    let mut sk: Vec<u8> = vec![0; sk_len];
    let mut signature: Vec<u8> = vec![0; sig_len];
    let mut tmp: Vec<u8> = vec![0; tmp_len];
    let mut tmp_sig: Vec<u8> = vec![0; tmp_sig_len];
    falcon_keygen_make(&mut rng, logn as u32, sk.as_mut_slice(), sk_len,
                       pk.as_mut_slice(), pk_len, tmp.as_mut_slice(), tmp_len);
    c.bench_function("Sign dyn logn = 9", |b| b.iter(||
        falcon_sign_dyn(&mut rng, signature.as_mut_slice(), sig_len,
                        FALCON_SIG_COMPRESS, sk.as_mut_slice(),
                        sk_len, "data".as_bytes(), tmp_sig.as_mut_slice(), tmp_sig_len)));
}

pub fn sign_dyn_9_ct(c: &mut Criterion) {
    let mut rng = InnerShake256Context { st: [0; 25], dptr: 0 };
    let logn: usize = 9;
    let pk_len = falcon_publickey_size!(logn);
    let sk_len = falcon_privatekey_size!(logn);
    let tmp_len = falcon_tmpsize_keygen!(logn);
    let sig_len = falcon_sig_ct_size!(logn);
    let tmp_sig_len = falcon_tmpsize_signdyn!(logn);
    let tmp_vrfy_len = falcon_tmpsize_verify!(logn);
    let mut tmp_ver: Vec<u8> = vec![0; tmp_sig_len];
    let mut pk: Vec<u8> = vec![0; pk_len];
    let mut sk: Vec<u8> = vec![0; sk_len];
    let mut signature: Vec<u8> = vec![0; sig_len];
    let mut tmp: Vec<u8> = vec![0; tmp_len];
    let mut tmp_sig: Vec<u8> = vec![0; tmp_sig_len];
    falcon_keygen_make(&mut rng, logn as u32, sk.as_mut_slice(), sk_len,
                       pk.as_mut_slice(), pk_len, tmp.as_mut_slice(), tmp_len);
    c.bench_function("Sign dyn CT logn = 9", |b| b.iter(||
        falcon_sign_dyn(&mut rng, signature.as_mut_slice(), sig_len,
                        FALCON_SIG_CT, sk.as_mut_slice(),
                        sk_len, "data".as_bytes(), tmp_sig.as_mut_slice(), tmp_sig_len)));
    falcon_verify(signature.as_mut_slice(), sig_len, FALCON_SIG_CT, pk.as_mut_slice(), pk_len, "data".as_bytes(), tmp_ver.as_mut_slice(), tmp_vrfy_len);
}


pub fn sign_dyn_10(c: &mut Criterion) {
    let mut rng = InnerShake256Context { st: [0; 25], dptr: 0 };
    let logn: usize = 10;
    let pk_len = falcon_publickey_size!(logn);
    let sk_len = falcon_privatekey_size!(logn);
    let tmp_len = falcon_tmpsize_keygen!(logn);
    let sig_len = falcon_sig_compressed_maxsize!(logn);
    let tmp_sig_len = falcon_tmpsize_signdyn!(logn);
    let mut pk: Vec<u8> = vec![0; pk_len];
    let mut sk: Vec<u8> = vec![0; sk_len];
    let mut signature: Vec<u8> = vec![0; sig_len];
    let mut tmp: Vec<u8> = vec![0; tmp_len];
    let mut tmp_sig: Vec<u8> = vec![0; tmp_sig_len];
    falcon_keygen_make(&mut rng, logn as u32, sk.as_mut_slice(), sk_len,
                       pk.as_mut_slice(), pk_len, tmp.as_mut_slice(), tmp_len);
    c.bench_function("Sign dyn logn = 10", |b| b.iter(||
        falcon_sign_dyn(&mut rng, signature.as_mut_slice(), sig_len,
                        FALCON_SIG_COMPRESS, sk.as_mut_slice(),
                        sk_len, "data".as_bytes(), tmp_sig.as_mut_slice(), tmp_sig_len)));
}

pub fn sign_dyn_10_ct(c: &mut Criterion) {
    let mut rng = InnerShake256Context { st: [0; 25], dptr: 0 };
    let logn: usize = 10;
    let pk_len = falcon_publickey_size!(logn);
    let sk_len = falcon_privatekey_size!(logn);
    let tmp_len = falcon_tmpsize_keygen!(logn);
    let sig_len = falcon_sig_ct_size!(logn);
    let tmp_sig_len = falcon_tmpsize_signdyn!(logn);
    let tmp_vrfy_len = falcon_tmpsize_verify!(logn);
    let mut tmp_ver: Vec<u8> = vec![0; tmp_sig_len];
    let mut pk: Vec<u8> = vec![0; pk_len];
    let mut sk: Vec<u8> = vec![0; sk_len];
    let mut signature: Vec<u8> = vec![0; sig_len];
    let mut tmp: Vec<u8> = vec![0; tmp_len];
    let mut tmp_sig: Vec<u8> = vec![0; tmp_sig_len];
    falcon_keygen_make(&mut rng, logn as u32, sk.as_mut_slice(), sk_len,
                       pk.as_mut_slice(), pk_len, tmp.as_mut_slice(), tmp_len);
    c.bench_function("Sign dyn CT logn = 10", |b| b.iter(||
        black_box(falcon_sign_dyn(&mut rng, signature.as_mut_slice(), sig_len,
                                  FALCON_SIG_CT, sk.as_mut_slice(),
                                  sk_len, "data".as_bytes(), tmp_sig.as_mut_slice(), tmp_sig_len))));
    falcon_verify(signature.as_mut_slice(), sig_len, FALCON_SIG_CT, pk.as_mut_slice(), pk_len, "data".as_bytes(), tmp_ver.as_mut_slice(), tmp_vrfy_len);
}

pub fn sign_tree_9(c: &mut Criterion) {
    let mut rng = InnerShake256Context { st: [0; 25], dptr: 0 };
    let logn: usize = 9;
    let pk_len = falcon_publickey_size!(logn);
    let sk_len = falcon_privatekey_size!(logn);
    let tmp_len = falcon_tmpsize_keygen!(logn);
    let sig_len = falcon_sig_compressed_maxsize!(logn);
    let tmp_sig_len = falcon_tmpsize_signtree!(logn);
    let exp_key_len = falcon_tmpsize_expanded_key_size!(logn);
    let exp_tmp_len = falcon_tmpsize_expandprivate!(logn);
    let mut pk: Vec<u8> = vec![0; pk_len];
    let mut sk: Vec<u8> = vec![0; sk_len];
    let mut signature: Vec<u8> = vec![0; sig_len];
    let mut tmp: Vec<u8> = vec![0; tmp_len];
    let mut tmp_sig: Vec<u8> = vec![0; tmp_sig_len];
    let mut exp_key: Vec<u8> = vec![0; exp_key_len];
    let mut tmp_exp: Vec<u8> = vec![0; exp_tmp_len];
    falcon_keygen_make(&mut rng, logn as u32, sk.as_mut_slice(), sk_len,
                       pk.as_mut_slice(), pk_len, tmp.as_mut_slice(), tmp_len);
    falcon_expand_privatekey(exp_key.as_mut_slice(), exp_key_len, sk.as_mut_slice(), sk_len, tmp_exp.as_mut_slice(), exp_tmp_len);
    c.bench_function("Sign tree logn = 9", |b| b.iter(||
        falcon_sign_tree(&mut rng, signature.as_mut_slice(), sig_len,
                         FALCON_SIG_COMPRESS, exp_key.as_mut_slice(),
                         "data".as_bytes(), tmp_sig.as_mut_slice(), tmp_sig_len)));
}

pub fn sign_tree_9_ct(c: &mut Criterion) {
    let mut rng = InnerShake256Context { st: [0; 25], dptr: 0 };
    let logn: usize = 9;
    let pk_len = falcon_publickey_size!(logn);
    let sk_len = falcon_privatekey_size!(logn);
    let tmp_len = falcon_tmpsize_keygen!(logn);
    let sig_len = falcon_sig_ct_size!(logn);
    let tmp_sig_len = falcon_tmpsize_signtree!(logn);
    let exp_key_len = falcon_tmpsize_expanded_key_size!(logn);
    let exp_tmp_len = falcon_tmpsize_expandprivate!(logn);
    let tmp_vrfy_len = falcon_tmpsize_verify!(logn);
    let mut tmp_ver: Vec<u8> = vec![0; tmp_sig_len];
    let mut pk: Vec<u8> = vec![0; pk_len];
    let mut sk: Vec<u8> = vec![0; sk_len];
    let mut signature: Vec<u8> = vec![0; sig_len];
    let mut tmp: Vec<u8> = vec![0; tmp_len];
    let mut tmp_sig: Vec<u8> = vec![0; tmp_sig_len];
    let mut exp_key: Vec<u8> = vec![0; exp_key_len];
    let mut tmp_exp: Vec<u8> = vec![0; exp_tmp_len];
    falcon_keygen_make(&mut rng, logn as u32, sk.as_mut_slice(), sk_len,
                       pk.as_mut_slice(), pk_len, tmp.as_mut_slice(), tmp_len);
    falcon_expand_privatekey(exp_key.as_mut_slice(), exp_key_len, sk.as_mut_slice(), sk_len, tmp_exp.as_mut_slice(), exp_tmp_len);
    c.bench_function("Sign tree CT logn = 9", |b| b.iter(||
        black_box(falcon_sign_tree(&mut rng, signature.as_mut_slice(), sig_len,
                                   FALCON_SIG_CT, exp_key.as_mut_slice(),
                                   "data".as_bytes(), tmp_sig.as_mut_slice(), tmp_sig_len))));
    falcon_verify(signature.as_mut_slice(), sig_len, FALCON_SIG_CT, pk.as_mut_slice(), pk_len, "data".as_bytes(), tmp_ver.as_mut_slice(), tmp_vrfy_len);
}


pub fn sign_tree_10(c: &mut Criterion) {
    let mut rng = InnerShake256Context { st: [0; 25], dptr: 0 };
    let logn: usize = 10;
    let pk_len = falcon_publickey_size!(logn);
    let sk_len = falcon_privatekey_size!(logn);
    let tmp_len = falcon_tmpsize_keygen!(logn);
    let sig_len = falcon_sig_compressed_maxsize!(logn);
    let tmp_sig_len = falcon_tmpsize_signtree!(logn);
    let exp_key_len = falcon_tmpsize_expanded_key_size!(logn);
    let exp_tmp_len = falcon_tmpsize_expandprivate!(logn);
    let mut pk: Vec<u8> = vec![0; pk_len];
    let mut sk: Vec<u8> = vec![0; sk_len];
    let mut signature: Vec<u8> = vec![0; sig_len];
    let mut tmp: Vec<u8> = vec![0; tmp_len];
    let mut tmp_sig: Vec<u8> = vec![0; tmp_sig_len];
    let mut exp_key: Vec<u8> = vec![0; exp_key_len];
    let mut tmp_exp: Vec<u8> = vec![0; exp_tmp_len];
    falcon_keygen_make(&mut rng, logn as u32, sk.as_mut_slice(), sk_len,
                       pk.as_mut_slice(), pk_len, tmp.as_mut_slice(), tmp_len);
    falcon_expand_privatekey(exp_key.as_mut_slice(), exp_key_len, sk.as_mut_slice(), sk_len, tmp_exp.as_mut_slice(), exp_tmp_len);
    c.bench_function("Sign tree logn = 10", |b| b.iter(||
        falcon_sign_tree(&mut rng, signature.as_mut_slice(), sig_len,
                         FALCON_SIG_COMPRESS, exp_key.as_mut_slice(),
                         "data".as_bytes(), tmp_sig.as_mut_slice(), tmp_sig_len)));
}

pub fn sign_tree_10_ct(c: &mut Criterion) {
    let mut rng = InnerShake256Context { st: [0; 25], dptr: 0 };
    let logn: usize = 10;
    let pk_len = falcon_publickey_size!(logn);
    let sk_len = falcon_privatekey_size!(logn);
    let tmp_len = falcon_tmpsize_keygen!(logn);
    let sig_len = falcon_sig_ct_size!(logn);
    let tmp_sig_len = falcon_tmpsize_signtree!(logn);
    let exp_key_len = falcon_tmpsize_expanded_key_size!(logn);
    let exp_tmp_len = falcon_tmpsize_expandprivate!(logn);
    let tmp_vrfy_len = falcon_tmpsize_verify!(logn);
    let mut tmp_ver: Vec<u8> = vec![0; tmp_sig_len];
    let mut pk: Vec<u8> = vec![0; pk_len];
    let mut sk: Vec<u8> = vec![0; sk_len];
    let mut signature: Vec<u8> = vec![0; sig_len];
    let mut tmp: Vec<u8> = vec![0; tmp_len];
    let mut tmp_sig: Vec<u8> = vec![0; tmp_sig_len];
    let mut exp_key: Vec<u8> = vec![0; exp_key_len];
    let mut tmp_exp: Vec<u8> = vec![0; exp_tmp_len];
    falcon_keygen_make(&mut rng, logn as u32, sk.as_mut_slice(), sk_len,
                       pk.as_mut_slice(), pk_len, tmp.as_mut_slice(), tmp_len);
    falcon_expand_privatekey(exp_key.as_mut_slice(), exp_key_len, sk.as_mut_slice(), sk_len, tmp_exp.as_mut_slice(), exp_tmp_len);
    c.bench_function("Sign tree CT logn = 10", |b| b.iter(||
        black_box(falcon_sign_tree(&mut rng, signature.as_mut_slice(), sig_len,
                                   FALCON_SIG_CT, exp_key.as_mut_slice(),
                                   "data".as_bytes(), tmp_sig.as_mut_slice(), tmp_sig_len))));
    falcon_verify(signature.as_mut_slice(), sig_len, FALCON_SIG_CT, pk.as_mut_slice(), pk_len, "data".as_bytes(), tmp_ver.as_mut_slice(), tmp_vrfy_len);
}

pub fn expand_key_9(c: &mut Criterion) {
    let mut rng = InnerShake256Context { st: [0; 25], dptr: 0 };
    let logn: usize = 9;
    let pk_len = falcon_publickey_size!(logn);
    let sk_len = falcon_privatekey_size!(logn);
    let tmp_len = falcon_tmpsize_keygen!(logn);
    let exp_key_len = falcon_tmpsize_expanded_key_size!(logn);
    let exp_tmp_len = falcon_tmpsize_expandprivate!(logn);
    let mut pk: Vec<u8> = vec![0; pk_len];
    let mut sk: Vec<u8> = vec![0; sk_len];
    let mut tmp: Vec<u8> = vec![0; tmp_len];
    let mut exp_key: Vec<u8> = vec![0; exp_key_len];
    let mut tmp_exp: Vec<u8> = vec![0; exp_tmp_len];
    falcon_keygen_make(&mut rng, logn as u32, sk.as_mut_slice(), sk_len,
                       pk.as_mut_slice(), pk_len, tmp.as_mut_slice(), tmp_len);
    falcon_expand_privatekey(exp_key.as_mut_slice(), exp_key_len, sk.as_mut_slice(), sk_len, tmp_exp.as_mut_slice(), exp_tmp_len);
    c.bench_function("Exp key logn = 9", |b| b.iter(||
        falcon_expand_privatekey(exp_key.as_mut_slice(), exp_key_len, sk.as_mut_slice(), sk_len, tmp_exp.as_mut_slice(), exp_tmp_len)));
}

pub fn expand_key_10(c: &mut Criterion) {
    let mut rng = InnerShake256Context { st: [0; 25], dptr: 0 };
    let logn: usize = 10;
    let pk_len = falcon_publickey_size!(logn);
    let sk_len = falcon_privatekey_size!(logn);
    let tmp_len = falcon_tmpsize_keygen!(logn);
    let exp_key_len = falcon_tmpsize_expanded_key_size!(logn);
    let exp_tmp_len = falcon_tmpsize_expandprivate!(logn);
    let mut pk: Vec<u8> = vec![0; pk_len];
    let mut sk: Vec<u8> = vec![0; sk_len];
    let mut tmp: Vec<u8> = vec![0; tmp_len];
    let mut exp_key: Vec<u8> = vec![0; exp_key_len];
    let mut tmp_exp: Vec<u8> = vec![0; exp_tmp_len];
    falcon_keygen_make(&mut rng, logn as u32, sk.as_mut_slice(), sk_len,
                       pk.as_mut_slice(), pk_len, tmp.as_mut_slice(), tmp_len);
    falcon_expand_privatekey(exp_key.as_mut_slice(), exp_key_len, sk.as_mut_slice(), sk_len, tmp_exp.as_mut_slice(), exp_tmp_len);
    c.bench_function("Exp key logn = 10", |b| b.iter(||
        falcon_expand_privatekey(exp_key.as_mut_slice(), exp_key_len, sk.as_mut_slice(), sk_len, tmp_exp.as_mut_slice(), exp_tmp_len)));
}



criterion_group!(benches, /*sign_dyn_9, sign_dyn_9_ct,  sign_dyn_10, sign_dyn_10_ct,*/
    sign_tree_9,/* sign_tree_9_ct,  sign_tree_10, sign_tree_10_ct, expand_key_9, expand_key_10*/);
criterion_main!(benches);