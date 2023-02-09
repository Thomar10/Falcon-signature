#[cfg(test)]
mod tests {
    use rand::Rng;
    use crate::falcon_c::sign_c::{ffLDL_fft_inner_func as ffLDL_fft_inner_c, ffLDL_treesize_func as ffLDL_treesize_c, ffLDL_fft_func as ffLDL_fft_c, ffLDL_binary_normalize_func as ffLDL_binary_normalize_c, smallints_to_fpr_func as smallints_to_fpr_c, skoff_b00_func as skoff_b00_c, skoff_b01_func as skoff_b01_c, skoff_b10_func as skoff_b10_c, skoff_b11_func as skoff_b11_c, skoff_tree_func as skoff_tree_c, falcon_inner_expand_privkey};
    use crate::sign::{expand_privkey, ffLDL_binary_normalize, ffLDL_fft, ffLDL_fft_inner, ffLDL_treesize, skoff_b00, skoff_b01, skoff_b10, skoff_b11, skoff_tree, smallints_to_fpr};

    #[allow(non_snake_case)]
    #[test]
    fn test_ffLDL_treesize() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let logn: u32 = rng.gen_range(0..16);

            let res_rust = ffLDL_treesize(logn);
            let res_c: u32;

            unsafe {
                res_c = ffLDL_treesize_c(logn);
            }

