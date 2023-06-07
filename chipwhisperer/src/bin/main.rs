#![no_std]
#![no_main]

extern crate alloc;

use cortex_m_rt::entry;
use embedded_alloc::Heap;
use panic_halt as _;
use stm32f4xx_hal::block;
use stm32f4xx_hal::gpio::{self, Output, PushPull};
use stm32f4xx_hal::pac::Peripherals;
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::rng::Rng;
use stm32f4xx_hal::serial::Config;
use stm32f4xx_hal::time::U32Ext;


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
    let rcc = dp.RCC.constrain();

    let gpioa = dp.GPIOA.split();

    let mut trigger: TriggerPin = gpioa.pa12.into_push_pull_output();
    trigger.set_low();

    let clocks = rcc
        .cfgr
        .use_hse(7384609.Hz())
        .sysclk(7384609.Hz())
        .require_pll48clk()
        .freeze();

    let rand_source: Rng = dp.RNG.constrain(&clocks);
    let mut rng: RngBoth = RngBoth { hal_rng: Some(rand_source), rust_rng: None };

    let tx_pin = gpioa.pa9.into_alternate();
    let rx_pin = gpioa.pa10.into_alternate();

    let serial = dp.USART1.serial(
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

        let result_buffer: [u8; 8];

        match cmd {
            1 => result_buffer = test_add(&mut trigger, &read_buffer),
            2 => result_buffer = test_add_masked(&mut trigger, &read_buffer),
            3 => result_buffer = test_add_masked_deep(&mut trigger, &read_buffer, &mut rng),
            4 => result_buffer = test_mul(&mut trigger, &read_buffer),
            5 => result_buffer = test_mul_masked(&mut trigger, &read_buffer),
            6 => result_buffer = test_mul_masked_deep(&mut trigger, &read_buffer, &mut rng),
            7 => result_buffer = test_ursh(&mut trigger, &read_buffer),
            8 => result_buffer = test_secure_ursh(&mut trigger, &read_buffer, &mut rng),
            9 => result_buffer = test_norm(&mut trigger, &read_buffer),
            10 => result_buffer = test_secure_norm(&mut trigger, &read_buffer, &mut rng),
            11 => result_buffer = test_fft(&mut trigger, &read_buffer, &mut rng),
            12 => result_buffer = test_fft_masked(&mut trigger, &read_buffer, &mut rng),
            13 => result_buffer = test_secure_fft(&mut trigger, &read_buffer, &mut rng),
            14 => result_buffer = test_poly_mul_fft(&mut trigger, &read_buffer, &mut rng),
            15 => result_buffer = test_poly_mul_fft_masked(&mut trigger, &read_buffer, &mut rng),
            16 => result_buffer = test_sign(&mut trigger, &read_buffer),
            17 => result_buffer = test_masked_sign(&mut trigger, &read_buffer, &mut rng),
            _ => result_buffer = [0; 8],
        }

        //Return result buffer
        for i in 0..result_buffer.len() {
            let _err = block!(tx.write(result_buffer[i]));
        }
    }
}


