use randomness::random::RngBoth;
use stm32f4xx_hal::gpio;
use stm32f4xx_hal::gpio::{Output, PushPull};
use falcon::falcon::fpr;
use falcon::fpr::fpr_norm64;
use falcon_masked::fpr_masked_deep::secure_fpr_norm;

type TriggerPin = gpio::PA12<Output<PushPull>>;

pub fn test_norm(trigger: &mut TriggerPin, read_buffer: &[u8]) -> [u8; 8] {
    let a: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[..8]).unwrap());

    let ee: i16 = read_buffer[16] as i16;

    let mut c: fpr = 0;
    let mut e: i32 = 0;

    cortex_m::interrupt::free(|_| {
        trigger.set_high();
        (c, e) = fpr_norm64(a, ee as i32);
        trigger.set_low();
    });

    let mut return_buffer: [u8; 8] = [0; 8];
    return_buffer.copy_from_slice(&u64::to_le_bytes(c));

    return return_buffer;
}

pub fn test_secure_norm(trigger: &mut TriggerPin, read_buffer: &[u8], rng: &mut RngBoth) -> [u8; 8] {
    let a: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[..8]).unwrap());
    let a_rand: fpr = u64::from_le_bytes(<[u8; 8]>::try_from(&read_buffer[8..16]).unwrap());

    let ee: i16 = read_buffer[16] as i16;
    let ee_rand: i16 = read_buffer[17] as i16;

    let a_shares: [fpr; 2] = [a ^ a_rand, a_rand];
    let ee_shares: [i16; 2] = [ee.wrapping_sub(ee_rand), ee];

    let mut c_shares: [fpr; 2] = [0; 2];
    let mut e_shares: [i16; 2] = [0; 2];

    cortex_m::interrupt::free(|_| {
        trigger.set_high();
        (c_shares, e_shares) = secure_fpr_norm(&a_shares, &ee_shares, rng);
        trigger.set_low();
    });

    let c: fpr = c_shares[0] ^ c_shares[1];

    let mut return_buffer: [u8; 8] = [0; 8];
    return_buffer.copy_from_slice(&u64::to_le_bytes(c));

    return return_buffer;
}