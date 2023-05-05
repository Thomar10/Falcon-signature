use rand::rngs::ThreadRng;
use rand_core::{Error, RngCore};
use stm32f4xx_hal::rng::Rng as HalRng;

pub struct RngBoth {
    pub hal_rng: Option<HalRng>,
    pub rust_rng: Option<ThreadRng>,
}

impl RngCore for RngBoth {
    fn next_u32(&mut self) -> u32 {
        next_u32(self)
    }

    fn next_u64(&mut self) -> u64 {
        next_u64(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        todo!()
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        todo!()
    }
}

#[cfg(target_os = "thumbv7em-none-eabihf")]
fn next_u64(mut rng: &mut RngBoth) -> u64 {
    rng.hal_rng.as_mut().unwrap().next_u64()
}

#[cfg(not(target_os = "thumbv7em-none-eabihf"))]
fn next_u64(mut rng: &mut RngBoth) -> u64 {
    rng.rust_rng.as_mut().unwrap().next_u64()
}


#[cfg(target_os = "thumbv7em-none-eabihf")]
fn next_u32(mut rng: &mut RngBoth) -> u32 {
    rng.hal_rng.as_mut().unwrap().next_u32()
}

#[cfg(not(target_os = "thumbv7em-none-eabihf"))]
fn next_u32(mut rng: &mut RngBoth) -> u32 {
    rng.rust_rng.as_mut().unwrap().next_u32()
}
