#![allow(dead_code)]
#![no_std]
extern crate core;
#[macro_use]
extern crate alloc;

pub mod fft;
pub mod fpr;
pub mod shake;
pub mod keygen;
pub mod falcon;
pub mod codec;
pub mod vrfy;
pub mod rng;
pub mod sign;
pub mod common;
