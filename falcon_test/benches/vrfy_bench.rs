#![allow(dead_code)]

use criterion::{black_box, Criterion, criterion_group, criterion_main};

use falcon::{falcon_privatekey_size, falcon_publickey_size, falcon_sig_compressed_maxsize, falcon_sig_ct_size, falcon_tmpsize_keygen, falcon_tmpsize_signdyn, falcon_tmpsize_verify};
use falcon::falcon::{falcon_keygen_make, FALCON_SIG_COMPRESS, FALCON_SIG_CT, falcon_sign_dyn, falcon_verify};
use falcon::shake::InnerShake256Context;

pub fn vrfy_9(c: &mut Criterion) {
    let mut rng = InnerShake256Context { st: [0; 25], dptr: 0 };
    let logn: usize = 9;
    let pk_len = falcon_publickey_size!(logn);
    let sk_len = falcon_privatekey_size!(logn);
    let tmp_len = falcon_tmpsize_keygen!(logn);
    let sig_len = falcon_sig_compressed_maxsize!(logn);
    let tmp_sig_len = falcon_tmpsize_signdyn!(logn);
    let tmpvv_len = falcon_tmpsize_verify!(logn);
    let mut pk: Vec<u8> = vec![0; pk_len];
    let mut sk: Vec<u8> = vec![0; sk_len];
    let mut signature: Vec<u8> = vec![0; sig_len];
    let mut tmp: Vec<u8> = vec![0; tmp_len];
    let mut tmp_ver: Vec<u8> = vec![0; tmpvv_len];
    let mut tmp_sig: Vec<u8> = vec![0; tmp_sig_len];
    falcon_keygen_make(&mut rng, logn as u32, sk.as_mut_slice(), sk_len,
                       pk.as_mut_slice(), pk_len, tmp.as_mut_slice(), tmp_len);
    let (_, sig_len) = falcon_sign_dyn(&mut rng, signature.as_mut_slice(), sig_len,
                                       FALCON_SIG_COMPRESS, sk.as_mut_slice(),
                                       sk_len, "data".as_bytes(), tmp_sig.as_mut_slice(), tmp_sig_len);
    c.bench_function("Verify logn = 9", |b| b.iter(||
        falcon_verify(signature.as_mut_slice(), sig_len, FALCON_SIG_COMPRESS,
                      pk.as_mut_slice(), pk_len, "data".as_bytes(), tmp_ver.as_mut_slice(), tmpvv_len)
    ));
}

pub fn vrfy_9_ct(c: &mut Criterion) {
    let mut rng = InnerShake256Context { st: [0; 25], dptr: 0 };
    let logn: usize = 9;
    let pk_len = falcon_publickey_size!(logn);
    let sk_len = falcon_privatekey_size!(logn);
    let tmp_len = falcon_tmpsize_keygen!(logn);
    let sig_len = falcon_sig_ct_size!(logn);
    let tmp_sig_len = falcon_tmpsize_signdyn!(logn);
    let tmpvv_len = falcon_tmpsize_verify!(logn);
    let mut pk: Vec<u8> = vec![0; pk_len];
    let mut sk: Vec<u8> = vec![0; sk_len];
    let mut signature: Vec<u8> = vec![0; sig_len];
    let mut tmp: Vec<u8> = vec![0; tmp_len];
    let mut tmp_ver: Vec<u8> = vec![0; tmpvv_len];
    let mut tmp_sig: Vec<u8> = vec![0; tmp_sig_len];
    falcon_keygen_make(&mut rng, logn as u32, sk.as_mut_slice(), sk_len,
                       pk.as_mut_slice(), pk_len, tmp.as_mut_slice(), tmp_len);
    let (_, sig_len) = falcon_sign_dyn(&mut rng, signature.as_mut_slice(), sig_len,
                                       FALCON_SIG_CT, sk.as_mut_slice(),
                                       sk_len, "data".as_bytes(), tmp_sig.as_mut_slice(), tmp_sig_len);
    println!("sig len= {}", sig_len);
    c.bench_function("Verify CT logn = 9", |b| b.iter(||
        falcon_verify(signature.as_mut_slice(), sig_len, FALCON_SIG_CT,
                                pk.as_mut_slice(), pk_len, "data".as_bytes(), tmp_ver.as_mut_slice(), tmpvv_len)
    ));
}


