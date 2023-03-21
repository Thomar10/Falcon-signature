#[cfg(test)]
mod tests {
    use ntru_gen::fxp::{fxr, fxr_div, vect_add, vect_adj_fft, vect_div_autoadj_fft, vect_fft, vect_ifft, vect_invnorm_fft, vect_mul_autoadj_fft, vect_mul_fft, vect_mul_realconst, vect_norm_fft, vect_set};
    use ntru_gen_c::fxp::{ntrugen_inner_fxr_div, ntrugen_vect_add, ntrugen_vect_adj_fft, ntrugen_vect_div_autoadj_fft, ntrugen_vect_FFT, ntrugen_vect_iFFT, ntrugen_vect_invnorm_fft, ntrugen_vect_mul_autoadj_fft, ntrugen_vect_mul_fft, ntrugen_vect_mul_realconst, ntrugen_vect_norm_fft, ntrugen_vect_set};
    use rand::Rng;

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

    #[test]
    fn vect_ifft_test() {
        let logn = 10;
        let mut rng = rand::thread_rng();
        let mut f: [fxr; 1024] = core::array::from_fn(|_| rng.gen::<fxr>());
        let fc: [fxr; 1024] = f.clone();
        vect_ifft(logn, &mut f);
        unsafe { ntrugen_vect_iFFT(logn as u32, fc.as_ptr()); }
        assert_eq!(f, fc);
    }

    #[test]
    fn vect_set_test() {
        let logn = 10;
        let mut rng = rand::thread_rng();
        let mut d: [fxr; 1024] = [0; 1024];
        let dc: [fxr; 1024] = [0; 1024];
        let f: [i8; 1024] = core::array::from_fn(|_| rng.gen::<i8>());
        vect_set(logn, &mut d, &f);
        unsafe { ntrugen_vect_set(logn as u32, dc.as_ptr(), f.as_ptr()); }
        assert_eq!(d, dc);
    }

    #[test]
    fn vect_add_test() {
        let logn = 10;
        let mut rng = rand::thread_rng();
        let mut a: [fxr; 1024] = core::array::from_fn(|_| rng.gen::<fxr>());
        let ac: [fxr; 1024] = a.clone();
        let b: [fxr; 1024] = core::array::from_fn(|_| rng.gen::<fxr>());
        vect_add(logn, &mut a, &b);
        unsafe { ntrugen_vect_add(logn as u32, ac.as_ptr(), b.as_ptr()); }
        assert_eq!(a, ac);
    }

    #[test]
    fn vect_mul_realconst_test() {
        let logn = 10;
        let c: u64 = rand::random();
        let mut rng = rand::thread_rng();
        let mut f: [fxr; 1024] = core::array::from_fn(|_| rng.gen::<fxr>());
        let fc: [fxr; 1024] = f.clone();
        vect_mul_realconst(logn, &mut f, c);
        unsafe { ntrugen_vect_mul_realconst(logn as u32, fc.as_ptr(), c); }
        assert_eq!(f, fc);
    }

    #[test]
    fn vect_mul_fft_test() {
        let logn = 10;
        let mut rng = rand::thread_rng();
        let mut f: [fxr; 1024] = core::array::from_fn(|_| rng.gen::<fxr>());
        let fc: [fxr; 1024] = f.clone();
        let b: [fxr; 1024] = core::array::from_fn(|_| rng.gen::<fxr>());
        vect_mul_fft(logn, &mut f, &b);
        unsafe { ntrugen_vect_mul_fft(logn as u32, fc.as_ptr(), b.as_ptr()); }
        assert_eq!(f, fc);
    }

    #[test]
    fn vect_adj_fft_test() {
        let logn = 10;
        let mut rng = rand::thread_rng();
        let mut f: [fxr; 1024] = core::array::from_fn(|_| rng.gen::<fxr>());
        let fc: [fxr; 1024] = f.clone();
        vect_adj_fft(logn, &mut f);
        unsafe { ntrugen_vect_adj_fft(logn as u32, fc.as_ptr()); }
        assert_eq!(f, fc);
    }

    #[test]
    fn vect_mul_autoadj_fft_test() {
        let logn = 10;
        let mut rng = rand::thread_rng();
        let mut f: [fxr; 1024] = core::array::from_fn(|_| rng.gen::<fxr>());
        let fc: [fxr; 1024] = f.clone();
        let b: [fxr; 1024] = core::array::from_fn(|_| rng.gen::<fxr>());
        vect_mul_autoadj_fft(logn, &mut f, &b);
        unsafe { ntrugen_vect_mul_autoadj_fft(logn as u32, fc.as_ptr(), b.as_ptr()); }
        assert_eq!(f, fc);
    }

    #[test]
    fn vect_div_autoadj_fft_test() {
        let logn = 10;
        let mut rng = rand::thread_rng();
        let mut f: [fxr; 1024] = core::array::from_fn(|_| rng.gen::<fxr>());
        let fc: [fxr; 1024] = f.clone();
        let b: [fxr; 1024] = core::array::from_fn(|_| rng.gen::<fxr>());
        vect_div_autoadj_fft(logn, &mut f, &b);
        unsafe { ntrugen_vect_div_autoadj_fft(logn as u32, fc.as_ptr(), b.as_ptr()); }
        assert_eq!(f, fc);
    }

    #[test]
    fn vect_norm_fft_test() {
        let logn = 10;
        let mut rng = rand::thread_rng();
        let f: [fxr; 1024] = core::array::from_fn(|_| rng.gen::<fxr>());
        let b: [fxr; 1024] = core::array::from_fn(|_| rng.gen::<fxr>());
        let mut c: [fxr; 1024] = [0; 1024];
        let cc: [fxr; 1024] = [0; 1024];
        vect_norm_fft(logn, &mut c, &f, &b);
        unsafe { ntrugen_vect_norm_fft(logn as u32, cc.as_ptr(), f.as_ptr(), b.as_ptr()); }
        assert_eq!(c, cc);
    }

    #[test]
    fn vect_invnorm_fft_test() {
        let logn = 10;
        let e: u32 = 10;
        let mut rng = rand::thread_rng();
        let f: [fxr; 1024] = core::array::from_fn(|_| rng.gen::<fxr>());
        let b: [fxr; 1024] = core::array::from_fn(|_| rng.gen::<fxr>());
        let mut c: [fxr; 1024] = [0; 1024];
        let cc: [fxr; 1024] = [0; 1024];
        vect_invnorm_fft(logn, &mut c, &f, &b, e);
        unsafe { ntrugen_vect_invnorm_fft(logn as u32, cc.as_ptr(), f.as_ptr(), b.as_ptr(), e); }
        assert_eq!(c, cc);
    }
}