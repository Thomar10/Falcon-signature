#[cfg(test)]
mod tests {
    use rand::Rng;

    use ntru_gen::fxp::{fxr, fxr_div, vect_fft};
    use ntru_gen_c::fxp::{ntrugen_inner_fxr_div, ntrugen_vect_FFT};

    const P: u32 = 12289;

    #[test]
    fn fxr_div_test() {
        let x: u64 = rand::random();
        let y: u64 = rand::random();
        assert_eq!(fxr_div(x, y), unsafe { ntrugen_inner_fxr_div(x, y) });
    }


    #[test]
    fn vect_fft_test() {
        let logn = 10;
        let mut rng = rand::thread_rng();
        let mut f: [fxr; 1024] = core::array::from_fn(|_| rng.gen::<fxr>());
        let fc: [fxr; 1024] = f.clone();
        vect_fft(logn, &mut f);
        unsafe { ntrugen_vect_FFT(logn as u32, fc.as_ptr()); }
        assert_eq!(f, fc);
    }
}