#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::falcon_c::vrfy_c::{falcon_inner_complete_private_func, falcon_inner_compute_public_func, falcon_inner_count_nttzero_func, falcon_inner_is_invertible_func, falcon_inner_to_ntt_monty_func, falcon_inner_verify_raw_func, falcon_inner_verify_recover_func, mq_add_func, mq_div_12289_func, mq_iNTT_func, mq_montymul_func, mq_montysqr_func, mq_NTT_func, mq_poly_montymul_ntt_func, mq_poly_sub_func, mq_poly_tomonty_func, mq_rshift1_func, mq_sub_func};
    use crate::falcon_tmpsize_keygen;
    use crate::keygen::keygen;
    use crate::shake::{InnerShake256Context, St};
    use crate::vrfy::{complete_private, compute_public, count_nttzero, is_invertible, mq_add, mq_div_12289, mq_innt, mq_montymul, mq_montysqr, mq_ntt, mq_poly_montymul_ntt, mq_poly_sub, mq_poly_tomonty, mq_rshift1, mq_sub, to_ntt_monty, verify_raw, verify_recover};

    #[test]
    fn test_monty_mul() {
        for _ in 0..1000 {
            let x: u32 = rand::random();
            let y: u32 = rand::random();
            let res = mq_montymul(x, y);
            let res_c = unsafe { mq_montymul_func(x, y) };
            assert_eq!(res, res_c);
        }
    }

    #[test]
    fn test_monty_add() {
        for _ in 0..1000 {
            let x: u32 = rand::random();
            let y: u32 = rand::random();
            let res = mq_add(x, y);
            let res_c = unsafe { mq_add_func(x, y) };
            assert_eq!(res, res_c);
        }
    }

    #[test]
    fn test_monty_sub() {
        for _ in 0..1000 {
            let x: u32 = rand::random();
            let y: u32 = rand::random();
            let res = mq_sub(x, y);
            let res_c = unsafe { mq_sub_func(x, y) };
            assert_eq!(res, res_c);
        }
    }

    #[test]
    fn test_mq_rshift1() {
        for _ in 0..1000 {
            let x: u32 = rand::random();
            let res = mq_rshift1(x);
            let res_c = unsafe { mq_rshift1_func(x) };
            assert_eq!(res, res_c);
        }
    }

    #[test]
    fn test_montysqr() {
        for _ in 0..1000 {
            let x: u32 = rand::random();
            let res = mq_montysqr(x);
            let res_c = unsafe { mq_montysqr_func(x) };
            assert_eq!(res, res_c);
        }
    }

    #[test]
    fn test_div_12289() {
        for _ in 0..1000 {
            let x: u32 = rand::random();
            let y: u32 = rand::random();
            let res = mq_div_12289(x, y);
            let res_c = unsafe { mq_div_12289_func(x, y) };
            assert_eq!(res, res_c);
        }
    }

    #[test]
    fn test_mq_ntt() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut h: [u16; 512] = core::array::from_fn(|_| rng.gen::<u16>());
                let h_c = h.clone();
                mq_ntt(h.as_mut_ptr(), logn);
                unsafe { mq_NTT_func(h_c.as_ptr(), logn) };
                assert_eq!(h, h_c);
            }
        }
    }

    #[test]
    fn test_mq_intt() {
        for logn in 1..10 {
            let mut rng = rand::thread_rng();
            let mut h: [u16; 512] = core::array::from_fn(|_| rng.gen::<u16>());
            let h_c = h.clone();
            mq_innt(h.as_mut_ptr(), logn);
            unsafe { mq_iNTT_func(h_c.as_ptr(), logn) };
            assert_eq!(h, h_c);
        }
    }

    #[test]
    fn test_mq_poly_tomonty() {
        for _ in 0..1000 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut h: [u16; 512] = core::array::from_fn(|_| rng.gen::<u16>());
                let h_c = h.clone();
                mq_poly_tomonty(&mut h, logn);
                unsafe { mq_poly_tomonty_func(h_c.as_ptr(), logn) };
                assert_eq!(h, h_c);
            }
        }
    }

    #[test]
    fn test_mq_poly_montymul_ntt() {
        for _ in 0..1000 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut f: [u16; 1024] = core::array::from_fn(|_| rng.gen::<u16>());
                let f_c = f.clone();
                let mut g: [u16; 512] = core::array::from_fn(|_| rng.gen::<u16>());
                let g_c = g.clone();
                mq_poly_montymul_ntt(f.as_mut_ptr(), &mut g, logn);
                unsafe { mq_poly_montymul_ntt_func(f_c.as_ptr(), g.as_ptr(), logn) };
                assert_eq!(f, f_c);
                assert_eq!(g, g_c);
            }
        }
    }

    #[test]
    fn test_mq_poly_sub() {
        for _ in 0..1000 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut f: [u16; 512] = core::array::from_fn(|_| rng.gen::<u16>());
                let f_c = f.clone();
                let mut g: [u16; 512] = core::array::from_fn(|_| rng.gen::<u16>());
                let g_c = g.clone();
                mq_poly_sub(f.as_mut_ptr(), &mut g, logn);
                unsafe { mq_poly_sub_func(f_c.as_ptr(), g.as_ptr(), logn) };
                assert_eq!(f, f_c);
                assert_eq!(g, g_c);
            }
        }
    }

    #[test]
    fn test_to_ntt_monty() {
        for _ in 0..1000 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut f: [u16; 1024] = core::array::from_fn(|_| rng.gen::<u16>());
                let f_c = f.clone();
                to_ntt_monty(&mut f, logn);
                unsafe { falcon_inner_to_ntt_monty_func(f_c.as_ptr(), logn) };
                assert_eq!(f, f_c);
            }
        }
    }

    #[test]
    fn test_verify_raw() {
        for _ in 0..1000 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut tmp: [u8; 1024] = [0; 1024];
                let tmp_c = tmp.clone();
                let mut hm: [u16; 512] = core::array::from_fn(|_| rng.gen::<u16>());
                let hm_c = hm.clone();
                let mut sig: [i16; 512] = core::array::from_fn(|_| rng.gen::<i16>());
                let sig_c = sig.clone();
                let mut h: [u16; 512] = core::array::from_fn(|_| rng.gen::<u16>());
                let h_c = h.clone();
                let res = verify_raw(&mut hm, &mut sig, &mut h, logn, &mut tmp);
                let res_c = unsafe { falcon_inner_verify_raw_func(hm_c.as_ptr(), sig_c.as_ptr(), h_c.as_ptr(), logn, tmp_c.as_ptr()) };
                assert_eq!(tmp, tmp_c);
                assert_eq!(res, res_c != 0);
            }
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_compute_public() {
        for _ in 0..10 {
            for logn in 1..11 {
                let buffer_size = falcon_tmpsize_keygen!(logn);
                let mut rng_rust = InnerShake256Context {
                    st: St { a: [0; 25] },
                    dptr: 10,
                };
                let mut h: Vec<u16> = vec![0u16; buffer_size];
                let mut tmp: Vec<u8> = vec![0; buffer_size];
                let mut F: Vec<i8> = vec![0; buffer_size];
                let mut G: Vec<i8> = vec![0; buffer_size];
                let mut f: Vec<i8> = vec![0; buffer_size];
                let mut g: Vec<i8> = vec![0; buffer_size];
                keygen(&mut rng_rust, f.as_mut_ptr(), g.as_mut_ptr(), F.as_mut_ptr(), G.as_mut_ptr(), h.as_mut_ptr(), logn, tmp.as_mut_ptr());
                let res = compute_public(h.as_mut_ptr(), f.as_mut_ptr(), g.as_mut_ptr(), logn, tmp.as_mut_ptr());
                let res_c = unsafe { falcon_inner_compute_public_func(h.as_mut_ptr(), f.as_mut_ptr(), g.as_mut_ptr(), logn, tmp.as_mut_ptr()) };
                assert_eq!(res, res_c != 0);
                assert_eq!(res, true);
            }
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_complete_private() {
        for _ in 0..10 {
            for logn in 1..11 {
                let buffer_size = falcon_tmpsize_keygen!(logn);
                let mut rng_rust = InnerShake256Context {
                    st: St { a: [0; 25] },
                    dptr: 10,
                };
                let mut h: Vec<u16> = vec![0u16; buffer_size];
                let mut tmp_gen: Vec<u8> = vec![0; buffer_size * 4];
                let mut tmp: Vec<u8> = vec![0; buffer_size * 4];
                let tmp_c = tmp.clone();
                let mut F: Vec<i8> = vec![0; buffer_size];
                let mut G_gen: Vec<i8> = vec![0; buffer_size];
                let mut G: Vec<i8> = vec![0; buffer_size];
                let G_c = G.clone();
                let mut f: Vec<i8> = vec![0; buffer_size];
                let mut g: Vec<i8> = vec![0; buffer_size];
                keygen(&mut rng_rust, f.as_mut_ptr(), g.as_mut_ptr(), F.as_mut_ptr(), G_gen.as_mut_ptr(), h.as_mut_ptr(), logn, tmp_gen.as_mut_ptr());
                let res = complete_private(&mut G, &mut f, &mut g, &mut F, logn, &mut tmp);
                let res_c = unsafe { falcon_inner_complete_private_func(G_c.as_ptr(), f.as_ptr(), g.as_ptr(), F.as_ptr(), logn, tmp_c.as_ptr()) };
                assert_eq!(res, res_c != 0);
                assert_eq!(G, G_c);
                assert_eq!(G, G_gen);
                assert_eq!(tmp, tmp_c);
            }
        }
    }

    #[test]
    fn test_is_invertible() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut tmp: [u8; 1024] = [0; 1024];
                let tmp_c = tmp.clone();
                let mut s2: [i16; 1024] = core::array::from_fn(|_| rng.gen::<i16>());
                let s2_c = s2.clone();
                let res = is_invertible(&mut s2, logn, &mut tmp);
                let res_c = unsafe { falcon_inner_is_invertible_func(s2_c.as_ptr(), logn, tmp_c.as_ptr()) };
                assert_eq!(res, res_c != 0);
                assert_eq!(tmp, tmp_c);
            }
        }
    }

    // TODO this test should probably rely on a correct signature to be fully correct test
    // However the test run through all the code, yet no guarantee that h is correct
    // as this is something the caller should check
    #[test]
    fn test_verify_recover() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut tmp: [u8; 1024] = [0; 1024];
                let tmp_c = tmp.clone();
                let mut h: [u16; 512] = core::array::from_fn(|_| rng.gen::<u16>());
                let h_c = h.clone();
                let mut c0: [u16; 512] = core::array::from_fn(|_| rng.gen::<u16>());
                let c0_c = c0.clone();
                let mut s1: [i16; 512] = core::array::from_fn(|_| rng.gen::<i16>());
                let s1_c = s1.clone();
                let mut s2: [i16; 512] = core::array::from_fn(|_| rng.gen::<i16>());
                let s2_c = s2.clone();
                let res = verify_recover(&mut h, &mut c0, &mut s1, &mut s2, logn, &mut tmp);
                let res_c = unsafe { falcon_inner_verify_recover_func(h_c.as_ptr(), c0_c.as_ptr(), s1_c.as_ptr(), s2_c.as_ptr(), logn, tmp_c.as_ptr()) };
                assert_eq!(res, res_c != 0);
                assert_eq!(tmp, tmp_c);
                assert_eq!(h, h_c);
            }
        }
    }

    #[test]
    fn test_count_nttzero() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut tmp: [u8; 1024] = [0; 1024];
                let tmp_c = tmp.clone();
                let mut sig: [i16; 1024] = core::array::from_fn(|_| rng.gen::<i16>());
                let sig_c = sig.clone();
                let res = count_nttzero(&mut sig, logn, &mut tmp);
                let res_c = unsafe { falcon_inner_count_nttzero_func(sig_c.as_ptr(), logn, tmp_c.as_ptr()) };
                assert_eq!(res, res_c);
                assert_eq!(tmp, tmp_c);
            }
        }
    }
}