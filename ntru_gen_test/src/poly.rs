#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use ntru_gen::fxp::fxr;
    use ntru_gen::poly::{poly_big_to_fixed, poly_big_to_small, poly_max_bitlength, poly_mp_norm, poly_mp_set, poly_mp_set_small, poly_sqnorm, poly_sub_kfg_scaled_depth1, poly_sub_scaled, poly_sub_scaled_ntt};
    use ntru_gen_c::poly::{ntrugen_poly_big_to_fixed, ntrugen_poly_big_to_small, ntrugen_poly_max_bitlength, ntrugen_poly_mp_norm, ntrugen_poly_mp_set, ntrugen_poly_mp_set_small, ntrugen_poly_sqnorm, ntrugen_poly_sub_kfg_scaled_depth1, ntrugen_poly_sub_scaled, ntrugen_poly_sub_scaled_ntt};
    use rand::Rng;

    const P: u32 = 12289;

    #[test]
    fn poly_mp_set_small_test() {
        let logn = 10;
        let mut rng = rand::thread_rng();
        let f: [i8; 1024] = core::array::from_fn(|_| rng.gen::<i8>());
        let mut d: [u32; 1024] = [0; 1024];
        let dc: [u32; 1024] = [0; 1024];
        poly_mp_set_small(logn, &mut d, &f, P);
        unsafe { ntrugen_poly_mp_set_small(logn as u32, dc.as_ptr(), f.as_ptr(), P); }
        assert_eq!(d, dc);
    }

    #[test]
    fn poly_mp_set_test() {
        let logn = 10;
        let mut rng = rand::thread_rng();
        let mut d: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
        let dc: [u32; 1024] = d.clone();
        poly_mp_set(logn, &mut d, P);
        unsafe { ntrugen_poly_mp_set(logn as u32, dc.as_ptr(), P); }
        assert_eq!(d, dc);
    }

    #[test]
    fn poly_mp_norm_test() {
        let logn = 10;
        let mut rng = rand::thread_rng();
        let mut d: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
        let dc: [u32; 1024] = d.clone();
        poly_mp_norm(logn, &mut d, P);
        unsafe { ntrugen_poly_mp_norm(logn as u32, dc.as_ptr(), P); }
        assert_eq!(d, dc);
    }

    #[test]
    fn poly_big_to_small_test() {
        let logn = 10;
        let mut rng = rand::thread_rng();
        let mut d: [i8; 1024] = [0; 1024];
        let dc: [i8; 1024] = [0; 1024];
        let s: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
        let ss = s.iter().map(|x| x % P).collect::<Vec<_>>();
        poly_big_to_small(logn, &mut d, ss.as_slice(), P as i32);
        unsafe { ntrugen_poly_big_to_small(logn as u32, dc.as_ptr(), ss.as_ptr(), P as i32); }
        assert_eq!(d, dc);
    }

    #[test]
    fn poly_max_bitlength_test() {
        let logn = 10;
        let mut rng = rand::thread_rng();
        let mut d: [u32; 1024 * 10] = core::array::from_fn(|_| rng.gen::<u32>());
        let dc: [u32; 1024 * 10] = d.clone();
        let res = poly_max_bitlength(logn, &mut d, 10);
        let resc = unsafe { ntrugen_poly_max_bitlength(logn as u32, dc.as_ptr(), 10) };
        assert_eq!(res, resc);
    }

    #[test]
    fn poly_big_to_fixed_test() {
        let logn = 10;
        let mut rng = rand::thread_rng();
        let f: [u32; 1024 * 5] = core::array::from_fn(|_| rng.gen::<u32>());
        let mut d: [fxr; 1024 * 5] = [0; 1024 * 5];
        let dc: [fxr; 1024 * 5] = [0; 1024 * 5];
        poly_big_to_fixed(logn, &mut d, &f, 5, P);
        unsafe { ntrugen_poly_big_to_fixed(logn as u32, dc.as_ptr(), f.as_ptr(), 5, P); }
        assert_eq!(d, dc);
    }

    #[test]
    fn poly_sub_scaled_test() {
        let logn = 10;
        let mut rng = rand::thread_rng();
        let mut F: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
        let Fc: [u32; 1024] = F.clone();
        let f: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
        let k: [i32; 1024] = core::array::from_fn(|_| rng.gen::<i32>());
        let sc = 12348;
        poly_sub_scaled(logn, &mut F, 10, &f, 10, &k, sc);
        unsafe { ntrugen_poly_sub_scaled(logn as u32, Fc.as_ptr(), 10, f.as_ptr(), 10, k.as_ptr(), sc); }
        assert_eq!(F, Fc);
    }

    #[test]
    fn poly_sub_scaled_ntt_test() {
        let logn = 10;
        let mut rng = rand::thread_rng();
        let mut F: [u32; 1024 * 7] = core::array::from_fn(|_| rng.gen::<u32>());
        let Fc: [u32; 1024 * 7] = F.clone();
        let f: [u32; 1024 * 7] = core::array::from_fn(|_| rng.gen::<u32>());
        let k: [i32; 1024 * 7] = core::array::from_fn(|_| rng.gen::<i32>());
        let mut tmp: [u32; 7 * 1024] = [0; 7 * 1024];
        let tmpc: [u32; 7 * 1024] = [0; 7 * 1024];
        let sc: u32 = 13254;
        let lengthf = 3;
        poly_sub_scaled_ntt(logn, &mut F, lengthf, &f, lengthf, &k, sc, &mut tmp);
        unsafe { ntrugen_poly_sub_scaled_ntt(logn as u32, Fc.as_ptr(), lengthf, f.as_ptr(), lengthf, k.as_ptr(), sc, tmpc.as_ptr()) };
        assert_eq!(F, Fc);
        assert_eq!(tmp, tmpc);
    }

    #[test]
    fn poly_sqnorm_test() {
        let logn = 10;
        let mut rng = rand::thread_rng();
        let f: [i8; 1024] = core::array::from_fn(|_| rng.gen::<i8>());

        let res = poly_sqnorm(logn, &f);
        let resc = unsafe { ntrugen_poly_sqnorm(logn as u32, f.as_ptr()) };
        assert_eq!(res, resc);
    }

    #[test]
    fn poly_sub_kfg_scaled_depth1_test() {
        let logn = 2;
        let mut rng = rand::thread_rng();
        let mut F: [u32; 2048] = [0; 2048];
        let Fc: [u32; 2048] = [0; 2048];
        let mut G: [u32; 2048] = [0; 2048];
        let Gc: [u32; 2048] = [0; 2048];
        let mut k: [u32; 2048] = core::array::from_fn(|_| rng.gen::<u32>());
        let kc: [u32; 2048] = k.clone();
        let f: [i8; 2048] = core::array::from_fn(|_| rng.gen::<i8>());
        let g: [i8; 2048] = core::array::from_fn(|_| rng.gen::<i8>());
        let mut tmp: [u32; 10 * 1024] = [0; 10 * 1024];
        let tmpc: [u32; 10 * 1024] = [0; 10 * 1024];
        let sc = 976234;
        poly_sub_kfg_scaled_depth1(logn, &mut F, &mut G, 1, &mut k, sc, &f, &g, &mut tmp);
        unsafe { ntrugen_poly_sub_kfg_scaled_depth1(logn as u32, Fc.as_ptr(), Gc.as_ptr(), 1, kc.as_ptr(), sc, f.as_ptr(), g.as_ptr(), tmpc.as_ptr()) };
        assert_eq!(tmp, tmpc);
        assert_eq!(k, kc);
        assert_eq!(F, Fc);
        assert_eq!(G, Gc);
        let mut k: [u32; 2048] = core::array::from_fn(|_| rng.gen::<u32>());
        let kc: [u32; 2048] = k.clone();
        poly_sub_kfg_scaled_depth1(logn, &mut F, &mut G, 2, &mut k, sc, &f, &g, &mut tmp);
        unsafe { ntrugen_poly_sub_kfg_scaled_depth1(logn as u32, Fc.as_ptr(), Gc.as_ptr(), 2, kc.as_ptr(), sc, f.as_ptr(), g.as_ptr(), tmpc.as_ptr()) };
        assert_eq!(k, kc);
        assert_eq!(tmp, tmpc);
        // assert_eq!(F, Fc);
        // assert_eq!(G, Gc);
    }
}