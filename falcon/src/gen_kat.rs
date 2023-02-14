use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::katrng::randombytes_init;
use crate::nist::crypto_sign_keypair;

pub unsafe fn genkat512() {
    let check_file_path = Path::new("./src/test/falcon512-KAT.rsp");
    if !check_file_path.exists() {
        panic!("No file to check up with!");
    }
    let check_file = File::open(check_file_path).expect("Could not open file!");
    let buf_reader = BufReader::new(check_file);

    let mut pk: [u8; 897] = [0; 897];
    let mut sk: [u8; 1281] = [0; 1281];
    let mut seedu8: Vec<u8>;
    let mut pk_hex: String = String::new();
    let mut sk_hex: String = String::new();
    for line in buf_reader.lines() {
        let string = line.unwrap();
        if string.contains("seed") {
            seedu8 = hex::decode(string.split_at(7).1).unwrap();
            randombytes_init(&mut seedu8);
            let res = crypto_sign_keypair(&mut pk, &mut sk);
            assert_eq!(res, true);
            pk_hex = hex::encode_upper(pk);
            sk_hex = hex::encode_upper(sk);
        }
        if string.contains("pk") {
            let x = string.split_at(5).1;
            assert_eq!(pk_hex, x);
        }
        if string.contains("sk") {
            let x = string.split_at(5).1;
            assert_eq!(sk_hex, x);
        }
    }
}
