#![allow(non_snake_case)]

#[cfg(test)]
mod tests {
    use ntru_gen::falcon_ntru::{FALCON_1024, FALCON_256, FALCON_512};
    use ntru_gen::ntru::{make_fg_step, NtruProfile};
    use ntru_gen_c::ntru::{make_fg_step_test, NtruProfileC};

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
    fn solve_ntru_test() {
        assert_eq!(1, 1);
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