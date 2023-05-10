#![no_std]
#![no_main]
/*
use stm32f4xx_hal as hal;
use stm32f4xx_hal::gpio::{Alternate, Pin};
use stm32f4xx_hal::serial::{Rx, Serial, Tx};

use crate::hal::{
    gpio::{self, Output, PushPull},
    pac::{USART1, Peripherals},
    prelude::*,
};

use usb_device::prelude::*;

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

//use stm32f4xx_hal::{self as hal, pac, prelude::*};
//use stm32f4xx_hal::gpio::{Alternate, Gpioa, Output, Pin, PushPull, U};
//use stm32f4xx_hal::pac::USART1;
//use stm32f4xx_hal::serial::Serial;

pub fn setup() -> (Pin<A, 12, Output>, (Tx<USART1, WORD>, Rx<USART1, WORD>)) {
    let dp = Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let mut trigger = gpioa.pa12.into_push_pull_output();
    trigger.set_low().unwrap();

    let clocks = rcc.cfgr.freeze();

    let tx_pin = gpioa.pa9.into_alternate();
    let rx_pin = gpioa.pa10.into_alternate();

    let mut tx = dp.USART1.tx(tx_pin, 38_400.Bd(), &clocks).unwrap();
    let mut rx = dp.USART1.rx(rx_pin, 38_400.Bd(), &clocks).unwrap();

    /*let mut serial = Serial::new(
        dp.USART1,
        (tx_pin, rx_pin),
        38_800.Bd(),
        &clocks
    ).unwrap();*/


    /*let tx = gpioa.pa9.into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    let rx = gpioa.pa10.into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);

    let usb_serial = hal::serial::Serial::new(
        dp.USART1,
        (tx, rx),
        38_400.Bd(),
        &clocks,
    );*/

    return (trigger, (tx, rx));
}*/