            assert_eq!(res_rust, res_c);
        }
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_ffLDL_fft_inner() {
        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            let mut tree: [fpr; 128] = core::array::from_fn(|_| rng.gen::<u64>());
            let mut g0: [fpr; 64] = core::array::from_fn(|_| rng.gen::<u64>());
            let mut g1: [fpr; 64] = core::array::from_fn(|_| rng.gen::<u64>());
            let logn: u32 = 4; //Must be a power of 2 and relates to the size of the tree somehow
            let mut tmp: [fpr; 64] = core::array::from_fn(|_| rng.gen::<u64>());

            let tree_c: [fpr; 128] = tree.clone();
            let g0_c: [fpr; 64] = g0.clone();
            let g1_c: [fpr; 64] = g1.clone();
            let tmp_c: [fpr; 64] = tmp.clone();

            ffLDL_fft_inner(&mut tree, &mut g0, &mut g1, logn, &mut tmp);

            unsafe {
                ffLDL_fft_inner_c(tree_c.as_ptr(), g0_c.as_ptr(), g1_c.as_ptr(), logn, tmp_c.as_ptr());
            }

            assert_eq!(tree, tree_c);
            assert_eq!(g0, g0_c);
            assert_eq!(g1, g1_c);
            assert_eq!(tmp, tmp_c);
        }
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_ffLDL_fft() {
        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            let mut tree: [fpr; 128] = core::array::from_fn(|_| rng.gen::<u64>());
            let mut g00: [fpr; 64] = core::array::from_fn(|_| rng.gen::<u64>());
            let mut g01: [fpr; 64] = core::array::from_fn(|_| rng.gen::<u64>());
            let mut g11: [fpr; 64] = core::array::from_fn(|_| rng.gen::<u64>());
            let logn: u32 = 4; //Must be a power of 2 and relates to the size of the tree somehow
            let mut tmp: [fpr; 64] = core::array::from_fn(|_| rng.gen::<u64>());

            let tree_c: [fpr; 128] = tree.clone();
            let g00_c: [fpr; 64] = g00.clone();
            let g01_c: [fpr; 64] = g01.clone();
            let g11_c: [fpr; 64] = g11.clone();
            let tmp_c: [fpr; 64] = tmp.clone();

            ffLDL_fft(&mut tree, &mut g00, &mut g01, &mut g11, logn, &mut tmp);

            unsafe {
                ffLDL_fft_c(tree_c.as_ptr(), g00_c.as_ptr(), g01_c.as_ptr(), g11_c.as_ptr(), logn, tmp_c.as_ptr());
            }

            assert_eq!(tree, tree_c);
            assert_eq!(g00, g00_c);
            assert_eq!(g01, g01_c);
            assert_eq!(g11, g11_c);
            assert_eq!(tmp, tmp_c);
        }
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_ffLDL_binary_normalize() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let mut tree: [fpr; 128] = core::array::from_fn(|_| rng.gen::<u64>());
            let logn: u32 = 4; //Must be a power of 2 and relates to the size of the tree somehow

            let tree_c: [fpr; 128] = tree.clone();

            ffLDL_binary_normalize(&mut tree, logn, logn);

            unsafe {
                ffLDL_binary_normalize_c(tree_c.as_ptr(), logn, logn);
            }

            assert_eq!(tree, tree_c);
        }
    }

    #[test]
    fn test_smallints_to_fpr() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let mut r: [fpr; 256] = [0; 256];
            let r_c: [fpr; 256] = [0; 256];

            let t: [i8; 256] = core::array::from_fn(|_| rng.gen::<i8>());
            let t_c: [i8; 256] = t.clone();

            smallints_to_fpr(&mut r, &t, 8);

            assert_ne!(r, r_c);

            unsafe {
                smallints_to_fpr_c(r_c.as_ptr(), t_c.as_ptr(), 8);
                assert_eq!(r, r_c)
            }
        }
    }

    #[test]
    fn test_skoff_b00() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let logn: u32 = rng.gen_range(0..16);

            let res_rust: usize = skoff_b00(logn);
            let res_c: usize;

            unsafe {
                res_c = skoff_b00_c(logn)
            }

            assert_eq!(res_rust, res_c);
        }
    }

    #[test]
    fn test_skoff_b01() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let logn: u32 = rng.gen_range(0..16);

            let res_rust: usize = skoff_b01(logn);
            let res_c: usize;

            unsafe {
                res_c = skoff_b01_c(logn)
            }

            assert_eq!(res_rust, res_c);
        }
    }

    #[test]
    fn test_skoff_b10() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let logn: u32 = rng.gen_range(0..16);

            let res_rust: usize = skoff_b10(logn);
            let res_c: usize;

            unsafe {
                res_c = skoff_b10_c(logn)
            }

            assert_eq!(res_rust, res_c);
        }
    }

    #[test]
    fn test_skoff_b11() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let logn: u32 = rng.gen_range(0..16);

            let res_rust: usize = skoff_b11(logn);
            let res_c: usize;

            unsafe {
                res_c = skoff_b11_c(logn)
            }

            assert_eq!(res_rust, res_c);
        }
    }

    #[test]
    fn test_skoff_tree() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let logn: u32 = rng.gen_range(0..16);

            let res_rust: usize = skoff_tree(logn);
            let res_c: usize;

            unsafe {
                res_c = skoff_tree_c(logn)
            }

            assert_eq!(res_rust, res_c);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_expand_privkey() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            const LOGN: usize = 8;
            const N: usize = 1 << LOGN;
            let mut expanded_key: [fpr; 4096] = [0; 4096]; //core::array::from_fn(|_| rng.gen::<u64>());
            let mut f: [i8; 256] = core::array::from_fn(|_| rng.gen::<i8>());
            let mut g: [i8; 256] = core::array::from_fn(|_| rng.gen::<i8>());
            let mut F: [i8; 256] = core::array::from_fn(|_| rng.gen::<i8>());
            let mut G: [i8; 256] = core::array::from_fn(|_| rng.gen::<i8>());
            let mut tmp: [fpr; N * 16] = [0; N * 16];

            let expanded_key_c = expanded_key.clone();
            let f_c = f.clone();
            let g_c = g.clone();
            let F_c = F.clone();
            let G_c = G.clone();
            let tmp_c: [i8; N * 16 * 8] = [0; N * 16 * 8];

            expand_privkey(&mut expanded_key, &mut f, &mut g, &mut F, &mut G, LOGN as u32, &mut tmp);

            assert_ne!(expanded_key, expanded_key_c);

            unsafe {
                falcon_inner_expand_privkey(expanded_key_c.as_ptr(), f_c.as_ptr(), g_c.as_ptr(), F_c.as_ptr(), G_c.as_ptr(), LOGN as u32, tmp_c.as_ptr() as *const u8);
            }

            assert_eq!(expanded_key, expanded_key_c)
        }
    }

    #[allow(non_camel_case_types)]
    type fpr = u64;
}