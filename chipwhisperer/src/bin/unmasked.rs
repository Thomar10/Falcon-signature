#![no_std]
#![no_main]

extern crate alloc;

use bytemuck;
use cortex_m::asm::delay;
use cortex_m_rt::entry;
use embedded_alloc::Heap;
use panic_halt as _;
use rand_core::RngCore;
use stm32f4xx_hal as hal;
use stm32f4xx_hal::block;
use stm32f4xx_hal::gpio::{self, Alternate, Output, Pin, PushPull};
use stm32f4xx_hal::gpio::Dynamic::OutputPushPull;
use stm32f4xx_hal::pac::{Peripherals, RNG, USART1};
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::rng::Rng;
use stm32f4xx_hal::serial::{Config, Rx, Serial, Tx};
use stm32f4xx_hal::time::U32Ext;

use falcon::{falcon_sig_compressed_maxsize, falcon_tmpsize_expanded_key_size, falcon_tmpsize_expandprivate, falcon_tmpsize_keygen, falcon_tmpsize_signtree};
use falcon::common::hash_to_point_vartime;
use falcon::falcon::{falcon_sign_tree, fpr};
use falcon::fft::{fft, fpc_mul};
use falcon::fpr::{fpr_add, fpr_mul, fpr_norm64, fpr_of, fpr_sub, fpr_ursh};
use falcon::keygen::keygen;
use falcon::shake::{i_shake256_extract, i_shake256_init, i_shake256_inject, InnerShake256Context};
use falcon::sign::{expand_privkey, sign_tree};
use falcon_masked::fft_masked::{fft as fft_masked, fpc_mul as fpc_mul_masked};
use falcon_masked::fft_masked_deep::{secure_fft, secure_fpc_mul};
use falcon_masked::fpr_masked::{fpr_add as fpr_add_masked, fpr_mul as fpr_mul_masked};
use falcon_masked::fpr_masked_deep::{secure_add, secure_fpr_add, secure_fpr_norm, secure_mul, secure_ursh};
use falcon_masked::sign_masked::sign_tree_with_temp as sign_tree_masked;
use randomness::random::RngBoth;
use chipwhisperer::fft::{test_fft, test_fft_masked, test_poly_mul_fft, test_poly_mul_fft_masked, test_secure_fft};
use chipwhisperer::fpr_add::{test_add, test_add_masked, test_add_masked_deep};
use chipwhisperer::fpr_mul::{test_mul, test_mul_masked, test_mul_masked_deep};
use chipwhisperer::norm::{test_norm, test_secure_norm};
use chipwhisperer::sign::{test_masked_sign, test_sign};
use chipwhisperer::ursh::{test_secure_ursh, test_ursh};

#[global_allocator]
static HEAP: Heap = Heap::empty();

type TriggerPin = gpio::PA12<Output<PushPull>>;

#[entry]
fn main() -> ! {

    //Initialize allocator
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 8192;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    let dp = Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();

    let mut gpioa = dp.GPIOA.split();

    let mut trigger: TriggerPin = gpioa.pa12.into_push_pull_output();
    trigger.set_low();

    //let clocks = rcc.cfgr.require_pll48clk().freeze();
    let clocks = rcc
        .cfgr
        .use_hse(7384609.Hz())
        .sysclk(7384609.Hz())
        .require_pll48clk()
        .freeze();

    let mut rand_source: Rng = dp.RNG.constrain(&clocks);
    let mut rng: RngBoth = RngBoth { hal_rng: Some(rand_source), rust_rng: None };

    let tx_pin = gpioa.pa9.into_alternate();
    let rx_pin = gpioa.pa10.into_alternate();

    let mut serial = dp.USART1.serial(
        (tx_pin, rx_pin),
        Config::default().baudrate(38_400.bps()),
        &clocks,
    ).unwrap();

    let (mut tx, mut rx) = serial.split();

    loop {
        let cmd: u8 = block!(rx.read()).unwrap() as u8;

        let data_len: usize = block!(rx.read()).unwrap() as usize;

        let mut read_buffer: [u8; 1024] = [0; 1024];

        for i in 0..data_len {
            read_buffer[i] = block!(rx.read()).unwrap();
        }

        let result_buffer = test_sign(cmd, &mut trigger, &read_buffer);

        //let result_buffer = test_masked_sign(cmd, &mut trigger, &read_buffer, &mut rng);


        // let mut result_buffer: [u8; 8] = [0; 8];
        //
        // match cmd {
        //     1 => result_buffer = test_add(&mut trigger, &read_buffer),
        //     2 => result_buffer = test_add_masked(&mut trigger, &read_buffer),
        //     3 => result_buffer = test_add_masked_deep(&mut trigger, &read_buffer, &mut rng),
        //     4 => result_buffer = test_mul(&mut trigger, &read_buffer),
        //     5 => result_buffer = test_mul_masked(&mut trigger, &read_buffer),
        //     6 => result_buffer = test_mul_masked_deep(&mut trigger, &read_buffer, &mut rng),
        //     7 => result_buffer = test_ursh(&mut trigger, &read_buffer),
        //     8 => result_buffer = test_secure_ursh(&mut trigger, &read_buffer, &mut rng),
        //     9 => result_buffer = test_norm(&mut trigger, &read_buffer),
        //     10 => result_buffer = test_secure_norm(&mut trigger, &read_buffer, &mut rng),
        //     11 => result_buffer = test_fft(&mut trigger, &read_buffer, &mut rng),
        //     12 => result_buffer = test_fft_masked(&mut trigger, &read_buffer, &mut rng),
        //     13 => result_buffer = test_secure_fft(&mut trigger, &read_buffer, &mut rng),
        //     14 => result_buffer = test_poly_mul_fft(&mut trigger, &read_buffer, &mut rng),
        //     15 => result_buffer = test_poly_mul_fft_masked(&mut trigger, &read_buffer, &mut rng),
        //     _ => result_buffer = [0; 8],
        // }

        //Return result buffer
        for i in 0..result_buffer.len() {
            block!(tx.write(result_buffer[i]));
        }
    }
}

fn led_blink_test(dp: Peripherals) {
    let mut cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output();

    // Set up system clock
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

    let mut delay = cp.SYST.delay(&clocks);

    loop {
        led.toggle();
        delay.delay_ms(1000_u32)
    }
}

fn rust_test(trigger: &mut TriggerPin, left: u32, right: u32, iter: usize) -> u32 {
    let mut result: u32 = 0;
    //let mut h: [u16; 1024] = [0; 1024];

    cortex_m::interrupt::free(|_| {
        trigger.set_high();
        for _ in 0..iter {
            result += left + right;
            //h[0] = result as u16;
        }
        trigger.set_low();
    });

    return result;
}

fn test_rand(trigger: &mut TriggerPin, read_buffer: &[u8], rng: &mut RngBoth) -> [u8; 8] {
    let mut return_buffer: [u8; 8] = [0; 8];

    let rn: u64 = rng.next_u64();

    return_buffer.copy_from_slice(&u64::to_le_bytes(rn));
    return return_buffer;
}


