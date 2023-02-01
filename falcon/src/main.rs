
mod main_test;
mod fft;
mod shake;

mod falcon_c {
    pub mod codec_c;
    pub mod nist_c;
    pub mod fpr_c;
    pub mod fft_c;
    pub mod shake_c;
    pub mod rng_c;
    pub mod common_c;
    pub mod vrfy_c;
    pub mod keygen_c;
}

mod test {
    pub mod fft_test;
    pub mod shake_test;
}



fn main() {

    println!("{} tissemand", 2);
    println!("Hello, world falcon!");
}

pub fn addd(a: i32, b: i32) -> i32  {
    a + b
}