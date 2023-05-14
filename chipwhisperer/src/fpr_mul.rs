#![no_std]
#![no_main]

use randomness::random::RngBoth;
use stm32f4xx_hal::gpio;
use stm32f4xx_hal::gpio::{Output, PushPull};
use falcon::falcon::fpr;
use falcon::fpr::{fpr_add, fpr_mul, fpr_sub};
use falcon_masked::fpr_masked::{fpr_mul as fpr_mul_masked};
use falcon_masked::fpr_masked_deep::secure_mul;

type TriggerPin = gpio::PA12<Output<PushPull>>;

pub fn test_mul(trigger: &mut TriggerPin, read_buffer: &[u8]) -> [u8; 8] {
    let a: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[..8]).unwrap());
    let b: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[8..16]).unwrap());

    let mut c: fpr = 0;

    cortex_m::interrupt::free(|_| {
        trigger.set_high();
        c = fpr_mul(a, b);
        trigger.set_low();
    });

    let mut return_buffer: [u8; 8] = [0; 8];
    return_buffer.copy_from_slice(&u64::to_le_bytes(c));

    return return_buffer;
}

pub fn test_mul_masked(trigger: &mut TriggerPin, read_buffer: &[u8]) -> [u8; 8] {
    let a_val: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[..8]).unwrap());
    let b_val: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[8..16]).unwrap());

    let a_share: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[16..24]).unwrap());
    let b_share: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[24..32]).unwrap());

    let a: [fpr; 2] = [fpr_sub(a_val, a_share), a_share];
    let b: [fpr; 2] = [fpr_sub(b_val, b_share), b_share];

    let mut c: [fpr; 2] = [0; 2];

    cortex_m::interrupt::free(|_| {
        trigger.set_high();
        c = fpr_mul_masked(&a, &b);
        trigger.set_low();
    });

    let c_val: fpr = fpr_add(c[0], c[1]);

    let mut return_buffer: [u8; 8] = [0; 8];
    return_buffer.copy_from_slice(&u64::to_le_bytes(c_val));

    return return_buffer;
}

pub fn test_mul_masked_deep(trigger: &mut TriggerPin, read_buffer: &[u8], rng: &mut RngBoth) -> [u8; 8] {
    let a_val: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[..8]).unwrap());
    let b_val: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[8..16]).unwrap());

    let a_share: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[16..24]).unwrap());
    let b_share: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[24..32]).unwrap());

    let a: [fpr; 2] = [a_val ^ a_share, a_share];
    let b: [fpr; 2] = [b_val ^ b_share, b_share];

    let mut c: [fpr; 2] = [0; 2];

    cortex_m::interrupt::free(|_| {
        trigger.set_high();
        c = secure_mul(&a, &b, rng);
        trigger.set_low();
    });

    let c_val: fpr = c[0] ^ c[1];

    let mut return_buffer: [u8; 8] = [0; 8];
    return_buffer.copy_from_slice(&u64::to_le_bytes(c_val));

    return return_buffer;
}