pub fn vrfy_10(c: &mut Criterion) {
    let mut rng = InnerShake256Context { st: [0; 25], dptr: 0 };
    let logn: usize = 10;
    let pk_len = falcon_publickey_size!(logn);
    let sk_len = falcon_privatekey_size!(logn);
    let tmp_len = falcon_tmpsize_keygen!(logn);
    let sig_len = falcon_sig_compressed_maxsize!(logn);
    let tmp_sig_len = falcon_tmpsize_signdyn!(logn);
    let tmpvv_len = falcon_tmpsize_verify!(logn);
    let mut pk: Vec<u8> = vec![0; pk_len];
    let mut sk: Vec<u8> = vec![0; sk_len];
    let mut signature: Vec<u8> = vec![0; sig_len];
    let mut tmp: Vec<u8> = vec![0; tmp_len];
    let mut tmp_ver: Vec<u8> = vec![0; tmpvv_len];
    let mut tmp_sig: Vec<u8> = vec![0; tmp_sig_len];
    falcon_keygen_make(&mut rng, logn as u32, sk.as_mut_slice(), sk_len,
                       pk.as_mut_slice(), pk_len, tmp.as_mut_slice(), tmp_len);
    let (_, sig_len) = falcon_sign_dyn(&mut rng, signature.as_mut_slice(), sig_len,
                                       FALCON_SIG_COMPRESS, sk.as_mut_slice(),
                                       sk_len, "data".as_bytes(), tmp_sig.as_mut_slice(), tmp_sig_len);
    c.bench_function("Verify logn = 10", |b| b.iter(||
        falcon_verify(signature.as_mut_slice(), sig_len, FALCON_SIG_COMPRESS,
                      pk.as_mut_slice(), pk_len, "data".as_bytes(), tmp_ver.as_mut_slice(), tmpvv_len)
    ));
}

pub fn vrfy_10_ct(c: &mut Criterion) {
    let mut rng = InnerShake256Context { st: [0; 25], dptr: 0 };
    let logn: usize = 10;
    let pk_len = falcon_publickey_size!(logn);
    let sk_len = falcon_privatekey_size!(logn);
    let tmp_len = falcon_tmpsize_keygen!(logn);
    let sig_len = falcon_sig_ct_size!(logn);
    let tmp_sig_len = falcon_tmpsize_signdyn!(logn);
    let tmpvv_len = falcon_tmpsize_verify!(logn);
    let mut pk: Vec<u8> = vec![0; pk_len];
    let mut sk: Vec<u8> = vec![0; sk_len];
    let mut signature: Vec<u8> = vec![0; sig_len];
    let mut tmp: Vec<u8> = vec![0; tmp_len];
    let mut tmp_ver: Vec<u8> = vec![0; tmpvv_len];
    let mut tmp_sig: Vec<u8> = vec![0; tmp_sig_len];
    falcon_keygen_make(&mut rng, logn as u32, sk.as_mut_slice(), sk_len,
                       pk.as_mut_slice(), pk_len, tmp.as_mut_slice(), tmp_len);
    let (_, sig_len) = falcon_sign_dyn(&mut rng, signature.as_mut_slice(), sig_len,
                                       FALCON_SIG_CT, sk.as_mut_slice(),
                                       sk_len, "data".as_bytes(), tmp_sig.as_mut_slice(), tmp_sig_len);
    c.bench_function("Verify CT logn = 10", |b| b.iter(||
        falcon_verify(signature.as_mut_slice(), sig_len, FALCON_SIG_CT,
                                pk.as_mut_slice(), pk_len, "data".as_bytes(), tmp_ver.as_mut_slice(), tmpvv_len)
    ));

}

criterion_group!(benches, vrfy_9, vrfy_9_ct,  vrfy_10, vrfy_10_ct);
criterion_main!(benches);