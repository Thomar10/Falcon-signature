#![allow(non_snake_case)]

#[cfg(test)]
mod tests {
    use rand::Rng;

    use ntru_gen::falcon_ntru::{FALCON_1024, FALCON_256, FALCON_512, GAUSS_FALCON_1024, GAUSS_FALCON_256, GAUSS_FALCON_512};
    use ntru_gen::gauss::gauss_sample_poly;
    use ntru_gen::ntru::{make_fg_intermediate, make_fg_step, NtruProfile, solve_ntru, solve_ntru_deepest, solve_ntru_depth0, solve_ntru_intermediate};
    use ntru_gen::prng::{NtruPrngChacha8Context, prng_chacha8_out};
    use ntru_gen_c::ntru::{make_fg_intermediate_test, make_fg_step_test, ntrugen_solve_NTRU, NtruProfileC, solve_NTRU_deepest_test, solve_NTRU_depth0_test, solve_NTRU_intermediate_test};

    #[test]
    fn test_make_fg_step() {
        for logn in 8..11 {
            for depth in 0..5 {
                let (profile, profilec) = get_profiles(logn);
                let mut tmp: [u32; 50000] = [0; 50000];
                let tmpc: [u32; 50000] = [0; 50000];
                make_fg_step(&profile, logn + depth, depth as u32, &mut tmp);
                unsafe { make_fg_step_test(&profilec, (logn + depth) as u32, depth as u32, tmpc.as_ptr()); }
                assert_eq!(tmp, tmpc);
            }
        }
    }

    #[test]
    fn test_make_fg_intermediate() {
        let logn = 9;
        let depth = 1;
        let (profile, profilec) = get_profiles(10);
        let mut f: [i8; 1024] = [0; 1024];
        let mut g: [i8; 1024] = [0; 1024];
        let gauss = get_gauss(logn);
        let mut ctx = NtruPrngChacha8Context {
            d: [0; 40],
        };
        gauss_sample_poly(logn, &mut f, gauss, prng_chacha8_out, &mut ctx);
        gauss_sample_poly(logn, &mut g, gauss, prng_chacha8_out, &mut ctx);
        let mut tmp: [u32; 5000] = [0; 5000];
        let tmpc: [u32; 5000] = [0; 5000];
        make_fg_intermediate(&profile, logn, &f, &g, depth as u32, &mut tmp);
        unsafe { make_fg_intermediate_test(&profilec, logn as u32, f.as_ptr(), g.as_ptr(), depth as u32, tmpc.as_ptr()); }
        assert_eq!(tmp, tmpc);
    }

    #[test]
    fn test_solve_ntru_depth0() {
        for logn in 8..11 {
            let (profile, profilec) = get_profiles(logn);
            let gauss = get_gauss(logn);
            let mut tmp: [u32; 10000] = [0; 10000];
            let tmpc: [u32; 10000] = [0; 10000];
            let mut f: [i8; 1024] = [0; 1024];
            let mut g: [i8; 1024] = [0; 1024];
            let mut ctx = NtruPrngChacha8Context {
                d: [0; 40],
            };
            gauss_sample_poly(logn, &mut f, gauss, prng_chacha8_out, &mut ctx);
            gauss_sample_poly(logn, &mut g, gauss, prng_chacha8_out, &mut ctx);
            let res = solve_ntru_depth0(&profile, logn, &f, &g, &mut tmp);
            let resc = unsafe { solve_NTRU_depth0_test(&profilec, logn as u32, f.as_ptr(), g.as_ptr(), tmpc.as_ptr()) };
            assert_eq!(tmp, tmpc);
            assert_eq!(res, resc == 0);
        }
    }

    #[test]
    fn test_solve_ntru_deepest() {
        for logn in 8..11 {
            let (profile, profilec) = get_profiles(logn);
            let gauss = get_gauss(logn);
            let mut tmp: [u32; 10000] = [0; 10000];
            let tmpc: [u32; 10000] = [0; 10000];
            let mut f: [i8; 1024] = [0; 1024];
            let mut g: [i8; 1024] = [0; 1024];
            let mut ctx = NtruPrngChacha8Context {
                d: [0; 40],
            };
            gauss_sample_poly(logn, &mut f, gauss, prng_chacha8_out, &mut ctx);
            gauss_sample_poly(logn, &mut g, gauss, prng_chacha8_out, &mut ctx);
            let res = solve_ntru_deepest(&profile, logn, &f, &g, &mut tmp);
            let resc = unsafe { solve_NTRU_deepest_test(&profilec, logn as u32, f.as_ptr(), g.as_ptr(), tmpc.as_ptr()) };
            assert_eq!(tmp, tmpc);
            assert_eq!(res, resc == 0);
        }
    }

