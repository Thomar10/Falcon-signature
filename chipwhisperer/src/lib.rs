#![no_std]
#![no_main]

use stm32f3xx_hal::{self as hal, pac, prelude::*};
use stm32f3xx_hal::gpio::{Alternate, Gpioa, Output, Pin, PushPull, U};
use stm32f3xx_hal::pac::USART1;
use stm32f3xx_hal::serial::Serial;

pub fn setup() -> (Pin<Gpioa, U<12>, Output<PushPull>>, Serial<USART1, (Pin<Gpioa, U<9>, Alternate<PushPull, 7>>, Pin<Gpioa, U<10>, Alternate<PushPull, 7>>)>) {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let mut trigger = gpioa.pa12.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    trigger.set_low().unwrap();

    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let tx = gpioa.pa9.into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);
    let rx = gpioa.pa10.into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh);

    let usb_serial = hal::serial::Serial::new(
        dp.USART1,
        (tx, rx),
        38_400.Bd(),
        clocks,
        &mut rcc.apb2,
    );

    return (trigger, usb_serial);
}
