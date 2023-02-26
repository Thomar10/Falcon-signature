#![allow(dead_code)]

mod fft;
mod fpr;
mod shake;
mod keygen;
mod falcon;
mod codec;
mod vrfy;
mod katrng;
mod nist;
mod rng;
mod sign;
mod common;
mod tests;
mod kat_tests;

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
    pub mod test_falcon_c;
}



fn main() {
    println!("HEJ :)");
}