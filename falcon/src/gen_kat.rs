use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::path::Path;
use std::ptr::{null, null_mut};
use crate::falcon_c::nist_c::randombytes_init_func;
use crate::katrng::randombytes_init;
use crate::keygen;
use crate::nist::crypto_sign_keypair;

pub unsafe fn genkat512() {
    let check_file_path = Path::new("./src/test/falcon512-KAT.rsp");
    if !check_file_path.exists() {
        panic!("No file to check up with!");
    }
    let check_file = File::open(check_file_path).expect("Could not open file!");
    let buf_reader = BufReader::new(check_file);

    let mut pk: [u16; 897] = [0; 897];
    let mut sk: [u16; 1281] = [0; 1281];
    let mut seedu8: Vec<u8>;
    let mut seedu16: [u16; 48] = [0; 48];
    for line in buf_reader.lines() {
        let mut string = line.unwrap();
        if string.contains("seed") {
            seedu8 = hex::decode(string.split_at(7).1).unwrap();
            //byte_to_short(seedu8, &mut seedu16);
            //println!("seed {:?}", seedu16);
            randombytes_init_func(seedu8.as_ptr(), null(), 256);
            let res = crypto_sign_keypair(&mut pk, &mut sk);
            if res {
                println!("Got pk {:?}", pk);
                println!("Got sk {:?}", sk);
            }
        }
    }
}

fn byte_to_short(input: Vec<u8>, res: &mut [u16]) {
    for (i, byte) in input.iter().enumerate() {
        res[i] = *byte as u16;
    }
}
