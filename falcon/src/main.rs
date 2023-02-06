use rand::Rng;

mod main_test;
mod fft;
mod shake;
mod rng;

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
    pub mod rng_test;
}



fn main() {

    println!("Hello, world falcon!");

    let mut rng = rand::thread_rng();
    let _array: [u8; 10] = core::array::from_fn(|_| rng.gen::<u8>());

}

pub fn addd(a: i32, b: i32) -> i32  {
    a + b
}