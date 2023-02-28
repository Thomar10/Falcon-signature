#[cfg(test)]
mod tests {
    use rand::Rng;

    use falcon::fft::{{fft, ifft, poly_add, poly_add_muladj_fft, poly_adj_fft, poly_div_autoadj_fft, poly_div_fft, poly_invnorm2_fft, poly_LDL_fft, poly_LDLmv_fft, poly_merge_fft, poly_mul_autoadj_fft, poly_mul_fft, poly_muladj_fft, poly_mulconst, poly_mulselfadj_fft, poly_neg, poly_split_fft, poly_sub}};

    use crate::falcon_c::fft_c::{falcon_inner_FFT, falcon_inner_iFFT, falcon_inner_poly_add, falcon_inner_poly_add_muladj_fft, falcon_inner_poly_adj_fft, falcon_inner_poly_div_autoadj_fft, falcon_inner_poly_div_fft, falcon_inner_poly_invnorm2_fft, falcon_inner_poly_LDL_fft, falcon_inner_poly_LDLmv_fft, falcon_inner_poly_merge_fft, falcon_inner_poly_mul_autoadj_fft, falcon_inner_poly_mul_fft, falcon_inner_poly_muladj_fft, falcon_inner_poly_mulconst, falcon_inner_poly_mulselfadj_fft, falcon_inner_poly_neg, falcon_inner_poly_split_fft, falcon_inner_poly_sub};

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

