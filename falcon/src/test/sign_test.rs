#[cfg(test)]
mod tests {
    use std::ffi::c_void;
    use rand::Rng;
    use crate::falcon_c::sign_c::{ffLDL_fft_inner_func as ffLDL_fft_inner_c, ffLDL_treesize_func as ffLDL_treesize_c, ffLDL_fft_func as ffLDL_fft_c, ffLDL_binary_normalize_func as ffLDL_binary_normalize_c, smallints_to_fpr_func as smallints_to_fpr_c, skoff_b00_func as skoff_b00_c, skoff_b01_func as skoff_b01_c, skoff_b10_func as skoff_b10_c, skoff_b11_func as skoff_b11_c, skoff_tree_func as skoff_tree_c, falcon_inner_expand_privkey, falcon_inner_gaussian0_sampler as gaussian0_sampler_c, BerExp_func as BerExpC, SamplerContext as SamplerContextC, falcon_inner_sampler as sampler_c, ffSampling_fft_dyntree_func as ffSampling_fft_dyntree_c};
    use crate::falcon_c::rng_c::{Prng as PrngC};
    use crate::fpr::{fpr_add, fpr_div, fpr_half, FPR_INV_SIGMA, fpr_of, FPR_SIGMA_MIN};
    use crate::rng::Prng;
    use crate::sign::{expand_privkey,
                      ffLDL_binary_normalize,
                      ffLDL_fft, ffLDL_fft_inner,
                      ffLDL_treesize, gaussian0_sampler,
                      skoff_b00, skoff_b01, skoff_b10, skoff_b11, skoff_tree, smallints_to_fpr, BerExp, SamplerContext, sampler, ffSampling_fft_dyntree};
    use crate::test::rng_test::tests::{create_random_prngs, init_prngs};

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

