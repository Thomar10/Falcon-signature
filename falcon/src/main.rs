#![allow(dead_code)]

use crate::gen_kat::genkat;
use crate::test_falcon::run_falcon_tests;

mod fft;
mod fpr;
mod shake;
mod keygen;
mod falcon;
mod codec;
mod vrfy;
mod gen_kat;
mod katrng;
mod nist;
mod rng;
mod sign;
mod common;
mod test_falcon;
mod tests;

mod falcon_c {
    pub mod codec_c;
    pub mod nist_c;
    pub mod fpr_c;
    pub mod fft_c;
    pub mod shake_c;
    pub mod rng_c;
    pub mod common_c;
    pub mod sign_c;
    pub mod vrfy_c;
    pub mod keygen_c;
}



fn main() {
    //run_falcon_tests();
    genkat();
}