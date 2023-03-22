#![allow(non_snake_case)]

#[cfg(test)]
mod tests {
    use rand::Rng;

    use ntru_gen::falcon_ntru::{FALCON_1024, FALCON_256, FALCON_512};
    use ntru_gen::ntru::{make_fg_step, NtruProfile, solve_ntru, solve_ntru_deepest, solve_ntru_depth0, solve_ntru_intermediate};
    use ntru_gen_c::ntru::{make_fg_step_test, ntrugen_solve_NTRU, NtruProfileC, solve_NTRU_deepest_test, solve_NTRU_depth0_test, solve_NTRU_intermediate_test};

// #[test]
    // fn test_make_fg_step() {
    //     for logn in 8..11 {
    //         for depth in 0..5 {
    //             let (profile, profilec) = get_profiles(logn);
    //             let mut tmp: [u32; 50000] = [0; 50000];
    //             let tmpc: [u32; 50000] = [0; 50000];
    //             make_fg_step(&profile, logn + depth, depth as u32, &mut tmp);
    //             unsafe { make_fg_step_test(&profilec, (logn + depth) as u32, depth as u32, tmpc.as_ptr()); }
    //             assert_eq!(tmp, tmpc);
    //         }
    //     }
    // }
    //
    // #[test]
    // fn test_solve_ntru_depth0() {
    //     for logn in 8..11 {
    //         let (profile, profilec) = get_profiles(logn);
    //         let mut tmp: [u32; 50000] = [0; 50000];
    //         let tmpc: [u32; 50000] = [0; 50000];
    //         let f: [i8; 1024] = [0; 1024];
    //         let g: [i8; 1024] = [0; 1024];
    //         let res = solve_ntru_depth0(&profile, logn, &f, &g, &mut tmp);
    //         let resc = unsafe { solve_NTRU_depth0_test(&profilec, logn as u32, f.as_ptr(), g.as_ptr(), tmpc.as_ptr()) };
    //         assert_eq!(tmp, tmpc);
    //         assert_eq!(res, resc == 0);
    //     }
    // }
    //
    // #[test]
    // fn test_solve_ntru_deepest() {
    //     for logn in 8..11 {
    //         let (profile, profilec) = get_profiles(logn);
    //         let mut tmp: [u32; 50000] = [0; 50000];
    //         let tmpc: [u32; 50000] = [0; 50000];
    //         let f: [i8; 1024] = [0; 1024];
    //         let g: [i8; 1024] = [0; 1024];
    //         let res = solve_ntru_deepest(&profile, logn, &f, &g, &mut tmp);
    //         let resc = unsafe { solve_NTRU_deepest_test(&profilec, logn as u32, f.as_ptr(), g.as_ptr(), tmpc.as_ptr()) };
    //         assert_eq!(tmp, tmpc);
    //         assert_eq!(res, resc == 0);
    //     }
    // }

    #[test]
    fn test_solve_ntru_intermediate() {
        let mut rng = rand::thread_rng();
        for logn in 3..6 {
            for depth in 1..5 {
                let (profile, profilec) = get_profiles(logn);
                let mut tmp: [u32; 5000] = core::array::from_fn(|_| rng.gen::<u32>());
                let tmpc: [u32; 5000] = tmp.clone();
                let f: [i8; 1024] = [0; 1024];
                let g: [i8; 1024] = [0; 1024];
                let resc = unsafe { solve_NTRU_intermediate_test(&profilec, (logn + depth) as u32, f.as_ptr(), g.as_ptr(), depth as usize, tmpc.as_ptr()) };
                let res = solve_ntru_intermediate(&profile, logn + depth, &f, &g, depth, &mut tmp);
                assert_eq!(tmp, tmpc);
                assert_eq!(res, resc == 0);
            }
        }
    }

    #[test]
    fn solve_ntru_test() {
        for logn in 2..11 {
            let (profile, profilec) = get_profiles(logn);
            let mut tmp: [u32; 50000] = [0; 50000];
            let tmpc: [u32; 50000] = [0; 50000];
            let f: [i8; 1024] = [0; 1024];
            let g: [i8; 1024] = [0; 1024];
            let res = solve_ntru(&profile, logn, &f, &g, &mut tmp);
            let resc = unsafe { ntrugen_solve_NTRU(&profilec, logn  as u32, f.as_ptr(), g.as_ptr(), tmpc.as_ptr()) };
            assert_eq!(tmp, tmpc);
            assert_eq!(res, resc == 0);
        }
    }

    pub fn get_profiles(logn: usize) -> (NtruProfile, NtruProfileC) {
        if logn == 8 {
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