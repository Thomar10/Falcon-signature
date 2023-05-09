#![allow(dead_code)]

use criterion::{Criterion, criterion_group, criterion_main};

use falcon::{falcon_privatekey_size, falcon_publickey_size, falcon_tmpsize_keygen};
use falcon::falcon::falcon_keygen_make;
use falcon::keygen::keygen;
use falcon::shake::InnerShake256Context;
use ntru_gen::falcon_ntru::falcon_keygen;
use ntru_gen::prng::{NtruPrngChacha8Context, prng_chacha8_out};

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

pub fn keygen_ntrugen_9(c: &mut Criterion) {
    const LOGN: usize = 9;
    const SIZE: usize = 1 << LOGN;
    let mut ctx = NtruPrngChacha8Context {
        d: [0; 40],
    };
    let mut tmp: [u32; 24 * SIZE] = [0; 24 * SIZE];
    let mut f: [i8; SIZE] = [0; SIZE];
    let mut g: [i8; SIZE] = [0; SIZE];
    let mut ff: [i8; SIZE] = [0; SIZE];
    let mut gg: [i8; SIZE] = [0; SIZE];
    c.bench_function("ntru keygen logn = 9", |b| b.iter(||
        falcon_keygen(LOGN, &mut f, &mut g, &mut ff, &mut gg, prng_chacha8_out, &mut ctx, &mut tmp)));
}


pub fn keygen_ntrugen_10(c: &mut Criterion) {
    const LOGN: usize = 10;
    const SIZE: usize = 1 << LOGN;
    let mut ctx = NtruPrngChacha8Context {
        d: [0; 40],
    };
    let mut tmp: [u32; 24 * SIZE] = [0; 24 * SIZE];
    let mut f: [i8; SIZE] = [0; SIZE];
    let mut g: [i8; SIZE] = [0; SIZE];
    let mut ff: [i8; SIZE] = [0; SIZE];
    let mut gg: [i8; SIZE] = [0; SIZE];
    c.bench_function("ntru keygen logn = 10", |b| b.iter(||
        falcon_keygen(LOGN, &mut f, &mut g, &mut ff, &mut gg, prng_chacha8_out, &mut ctx, &mut tmp)));
}


criterion_group!(benches, keygen_9, keygen_10, keygen_ntrugen_9, keygen_ntrugen_10);
criterion_main!(benches);