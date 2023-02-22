#![allow(dead_code)]
use crate::gen_kat::genkat512;
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

mod test {
    pub mod fft_test;
    pub mod fpr_test;
    pub mod shake_test;
    pub mod rng_test;
    pub mod sign_test;
    pub mod keygen_test;
    pub mod vrfy_test;
    pub mod codec_test;
    pub mod nist_test;
    pub mod katrng_test;
    pub mod common_test;
}


fn main() {
    run_falcon_tests();
    //unsafe { genkat512(); }
}