extern crate core;

mod fft;
mod fpr;
mod shake;
mod keygen;
mod falcon;
mod codec;
mod vrfy;
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
    pub mod fpr_test;
    pub mod shake_test;
    pub mod rng_test;
    pub mod keygen_test;
    pub mod vrfy_test;
}


fn main() {
    println!("Hello, world falcon!");
    /*let now = Instant::now();
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
            println!("f");
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
            assert_eq!(tmp, tmp_c);
        }
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed); */
}