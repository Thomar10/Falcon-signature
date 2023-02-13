use aes::Aes256;
use aes::cipher::{Block, BlockEncrypt, KeyInit};
use aes::cipher::generic_array::GenericArray;

const context: Aes256Drbg = Aes256Drbg {
    key: [0; 32],
    v: [0; 16],
    reseed_counter: 0,
};

pub fn randombytes_init(entropy: &mut [u16], ps: &mut [u16], security: i32) {
    /*let mut seed: [u16; 48] = [0; 48];
    seed.copy_from_slice(entropy);

    if ps.len() > 0 {
        for i in 0..48 {
            seed[i] ^= ps[i];
        }
    }
    context.key = [0; 25];
    context.v = [0; 16];

    context.reseed_counter = 1; */
}


pub fn randombytes(x: &mut [u16]) -> bool {
    let mut i: i32 = 0;
    let mut xlen = x.len();

    while xlen > 0 {
        for j in (0..=15).rev() {
            if context.v[j] == 0xff {
                context.v[j] = 0x00;
            } else {
                context.v[j] += 1;
                break;
            }
        }
        aes_ecb(&mut context.key, &mut context.v);
        if xlen > 15 {
            //memcpy(x+i, block, 16);
            i += 16;
            xlen -= 16;
        } else {
            //memcpy(x+i, block, xlen);
            xlen = 0;
        }
    }
    context.reseed_counter += 1;
    true
}

fn aes_ecb(mut key: &mut [u8; 32], mut ctr: &mut [u8; 16]) {
  /*  let aes_key = GenericArray::from_slice(key);
    let cipher = Aes256::new(&aes_key);
    let mut block = GenericArray::from_mut_slice(&mut [0u8; 16]);
    cipher.encrypt_block(&mut block);
    ctr.copy_from_slice(block.as_slice()); */
}


pub(crate) struct Aes256Drbg {
    pub(crate) key: [u8; 32],
    pub(crate) v: [u8; 16],
    pub(crate) reseed_counter: i32,
}