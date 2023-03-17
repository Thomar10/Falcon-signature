#![no_std]
#![no_main]

extern crate alloc;

use bytemuck;
use embedded_alloc::Heap;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::block;
use stm32f3xx_hal::gpio::{Gpioa, Output, Pin, PushPull, U};
use stm32f3xx_hal::prelude::*;
use chipwhisperer::setup;
use falcon::common::hash_to_point_vartime;
use falcon::falcon::fpr;
use falcon::{falcon_tmpsize_expandprivate, falcon_tmpsize_keygen};

use falcon::keygen::keygen;
use falcon::shake::{i_shake256_init, i_shake256_inject, InnerShake256Context};
use falcon::sign::{expand_privkey, sign_tree};

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {

    //Initialize allocator
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }


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

        rust_genkey(&mut trigger);

        /*let mut result: u32 = 0;
        cortex_m::interrupt::free(|_| {
            trigger.set_high().unwrap();
            result = rust_test(3, 42);
            //result = rust_genkey(&mut trigger);
            trigger.set_low().unwrap();
        });*/

        //result = rust_genkey(&mut trigger);
        //result = 120;
        let mut write_buffer: [u8; 4] = [0; 4]; //u32::to_le_bytes(result);
        let wlen: usize = 4;
        //block!(tx.write(32u8));
        for i in 0..wlen {
            let write_res = block!(tx.write(write_buffer[i]));
        }
    }
}

fn rust_test(left: u32, right: u32) -> u32 {
    let mut result: u32 = 0;
    //let mut h: [u16; 1024] = [0; 1024];

    for _ in 0..10 {
        result += left + right;
        //h[0] = result as u16;
    }

    return result;
}

fn rust_genkey(trigger: &mut  Pin<Gpioa, U<12>, Output<PushPull>>){

    let input: [u8; 6] = [1, 2, 3, 4, 5, 6];

    const LOGN: usize = 5;
    const BUFFER_SIZE: usize = 272 + ((3 << LOGN) + 7);
    let mut h: [u16; 1024] = [0; 1024];
    let mut f: [i8; 1024] = [0; 1024];
    let mut g: [i8; 1024] = [0; 1024];
    let mut F: [i8; 1024] = [0; 1024];
    let mut G: [i8; 1024] = [0; 1024];

    let mut tmp_keygen: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut rng_rust: InnerShake256Context = gen_rng(&input);
    cortex_m::interrupt::free(|_| {
        trigger.set_high().unwrap();
        keygen(&mut rng_rust, &mut f, &mut g, &mut F, &mut G, &mut h, LOGN as u32, &mut tmp_keygen);
        trigger.set_low().unwrap();
    });
    return;
}

/*fn rust_sign(trigger: &mut  Pin<Gpioa, U<12>, Output<PushPull>>) -> u32 {
    let input: [u8; 6] = [1, 2, 3, 4, 5, 6];

    const LOGN: usize = 10;
    const N: usize = 1 << LOGN;
    const BUFFER_SIZE: usize = falcon_tmpsize_keygen!(LOGN);
    let mut h: [u16; 1024] = [0; 1024];
    let mut f: [i8; 1024] = [0; 1024];
    let mut g: [i8; 1024] = [0; 1024];
    let mut F: [i8; 1024] = [0; 1024];
    let mut G: [i8; 1024] = [0; 1024];

    let mut tmp_keygen: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut rng_rust: InnerShake256Context = gen_rng(&input);
    keygen(&mut rng_rust, &mut f, &mut g, &mut F, &mut G, &mut h, LOGN as u32, &mut tmp_keygen);
    const EXKLENGTH: usize = (LOGN + 40) << LOGN;
    let mut expanded_key: [fpr; EXKLENGTH] = [0; EXKLENGTH];
    let mut tmp_key: [fpr; falcon_tmpsize_expandprivate!(LOGN)] = [0; falcon_tmpsize_expandprivate!(LOGN)];
    expand_privkey(&mut expanded_key, &mut f, &mut g, &mut F, &mut G, LOGN as u32, &mut tmp_key);
    let mut sig: [i16; 1024] = [0; 1024];

    let mut sc = gen_rng(&input);
    let mut hm: [u16; 1024] = [0; 1024];
    hash_to_point_vartime(&mut sc, &mut hm, LOGN as u32);

    trigger.set_high().unwrap();
    sign_tree(&mut sig, &mut rng_rust, &mut expanded_key, &hm, LOGN as u32, &mut tmp_keygen);
    trigger.set_low().unwrap();

    return sig[0] as u32;
}*/

fn gen_rng(input: &[u8]) -> InnerShake256Context {
    let state: [u64; 25] = [0; 25];
    let dptr: u64 = 0;
    let mut sc_rust = InnerShake256Context { st: state, dptr};
    i_shake256_init(&mut sc_rust);
    i_shake256_inject(&mut sc_rust, &input);
    return sc_rust;
}