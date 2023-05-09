#![allow(unreachable_code)]
use rand_core::{Error, RngCore};
use stm32f4xx_hal::rng::Rng as HalRng;

#[cfg(feature = "withstd")]
use rand::rngs::ThreadRng;

pub struct RngBoth {
    pub hal_rng: Option<HalRng>,
    #[cfg(feature = "withstd")]
    pub rust_rng: Option<ThreadRng>,
    #[cfg(not(feature = "withstd"))]
    pub rust_rng: Option<HalRng>,
}

impl RngCore for RngBoth {
    fn next_u32(&mut self) -> u32 {
        next_u32(self)
    }

    fn next_u64(&mut self) -> u64 {
        next_u64(self)
    }

    fn fill_bytes(&mut self, _dest: &mut [u8]) {
        todo!()
    }

    fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), Error> {
        todo!()
    }
}

#[cfg(target_os = "thumbv7em-none-eabihf")]
fn next_u64(mut rng: &mut RngBoth) -> u64 {
    rng.hal_rng.as_mut().unwrap().next_u64()
}

#[cfg(not(target_os = "thumbv7em-none-eabihf"))]
fn next_u64(rng: &mut RngBoth) -> u64 {
    #[cfg(feature = "withstd")]
    return rng.rust_rng.as_mut().unwrap().next_u64();
    #[cfg(feature = "nostd")]
    return rng.hal_rng.as_mut().unwrap().next_u64();
}


#[cfg(target_os = "thumbv7em-none-eabihf")]
fn next_u32(mut rng: &mut RngBoth) -> u32 {
    rng.hal_rng.as_mut().unwrap().next_u32()
}

#[cfg(not(target_os = "thumbv7em-none-eabihf"))]
fn next_u32(rng: &mut RngBoth) -> u32 {
    #[cfg(feature = "withstd")]
    return rng.rust_rng.as_mut().unwrap().next_u32();
    #[cfg(feature = "nostd")]
    return rng.hal_rng.as_mut().unwrap().next_u32();
}


