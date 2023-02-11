use crate::keygen::keygen;
use crate::falcon_c::shake_c::{falcon_inner_i_shake256_init, falcon_inner_i_shake256_inject, InnerShake256Context as InnerShake256ContextC, St as StC};
use crate::shake::{i_shake256_init, i_shake256_inject, InnerShake256Context, St};
use std::time::Instant;

mod main_test;
mod fft;
mod fpr;
mod shake;
mod keygen;
mod falcon;
mod codec;
mod vrfy;

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
    pub mod fpr_test;
    pub mod shake_test;
    pub mod keygen_test;
    pub mod vrfy_test;
}



fn main() {

    println!("Hello, world falcon!");
    let now = Instant::now();
    for _ in 0..20 {
        for logn in 1..11 {
            let buffer_size = falcon_tmpsize_keygen!(logn);
            let (mut rng_rust, rng_c) = init_shake_with_random_context();
            let mut h: Vec<u16> = vec![0u16; buffer_size];
            let h_c: Vec<u16> = vec![0u16; buffer_size];
            let mut tmp: Vec<u8> = vec![0; buffer_size];
            let tmp_c: Vec<u8> = vec![0; buffer_size];
            let mut F: Vec<i8> = vec![0; buffer_size];
            let F_c: Vec<i8> = vec![0; buffer_size];
            let mut G: Vec<i8> = vec![0; buffer_size];
            let G_c: Vec<i8> = vec![0; buffer_size];
            let mut f: Vec<i8> = vec![0; buffer_size];
            let f_c: Vec<i8> = vec![0; buffer_size];
            let mut g: Vec<i8> = vec![0; buffer_size];
            let g_c: Vec<i8> = vec![0; buffer_size];
            keygen(&mut rng_rust, f.as_mut_ptr(), g.as_mut_ptr(), F.as_mut_ptr(), G.as_mut_ptr(), h.as_mut_ptr(), logn, tmp.as_mut_ptr());
            //unsafe { falcon_inner_keygen(&rng_c, f_c.as_ptr(), g_c.as_ptr(), F_c.as_ptr(), G_c.as_ptr(), h_c.as_ptr(), logn, tmp_c.as_ptr()); }
            /*println!("f");
            assert_eq!(f, f_c);
            println!("g");
            assert_eq!(g, g_c);
            println!("G");
            assert_eq!(G, G_c);
            println!("F");
            assert_eq!(F, F_c);
            println!("h");
            assert_eq!(h, h_c);
            println!("tmp");
            assert_eq!(tmp, tmp_c);*/
        }
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn init_shake_with_random_context() -> (InnerShake256Context, InnerShake256ContextC) {
    let random_state: [u64; 25] = rand::random();
    let random_dptr: u64 = rand::random();
    let st = St { a: random_state };
    let mut sc_rust = InnerShake256Context { st, dptr: random_dptr };
    let sc_c = InnerShake256ContextC { st: StC { a: random_state.clone() }, dptr: random_dptr };
    unsafe { falcon_inner_i_shake256_init(&sc_c as *const InnerShake256ContextC) }
    i_shake256_init(&mut sc_rust);
    let input_rust: [u8; 25] = rand::random();
    let input_c: [u8; 25] = input_rust.clone();
    i_shake256_inject(&mut sc_rust, &input_rust);
    unsafe { falcon_inner_i_shake256_inject(&sc_c as *const InnerShake256ContextC, input_c.as_ptr(), input_c.len() as u64) }
    (sc_rust, sc_c)
}