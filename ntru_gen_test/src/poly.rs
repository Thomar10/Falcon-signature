#[cfg(test)]
mod tests {
    use ntru_gen::fxp::fxr;
    use ntru_gen::poly::{poly_big_to_fixed, poly_big_to_small, poly_max_bitlength, poly_mp_norm, poly_mp_set, poly_mp_set_small};
    use ntru_gen_c::poly::{ntrugen_poly_big_to_fixed, ntrugen_poly_big_to_small, ntrugen_poly_max_bitlength, ntrugen_poly_mp_norm, ntrugen_poly_mp_set, ntrugen_poly_mp_set_small};
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
}