    #[test]
    fn test_solve_ntru_intermediate() {
        let mut rng = rand::thread_rng();
        //for logn in 2..10 {
        let logn = 10;
        let depth = 1;
        let (profile, profilec) = get_profiles(logn);
        let gauss = get_gauss(logn);
        let mut tmp: [u32; 200000] = [0; 200000];
        let tmpc: [u32; 200000] = [0; 200000];
        let mut f: [i8; 1024] = [0; 1024];
        let mut g: [i8; 1024] = [0; 1024];
        let mut ctx = NtruPrngChacha8Context {
            d: [0; 40],
        };
        gauss_sample_poly(logn, &mut f, gauss, prng_chacha8_out, &mut ctx);
        gauss_sample_poly(logn, &mut g, gauss, prng_chacha8_out, &mut ctx);
        let resc = unsafe { solve_NTRU_intermediate_test(&profilec, logn as u32, f.as_ptr(), g.as_ptr(), depth as usize, tmpc.as_ptr()) };
        let res = solve_ntru_intermediate(&profile, logn, &f, &g, depth, &mut tmp);
        assert_eq!(tmp, tmpc);
        assert_eq!(res, resc == 0);
    }
    //}

    #[test]
    fn solve_ntru_test() {
        for logn in 10..11 {
            let (profile, profilec) = get_profiles(logn);
            let mut tmp: [u32; 50000] = [0; 50000];
            let tmpc: [u32; 50000] = [0; 50000];
            let mut f: [i8; 1024] = [0; 1024];
            let mut g: [i8; 1024] = [0; 1024];
            let mut ctx = NtruPrngChacha8Context {
                d: [0; 40],
            };
            gauss_sample_poly(logn, &mut f, &GAUSS_FALCON_1024, prng_chacha8_out, &mut ctx);
            gauss_sample_poly(logn, &mut g, &GAUSS_FALCON_1024, prng_chacha8_out, &mut ctx);
            let res = solve_ntru(&profile, logn, &f, &g, &mut tmp);
            let resc = unsafe { ntrugen_solve_NTRU(&profilec, logn as u32, f.as_ptr(), g.as_ptr(), tmpc.as_ptr()) };
            println!("res {}", res);
            println!("resc {}", resc);
            //assert_eq!(tmp, tmpc);
            assert_eq!(res, resc == 0);
        }
    }

    pub fn get_gauss(logn: usize) -> &'static [u16] {
        return if logn <= 8 {
            &GAUSS_FALCON_256
        } else if logn == 9 {
            &GAUSS_FALCON_512
        } else {
            &GAUSS_FALCON_1024
        };
    }

    pub fn get_profiles(logn: usize) -> (NtruProfile, NtruProfileC) {
        if logn <= 8 {
            return (FALCON_256, NtruProfileC {
                q: 12289,
                min_logn: 2,
                max_logn: 8,
                max_bl_small: [1, 1, 2, 3, 4, 8, 14, 27, 53, 104, 207],
                max_bl_large: [1, 2, 3, 6, 11, 21, 40, 78, 155, 308],
                word_win: [1, 1, 2, 2, 2, 3, 3, 4, 5, 7],
                reduce_bits: 16,
                coeff_FG_limit: [0, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127],
                min_save_fg: [0, 0, 1, 2, 2, 2, 2, 2, 2, 3, 3],
            });
        } else if logn == 9 {
            return (FALCON_512, NtruProfileC {
                q: 12289,
                min_logn: 9,
                max_logn: 9,
                max_bl_small: [1, 1, 2, 3, 4, 8, 14, 27, 53, 104, 207],
                max_bl_large: [1, 2, 3, 6, 11, 21, 40, 78, 155, 308],
                word_win: [1, 1, 2, 2, 2, 3, 3, 4, 5, 7],
                reduce_bits: 13,
                coeff_FG_limit: [0, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127],
                min_save_fg: [0, 0, 1, 2, 2, 2, 2, 2, 2, 3, 3],
            });
        } else {
            (FALCON_1024, NtruProfileC {
                q: 12289,
                min_logn: 10,
                max_logn: 10,
                max_bl_small: [1, 1, 2, 3, 4, 8, 14, 27, 53, 104, 207],
                max_bl_large: [1, 2, 3, 6, 11, 21, 40, 78, 155, 308],
                word_win: [1, 1, 2, 2, 2, 3, 3, 4, 5, 7],
                reduce_bits: 11,
                coeff_FG_limit: [0, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127],
                min_save_fg: [0, 0, 1, 2, 2, 2, 2, 2, 2, 3, 3],
            })
        }
    }
}