    #[test]
    fn test_poly_add() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut a: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let a_c = a.clone();
                let mut b: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                poly_add(&mut a, &mut b, logn);
                unsafe { falcon_inner_poly_add(a_c.as_ptr(), b.as_ptr(), logn) };
                assert_eq!(a, a_c);
            }
        }
    }

    #[test]
    fn test_poly_sub() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut a: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let a_c = a.clone();
                let b: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                poly_sub(&mut a, &b, logn);
                unsafe { falcon_inner_poly_sub(a_c.as_ptr(), b.as_ptr(), logn) };
                assert_eq!(a, a_c);
            }
        }
    }

    #[test]
    fn test_poly_neg() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut a: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let a_c = a.clone();
                poly_neg(&mut a, logn);
                unsafe { falcon_inner_poly_neg(a_c.as_ptr(), logn) };
                assert_eq!(a, a_c);
            }
        }
    }

    #[test]
    fn test_poly_adj_fft() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut a: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let a_c = a.clone();
                poly_adj_fft(&mut a, logn);
                unsafe { falcon_inner_poly_adj_fft(a_c.as_ptr(), logn) };
                assert_eq!(a, a_c);
            }
        }
    }

    #[test]
    fn test_poly_mul_fft() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut a: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let a_c = a.clone();
                let b: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                poly_mul_fft(&mut a, &b, logn);
                unsafe { falcon_inner_poly_mul_fft(a_c.as_ptr(), b.as_ptr(), logn) };
                assert_eq!(a, a_c);
            }
        }
    }

    #[test]
    fn test_poly_muladj_fft() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut a: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let a_c = a.clone();
                let b: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                poly_muladj_fft(&mut a, &b, logn);
                unsafe { falcon_inner_poly_muladj_fft(a_c.as_ptr(), b.as_ptr(), logn) };
                assert_eq!(a, a_c);
            }
        }
    }

    #[test]
    fn test_poly_mulselfadj_fft() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut a: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let a_c = a.clone();
                poly_mulselfadj_fft(&mut a, logn);
                unsafe { falcon_inner_poly_mulselfadj_fft(a_c.as_ptr(), logn) };
                assert_eq!(a, a_c);
            }
        }
    }

    #[test]
    fn test_poly_mulconst() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut a: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let x: u64 = rand::random();
                let a_c = a.clone();
                poly_mulconst(&mut a, x, logn);
                unsafe { falcon_inner_poly_mulconst(a_c.as_ptr(), x, logn) };
                assert_eq!(a, a_c);
            }
        }
    }

    #[test]
    fn test_poly_div_fft() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut a: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let a_c = a.clone();
                let b: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                poly_div_fft(&mut a, &b, logn);
                unsafe { falcon_inner_poly_div_fft(a_c.as_ptr(), b.as_ptr(), logn) };
                assert_eq!(a, a_c);
            }
        }
    }

    #[test]
    fn test_poly_invnorm2_fft() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let a: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let b: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let mut d: [u64; 1024] = [0; 1024];
                let d_c = d.clone();
                poly_invnorm2_fft(&mut d, &a, &b, logn);
                unsafe { falcon_inner_poly_invnorm2_fft(d_c.as_ptr(), a.as_ptr(), b.as_ptr(), logn) };
                assert_eq!(d, d_c);
            }
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_poly_add_muladj_fft() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let f: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let g: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let F: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let G: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let mut d: [u64; 1024] = [0; 1024];
                let d_c = d.clone();
                poly_add_muladj_fft(&mut d, &F, &G, &f, &g, logn);
                unsafe { falcon_inner_poly_add_muladj_fft(d_c.as_ptr(), F.as_ptr(), G.as_ptr(), f.as_ptr(), g.as_ptr(), logn) };
                assert_eq!(d, d_c);
            }
        }
    }

    #[test]
    fn test_poly_mul_autoadj_fft() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut a: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let a_c: [u64; 1024] = a.clone();
                let b: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                poly_mul_autoadj_fft(&mut a, &b, logn);
                unsafe { falcon_inner_poly_mul_autoadj_fft(a_c.as_ptr(), b.as_ptr(), logn) };
                assert_eq!(a, a_c);
            }
        }
    }

    #[test]
    fn test_poly_div_autoadj_fft() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut a: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let a_c: [u64; 1024] = a.clone();
                let b: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                poly_div_autoadj_fft(&mut a, &b, logn);
                unsafe { falcon_inner_poly_div_autoadj_fft(a_c.as_ptr(), b.as_ptr(), logn) };
                assert_eq!(a, a_c);
            }
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_poly_LDL_fft() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let g00: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let g00_c: [u64; 1024] = g00.clone();
                let mut g01: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let g01_c: [u64; 1024] = g01.clone();
                let mut g11: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let g11_c: [u64; 1024] = g11.clone();
                poly_LDL_fft(&g00, &mut g01, &mut g11, logn);
                unsafe { falcon_inner_poly_LDL_fft(g00_c.as_ptr(), g01_c.as_ptr(), g11_c.as_ptr(), logn) };
                assert_eq!(g00, g00_c);
                assert_eq!(g01, g01_c);
                assert_eq!(g11, g11_c);
            }
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_poly_LDLmv_fft() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut l01: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let l01_c: [u64; 1024] = l01.clone();
                let mut d11: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let d11_c: [u64; 1024] = d11.clone();
                let g00: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let g00_c: [u64; 1024] = g00.clone();
                let g01: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let g01_c: [u64; 1024] = g01.clone();
                let g11: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let g11_c: [u64; 1024] = g11.clone();
                poly_LDLmv_fft(&mut d11, &mut l01, &g00, &g01, &g11, logn);
                unsafe { falcon_inner_poly_LDLmv_fft(d11_c.as_ptr(), l01_c.as_ptr(), g00_c.as_ptr(), g01_c.as_ptr(), g11_c.as_ptr(), logn) };
                assert_eq!(d11, d11_c);
                assert_eq!(l01, l01_c);
                assert_eq!(g00, g00_c);
                assert_eq!(g01, g01_c);
                assert_eq!(g11, g11_c);
            }
        }
    }

    #[test]
    fn test_poly_split_fft() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut f0: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let f0_c: [u64; 1024] = f0.clone();
                let mut f1: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let f1_c: [u64; 1024] = f1.clone();
                let f: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let f_c: [u64; 1024] = f.clone();
                poly_split_fft(&mut f0, &mut f1, &f, logn);
                unsafe { falcon_inner_poly_split_fft(f0_c.as_ptr(), f1_c.as_ptr(), f_c.as_ptr(), logn) };
                assert_eq!(f0, f0_c);
                assert_eq!(f1, f1_c);
                assert_eq!(f, f_c);
            }
        }
    }

    #[test]
    fn test_poly_merge_fft() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut f0: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let f0_c: [u64; 1024] = f0.clone();
                let mut f1: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let f1_c: [u64; 1024] = f1.clone();
                let mut f: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let f_c: [u64; 1024] = f.clone();
                poly_split_fft(&mut f0, &mut f1, &f, logn);
                unsafe { falcon_inner_poly_split_fft(f0_c.as_ptr(), f1_c.as_ptr(), f_c.as_ptr(), logn) };
                poly_merge_fft(&mut f, &f0, &f1, logn);
                unsafe { falcon_inner_poly_merge_fft(f_c.as_ptr(), f0_c.as_ptr(), f1_c.as_ptr(), logn) };
                assert_eq!(f0, f0_c);
                assert_eq!(f1, f1_c);
                assert_eq!(f, f_c);
            }
        }
    }
}