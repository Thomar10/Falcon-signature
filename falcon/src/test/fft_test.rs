#[cfg(test)]
mod tests {
    use rand::Rng;
    use crate::falcon_c::fft_c::{falcon_inner_FFT, falcon_inner_iFFT};
    use crate::fft::{fft, ifft};

    #[test]
    fn test_fft() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut f: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let f_c = f.clone();
                fft(&mut f, logn);
                unsafe { falcon_inner_FFT(f_c.as_ptr(), logn) };
                assert_eq!(f, f_c);
            }
        }
    }

    #[test]
    fn test_ifft() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut f: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let f_c = f.clone();
                fft(&mut f, logn);
                ifft(&mut f, logn);
                unsafe { falcon_inner_FFT(f_c.as_ptr(), logn) };
                unsafe { falcon_inner_iFFT(f_c.as_ptr(), logn) };
                assert_eq!(f, f_c);
            }
        }
    }
}