use std::cell::RefCell;
use aes::Aes256;
use aes::cipher::{BlockEncrypt, KeyInit};
use aes::cipher::generic_array::GenericArray;

thread_local!(static CONTEXT: RefCell<Aes256Drbg> = RefCell::new(Aes256Drbg {
    key: [0; 32],
    v: [0; 16],
    reseed_counter: 0,
}););


#[allow(dead_code)]

// Does not have personalized string and security as they dont use them...
pub fn randombytes_init(entropy: &mut [u8]) {
    CONTEXT.with(|aes| {
        aes.borrow_mut().key = [0; 32];
        aes.borrow_mut().v = [0; 16];
    });
    aes_ctr_update(entropy);
    CONTEXT.with(|aes| aes.borrow_mut().reseed_counter = 1);
}


pub fn randombytes(x: &mut [u8]) -> bool {
    let mut i: usize = 0;
    let mut xlen = x.len();
    let mut block: [u8; 16] = [0; 16];
    while xlen > 0 {
        CONTEXT.with(|aes| {
            for j in (0..=15).rev() {
                if aes.borrow_mut().v[j] == 0xff {
                    aes.borrow_mut().v[j] = 0x00;
                } else {
                    aes.borrow_mut().v[j] += 1;
                    break;
                }
            }
        });
        aes_ecb(&mut block);
        if xlen > 15 {
            x[i..i + 16].copy_from_slice(&mut block);
            i += 16;
            xlen -= 16;
        } else {
            x[i..i + xlen].copy_from_slice(&mut block[0..xlen]);
            xlen = 0;
        }
    }
    aes_ctr_update(&mut []);
    CONTEXT.with(|aes| aes.borrow_mut().reseed_counter += 1);
    true
}

fn aes_ecb(buffer: &mut [u8]) {
    CONTEXT.with(|aes| {
        let mut key = aes.borrow_mut().key;
        let mut res = aes.borrow_mut().v;
        let aes_key = GenericArray::from_slice(&mut key);
        let cipher = Aes256::new(&aes_key);
        let mut block = GenericArray::from_mut_slice(&mut res);
        cipher.encrypt_block(&mut block);
        buffer[0..16].copy_from_slice(block.as_mut_slice());
    });
}

fn aes_ctr_update(provided_data: &mut [u8]) {
    let mut tmp: [u8; 48] = [0; 48];

    for i in 0..3 {
        CONTEXT.with(|aes| {
            for j in (0..=15).rev() {
                if aes.borrow_mut().v[j] == 0xff {
                    aes.borrow_mut().v[j] = 0x00;
                } else {
                    aes.borrow_mut().v[j] += 1;
                    break;
                }
            }
        });
        let (_, upper) = tmp.split_at_mut(16 * i);
        aes_ecb(upper);
    }
    if provided_data.len() > 0 {
        for i in 0..48 {
            tmp[i] ^= provided_data[i];
        }
    }
    CONTEXT.with(|aes| {
        aes.borrow_mut().key.copy_from_slice(&mut tmp[0..32]);
        aes.borrow_mut().v.copy_from_slice(&mut tmp[32..]);
    });
}


pub(crate) struct Aes256Drbg {
    pub(crate) key: [u8; 32],
    pub(crate) v: [u8; 16],
    pub(crate) reseed_counter: i32,
}
