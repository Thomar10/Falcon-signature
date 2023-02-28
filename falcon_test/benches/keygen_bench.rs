#![allow(dead_code)]

use criterion::{Criterion, criterion_group, criterion_main};

use falcon::{falcon_privatekey_size, falcon_publickey_size, falcon_tmpsize_keygen};
use falcon::falcon::falcon_keygen_make;
use falcon::shake::InnerShake256Context;

pub fn keygen_9(c: &mut Criterion) {
    let mut rng = InnerShake256Context { st: [0; 25], dptr: 0 };
    let logn: usize = 9;
    let pk_len = falcon_publickey_size!(logn);
    let sk_len = falcon_privatekey_size!(logn);
    let tmp_len = falcon_tmpsize_keygen!(logn);
    let mut pk: Vec<u8> = vec![0; pk_len];
    let mut sk: Vec<u8> = vec![0; sk_len];
    let mut tmp: Vec<u8> = vec![0; tmp_len];
    c.bench_function("keygen logn = 9", |b| b.iter(||
        falcon_keygen_make(&mut rng, logn as u32, sk.as_mut_slice(), sk_len,
                           pk.as_mut_slice(), pk_len, tmp.as_mut_slice(), tmp_len)));
}


pub fn keygen_10(c: &mut Criterion) {
    let mut rng = InnerShake256Context { st: [0; 25], dptr: 0 };
    let logn: usize = 10;
    let pk_len = falcon_publickey_size!(logn);
    let sk_len = falcon_privatekey_size!(logn);
    let tmp_len = falcon_tmpsize_keygen!(logn);
    let mut pk: Vec<u8> = vec![0; pk_len];
    let mut sk: Vec<u8> = vec![0; sk_len];
    let mut tmp: Vec<u8> = vec![0; tmp_len];
    c.bench_function("keygen logn = 10", |b| b.iter(||
        falcon_keygen_make(&mut rng, logn as u32, sk.as_mut_slice(), sk_len,
                           pk.as_mut_slice(), pk_len, tmp.as_mut_slice(), tmp_len)));
}


criterion_group!(benches, keygen_9, keygen_10,);
criterion_main!(benches);