    #[test]
    fn test_gaussion0_sampler() {
        for _ in 0..100 {
            let (mut prng, prng_c): (Prng, PrngC) = create_random_prngs();
            init_prngs(&mut prng, &prng_c);

            for _ in 0..20 {
                let output_rust:i32 = gaussian0_sampler(&mut prng);
                let output_c: i32;

                unsafe {
                    output_c = gaussian0_sampler_c(&prng_c);
                }

                assert_eq!(output_rust, output_c);
            }
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_BerExp() {
        for _ in 0..100 {
            let (mut prng, prng_c): (Prng, PrngC) = create_random_prngs();
            init_prngs(&mut prng, &prng_c);

            let x: fpr = rand::random();
            let ccs: fpr = rand::random();

            for _ in 0..20 {
                let output_rust:i32 = BerExp(&mut prng, x, ccs);
                let output_c: i32;

                unsafe {
                    output_c = BerExpC(&prng_c, x, ccs);
                }

                assert_eq!(output_rust, output_c);
            }
        }
    }

    #[test] //Sometimes loops forever
    fn test_sampler() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let (mut prng, prng_c): (Prng, PrngC) = create_random_prngs();
            init_prngs(&mut prng, &prng_c);

            let logn = rng.gen_range(1..8);
            let sigma_min: fpr = FPR_SIGMA_MIN[logn]; //fpr 1
            let isigma = FPR_INV_SIGMA[logn];
            let mu: fpr = rand::random();

            let mut samp_ctx: SamplerContext = SamplerContext{p: prng, sigma_min};
            let samp_ctx_c: SamplerContextC = SamplerContextC{p: prng_c, sigma_min};

            let res_rust: i32 = sampler(&mut samp_ctx, mu, isigma);
            let res_c: i32;

            unsafe {
                res_c = sampler_c(&samp_ctx_c, mu, isigma);
            }

            assert_eq!(res_rust, res_c);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_ffSampling_fft_dyntree() {
        let mut rng = rand::thread_rng();
        for _ in 0..2 {
            let (mut prng, prng_c): (Prng, PrngC) = create_random_prngs();
            init_prngs(&mut prng, &prng_c);

            const LOGN: usize = 8;
            const N: usize = 1 << LOGN;

            let sigma_min: fpr = FPR_SIGMA_MIN[LOGN];

            let mut samp_ctx: SamplerContext = SamplerContext{p: prng, sigma_min};
            let mut samp_ctx_c: SamplerContextC = SamplerContextC{p: prng_c, sigma_min};


            let mut t0: [fpr; N] = gen_small_fpr_array(N).try_into().unwrap(); //[0; N]; // = core::array::from_fn(|_| rng.gen::<u64>());
            let mut t1: [fpr; N] = gen_small_fpr_array(N).try_into().unwrap(); // = core::array::from_fn(|_| rng.gen::<u64>());
            let mut g00: [fpr; N] = gen_small_fpr_array(N).try_into().unwrap(); // = core::array::from_fn(|_| rng.gen::<u64>());
            let mut g01: [fpr; N] = gen_small_fpr_array(N).try_into().unwrap(); // = core::array::from_fn(|_| rng.gen::<u64>());
            let mut g11: [fpr; N] = gen_small_fpr_array(N).try_into().unwrap(); // = core::array::from_fn(|_| rng.gen::<u64>());

            /*let mut small_int_arr: [i8; N] = core::array::from_fn(|_| rng.gen::<i8>());
            smallints_to_fpr(&mut t0, &small_int_arr, LOGN as u32);
            small_int_arr = core::array::from_fn(|_| rng.gen::<i8>());
            smallints_to_fpr(&mut t1, &small_int_arr, LOGN as u32);
            small_int_arr = core::array::from_fn(|_| rng.gen::<i8>());
            smallints_to_fpr(&mut g00, &small_int_arr, LOGN as u32);
            small_int_arr = core::array::from_fn(|_| rng.gen::<i8>());
            smallints_to_fpr(&mut g01, &small_int_arr, LOGN as u32);
            small_int_arr = core::array::from_fn(|_| rng.gen::<i8>());
            smallints_to_fpr(&mut g11, &small_int_arr, LOGN as u32);
            small_int_arr = core::array::from_fn(|_| rng.gen::<i8>());*/



            let orig_logn: usize = LOGN;
            let mut tmp: [fpr; 32 * N] = [0; 32 * N];

            let t0_c: [fpr; N] = t0.clone();
            let t1_c: [fpr; N] = t0.clone();
            let g00_c: [fpr; N] = t0.clone();
            let g01_c: [fpr; N] = t0.clone();
            let g11_c: [fpr; N] = t0.clone();
            let tmp_c: [fpr; 32 * N] = [0; 32 * N];

            /*ffSampling_fft_dyntree(sampler, &mut samp_ctx, &mut t0, &mut t1, &mut g00,
                                   &mut g01, &mut g11, orig_logn as u32,
                                   orig_logn as u32, &mut tmp);*/

            unsafe {
                ffSampling_fft_dyntree_c(sampler_c, &mut samp_ctx_c as *mut _ as *const c_void,
                                         t0_c.as_ptr(), t1_c.as_ptr(), g00_c.as_ptr(),
                                         g01_c.as_ptr(), g11_c.as_ptr(), orig_logn as u32,
                                         orig_logn as u32, tmp_c.as_ptr());
            }

            assert_eq!(tmp, tmp_c);
        }
    }

    fn gen_small_fpr_array(size: usize) -> Vec<fpr> {
        let mut rng = rand::thread_rng();
        let mut vec: Vec<fpr> = Vec::with_capacity(size);
        vec.resize(size, 0);

        for i in 0..size {
            let randi8: i8 = rng.gen_range(1..=126);
            let mut fpr: fpr = fpr_of(randi8 as i64);
            fpr = fpr_of(1) + fpr_div(fpr, fpr_of(127));
            vec[i] = fpr;
        }

        return vec;
    }

    #[allow(non_camel_case_types)]
    type fpr = u64;
}