#![no_std]
#![no_main]

use embedded_alloc::Heap;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::block;
use stm32f3xx_hal::prelude::*;
use chipwhisperer::setup;

use falcon::keygen::keygen;
use falcon::shake::{i_shake256_init, i_shake256_inject, InnerShake256Context};

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    let (mut trigger, usb_serial) = setup();
    let (mut tx, mut rx) = usb_serial.split();

    loop {
        let mut read_buffer: [u8; 10] = [0; 10];
        let cmd = block!(rx.read()).unwrap();
        let rlen: usize = block!(rx.read()).unwrap() as usize;

        for i in 0..rlen {
            read_buffer[i] = block!(rx.read()).unwrap();
        }

        let left: u32 = u32::from_le_bytes([read_buffer[0], read_buffer[1], read_buffer[2], read_buffer[3]]);
        let right: u32 = u32::from_le_bytes([read_buffer[4], read_buffer[5], read_buffer[6], read_buffer[7]]);

        trigger.set_high().unwrap();
        let result = rust_test(left, right);
        trigger.set_low().unwrap();

        let mut write_buffer: [u8; 4] = u32::to_le_bytes(result);
        let wlen: usize = 4;
        for i in 0..wlen {
            block!(tx.write(write_buffer[i]));
        }
    }
}

fn rust_test(left: u32, right: u32) -> u32 {
    let mut result: u32 = 0;
    //let mut h: [u16; 1024] = [0; 1024];

    for _ in 0..50 {
        result += left + right;
        //h[0] = result as u16;
    }

    return result;
}

fn rust_genkey() -> u32{

    let input: [u8; 6] = [1, 2, 3, 4, 5, 6];

    const LOGN: usize = 10;
    const BUFFER_SIZE: usize = 8192 * 8;
    let mut h: [u16; 1024] = [0; 1024];
    let mut f: [i8; 1024] = [0; 1024];
    let mut g: [i8; 1024] = [0; 1024];
    let mut F: [i8; 1024] = [0; 1024];
    let mut G: [i8; 1024] = [0; 1024];

    let mut tmp_keygen: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut rng_rust: InnerShake256Context = gen_rng(&input);
    keygen(&mut rng_rust, &mut f, &mut g, &mut F, &mut G, &mut h, LOGN as u32, &mut tmp_keygen);
    return h[0] as u32;
}

fn gen_rng(input: &[u8]) -> InnerShake256Context {
    let state: [u64; 25] = [0; 25];
    let dptr: u64 = 0;
    let mut sc_rust = InnerShake256Context { st: state, dptr};
    i_shake256_init(&mut sc_rust);
    i_shake256_inject(&mut sc_rust, &input);
    return sc_rust;
}