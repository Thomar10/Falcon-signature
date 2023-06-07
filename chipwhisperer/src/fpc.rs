#![no_std]
#![no_main]

use randomness::random::RngBoth;
use stm32f4xx_hal::gpio;
use stm32f4xx_hal::gpio::{Output, PushPull};
use falcon::falcon::fpr;
use falcon::fft::fpc_mul;
use falcon::fpr::{fpr_add, fpr_sub};
use falcon_masked::fft_masked_deep::secure_fpc_mul;
use falcon_masked::fft_masked::{fpc_mul as fpc_mul_masked};

type TriggerPin = gpio::PA12<Output<PushPull>>;

pub fn test_fpc_mul(trigger: &mut  TriggerPin, read_buffer: &[u8]) -> [u8; 16] {
    let (mut a, mut b): (fpr, fpr) = (0, 0);

    let a_re: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[..8]).unwrap());
    let a_im: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[8..16]).unwrap());
    let b_re: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[16..24]).unwrap());
    let b_im: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[24..32]).unwrap());

    cortex_m::interrupt::free(|_| {
        trigger.set_high();
        (a, b) = fpc_mul(a_re, a_im, b_re, b_im);
        trigger.set_low();
    });

    let mut return_buffer: [u8; 16] = [0; 16];

    return_buffer[0..8].copy_from_slice(&u64::to_le_bytes(a));
    return_buffer[8..16].copy_from_slice(&u64::to_le_bytes(b));

    return return_buffer
}

pub fn test_fpc_mul_masked(trigger: &mut  TriggerPin, read_buffer: &[u8]) -> [u8; 16] {
    //let mut rng = WyRand::new();

    let (mut re, mut im): ([fpr; 2], [fpr; 2]) = ([0; 2], [0; 2]);

    let a_re_val: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[..8]).unwrap());
    let a_im_val: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[8..16]).unwrap());
    let b_re_val: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[16..24]).unwrap());
    let b_im_val: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[24..32]).unwrap());

    let a_re_rand: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[32..40]).unwrap()); //rng.generate::<u64>();
    let a_im_rand: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[40..48]).unwrap());
    let b_re_rand: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[48..56]).unwrap());
    let b_im_rand: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[56..64]).unwrap());

    let a_re: [fpr; 2] = [fpr_sub(a_re_val, a_re_rand), a_re_rand];
    let a_im: [fpr; 2] = [fpr_sub(a_im_val, a_im_rand), a_im_rand];
    let b_re: [fpr; 2] = [fpr_sub(b_re_val, b_re_rand), b_re_rand];
    let b_im: [fpr; 2] = [fpr_sub(b_im_val, b_im_rand), b_im_rand];

    cortex_m::interrupt::free(|_| {
        trigger.set_high();
        (re, im) = fpc_mul_masked(&a_re, &a_im, &b_re, &b_im);
        trigger.set_low();
    });

    let mut return_buffer: [u8; 16] = [0; 16];

    let a: fpr = fpr_add(re[0], re[1]);
    let b: fpr = fpr_add(im[0], im[1]);

    return_buffer[0..8].copy_from_slice(&u64::to_le_bytes(a));
    return_buffer[8..16].copy_from_slice(&u64::to_le_bytes(b));

    return return_buffer;
}

pub fn test_fpc_mul_masked_deep(trigger: &mut  TriggerPin, read_buffer: &[u8], rng: &mut RngBoth) -> [u8; 16] {
    let (mut re, mut im): ([fpr; 2], [fpr; 2]) = ([0; 2], [0; 2]);

    let a_re_val: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[..8]).unwrap());
    let a_im_val: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[8..16]).unwrap());
    let b_re_val: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[16..24]).unwrap());
    let b_im_val: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[24..32]).unwrap());

    let a_re_rand: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[32..40]).unwrap()); //rng.generate::<u64>();
    let a_im_rand: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[40..48]).unwrap());
    let b_re_rand: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[48..56]).unwrap());
    let b_im_rand: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[56..64]).unwrap());

    let a_re: [fpr; 2] = [a_re_val ^ a_re_rand, a_re_rand];
    let a_im: [fpr; 2] = [a_im_val ^ a_im_rand, a_im_rand];
    let b_re: [fpr; 2] = [b_re_val ^ b_re_rand, b_re_rand];
    let b_im: [fpr; 2] = [b_im_val ^ b_im_rand, b_im_rand];

    cortex_m::interrupt::free(|_| {
        trigger.set_high();
        (re, im) = secure_fpc_mul(&a_re, &a_im, &b_re, &b_im, rng);
        trigger.set_low();
    });

    let mut return_buffer: [u8; 16] = [0; 16];

    let a: fpr = re[0] ^ re[1];
    let b: fpr = im[0] ^ im[1];

    return_buffer[0..8].copy_from_slice(&u64::to_le_bytes(a));
    return_buffer[8..16].copy_from_slice(&u64::to_le_bytes(b));

    return return_buffer;
}