use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::katrng::randombytes_init;
use crate::nist::{crypto_sign_keypair, crypto_sign_open};

pub unsafe fn genkat512() {
    let check_file_path = Path::new("./src/tests/falcon512-KAT.rsp");
    if !check_file_path.exists() {
        panic!("No file to check up with!");
    }
    let check_file = File::open(check_file_path).expect("Could not open file!");
    let buf_reader = BufReader::new(check_file);

    let mut pk: [u8; 897] = [0; 897];
    let mut sk: [u8; 1281] = [0; 1281];
    let mut seed: Vec<u8>;
    let mut pk_hex: String = String::new();
    let mut sk_hex: String = String::new();
    let mut mlen = 0;
    let mut msg: Vec<u8> = vec![0; 0];
    let mut m1: Vec<u8>;
    //let mut smlen = 0;
    let mut sm: Vec<u8>;
    for line in buf_reader.lines() {
        let string = line.unwrap();
        if string.contains("seed") {
            seed = hex::decode(string.split_at(7).1).unwrap();
            randombytes_init(&mut seed);
            let res = crypto_sign_keypair(&mut pk, &mut sk);
            assert_eq!(res, true);
            pk_hex = hex::encode_upper(pk);
            sk_hex = hex::encode_upper(sk);
        }
        if string.contains("mlen") && !string.contains("s") {
            mlen = string.split_at(7).1.parse().unwrap();
        }
        if string.contains("msg") {
            msg = hex::decode(string.split_at(6).1).unwrap();
        }
        if string.contains("pk") {
            let x = string.split_at(5).1;
            assert_eq!(pk_hex, x);
        }
        if string.contains("sk") {
            let x = string.split_at(5).1;
            assert_eq!(sk_hex, x);
        }
        if string.contains("smlen") {
            //smlen = string.split_at(8).1.parse().unwrap();
        }
        if string.contains("sm") && !string.contains("len") {
            sm = hex::decode(string.split_at(5).1).unwrap();
            m1 = msg.clone();
            m1.fill(0);
            let signature_length = sm.len();
            let (res, length) = crypto_sign_open(m1.as_mut_slice(), sm.as_mut_slice(), signature_length, &mut pk);
            assert_eq!(res, true);
            assert_eq!(length, mlen);
            assert_eq!(m1, msg);
        }
    }
}
