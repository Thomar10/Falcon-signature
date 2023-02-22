#[cfg(test)]
mod tests {
    use std::ffi::c_void;

    use rand::Rng;

    use crate::common::hash_to_point_vartime;
    use crate::falcon_c::keygen_c::falcon_inner_keygen;
    use crate::falcon_c::rng_c::Prng as PrngC;
    use crate::falcon_c::shake_c::{InnerShake256Context as InnerShake256ContextC};
    use crate::falcon_c::sign_c::{BerExp_func as BerExpC, falcon_inner_expand_privkey, falcon_inner_gaussian0_sampler as gaussian0_sampler_c, falcon_inner_sampler as sampler_c, falcon_inner_sign_dyn, falcon_inner_sign_tree, ffLDL_binary_normalize_func as ffLDL_binary_normalize_c, ffLDL_fft_func as ffLDL_fft_c, ffLDL_fft_inner_func as ffLDL_fft_inner_c, ffLDL_treesize_func as ffLDL_treesize_c, ffSampling_fft_dyntree_func as ffSampling_fft_dyntree_c, SamplerContext as SamplerContextC, skoff_b00_func as skoff_b00_c, skoff_b01_func as skoff_b01_c, skoff_b10_func as skoff_b10_c, skoff_b11_func as skoff_b11_c, skoff_tree_func as skoff_tree_c, smallints_to_fpr_func as smallints_to_fpr_c};
    use crate::fpr::{fpr_div, fpr_of, FPR_SIGMA_MIN};
    use crate::keygen::keygen;
    use crate::rng::Prng;
    use crate::shake::{i_shake256_extract, i_shake256_flip, i_shake256_inject};
    use crate::sign::{BerExp, expand_privkey, ffLDL_binary_normalize, ffLDL_fft, ffLDL_fft_inner, ffLDL_treesize, ffSampling_fft_dyntree, gaussian0_sampler, sampler, SamplerContext, sign_dyn, sign_tree, skoff_b00, skoff_b01, skoff_b10, skoff_b11, skoff_tree, smallints_to_fpr};
    use crate::tests::keygen_test::tests::init_shake_with_random_context;
    use crate::tests::rng_test::tests::{create_random_prngs, init_prngs};

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

    #[test]
    fn test_sampler() {
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let (mut prng, prng_c): (Prng, PrngC) = create_random_prngs();
            init_prngs(&mut prng, &prng_c);

            let logn = rng.gen_range(1..8);
            let sigma_min: fpr = FPR_SIGMA_MIN[logn];
            let isigma = fpr_from_fraction(rng.gen_range(50..100), 100); //Should be between 0.5 and 1
            let mu: fpr = fpr_of(rng.gen_range(1..10));

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

    //TODO fix test
    #[test]
    #[ignore]
    #[allow(non_snake_case)]
    fn test_ffSampling_fft_dyntree() {
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

            ffSampling_fft_dyntree(sampler, &mut samp_ctx, &mut t0, &mut t1, &mut g00,
                                   &mut g01, &mut g11, orig_logn as u32,
                                   orig_logn as u32, &mut tmp);

            unsafe {
                ffSampling_fft_dyntree_c(sampler_c, &mut samp_ctx_c as *mut _ as *const c_void,
                                         t0_c.as_ptr(), t1_c.as_ptr(), g00_c.as_ptr(),
                                         g01_c.as_ptr(), g11_c.as_ptr(), orig_logn as u32,
                                         orig_logn as u32, tmp_c.as_ptr());
            }

            assert_eq!(tmp, tmp_c);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_sign_dyn() {
        for _ in 0..100 {

            const LOGN: usize = 10;

            const BUFFER_SIZE: usize = 8192 * 4 * 8;
            let (mut rng_rust, _) = init_shake_with_random_context();

            let mut sig: [i16; 1024] = [0; 1024];
            let sig_c: [i16; 1024] = [0; 1024];
            let mut h: [u16; 1024] = [0; 1024];
            let h_c: [u16; 1024] = [0; 1024];
            let mut f: [i8; 1024] = [0; 1024];
            let f_c: [i8; 1024] = [0; 1024];
            let mut g: [i8; 1024] = [0; 1024];
            let g_c: [i8; 1024] = [0; 1024];
            let mut F: [i8; 1024] = [0; 1024];
            let F_c: [i8; 1024] = [0; 1024];
            let mut G: [i8; 1024] = [0; 1024];
            let G_c: [i8; 1024] = [0; 1024];

            let mut tmp: Vec<u8> = vec![0; BUFFER_SIZE];
            let tmp_c: Vec<u8> = vec![0; BUFFER_SIZE];

            let mut hm: [u16; 1024] = [0; 1024];

            let msg: [u8; 128] = <[u8; 128]>::try_from(i_shake256_extract(&mut rng_rust, 128)).unwrap();

            let (mut rng_rust, rng_c) = init_shake_with_random_context();

            let (mut sc, _) = init_shake_with_random_context();
            i_shake256_inject(&mut sc, &msg);
            i_shake256_flip(&mut sc);
            hash_to_point_vartime(&mut sc, &mut hm, LOGN as u32);

            keygen(&mut rng_rust, f.as_mut_ptr(), g.as_mut_ptr(), F.as_mut_ptr(), G.as_mut_ptr(), h.as_mut_ptr(), LOGN as u32, tmp.as_mut_ptr());

            unsafe { falcon_inner_keygen(&rng_c, f_c.as_ptr(), g_c.as_ptr(), F_c.as_ptr(), G_c.as_ptr(), h_c.as_ptr(), LOGN as u32, tmp_c.as_ptr()); }

            let hm_c: [u16; 1024] = hm.clone();

            sign_dyn(&mut sig, &mut rng_rust, &f, &g, &F, &G, &mut hm, LOGN as u32, &mut tmp);

            unsafe {
                falcon_inner_sign_dyn(sig_c.as_ptr(), &rng_c as *const InnerShake256ContextC, f_c.as_ptr(), g_c.as_ptr(), F_c.as_ptr(), G_c.as_ptr(), hm_c.as_ptr(), LOGN as u32, tmp_c.as_ptr())
            }

            assert_eq!(tmp, tmp_c);
            assert_eq!(sig, sig_c);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_sign_tree() {
        for _ in 0..100 {
            const LOGN: usize = 4;
            const N: usize = 1 << LOGN;

            const BUFFER_SIZE: usize = 8192 * 8;
            let mut tmp: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
            let tmp_c: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

            let mut tmp_key: [fpr; N * 16] = [0; N * 16];

            let (mut rng_rust, _) = init_shake_with_random_context();

            let mut hm: [u16; 1024] = [0; 1024];

            let msg: [u8; 128] = <[u8; 128]>::try_from(i_shake256_extract(&mut rng_rust, 128)).unwrap();

            let (mut rng_rust, _) = init_shake_with_random_context();

            let (mut sc, _) = init_shake_with_random_context();
            i_shake256_inject(&mut sc, &msg);
            i_shake256_flip(&mut sc);
            hash_to_point_vartime(&mut sc, &mut hm, LOGN as u32);

            let mut sig: [i16; 1024] = [0; 1024];
            let sig_c: [i16; 1024] = [0; 1024];

            let mut h: [u16; 1024] = [0; 1024];
            let mut f: [i8; 1024] = [0; 1024];
            let mut g: [i8; 1024] = [0; 1024];
            let mut F: [i8; 1024] = [0; 1024];
            let mut G: [i8; 1024] = [0; 1024];

            let mut tmp_keygen: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
            keygen(&mut rng_rust, f.as_mut_ptr(), g.as_mut_ptr(), F.as_mut_ptr(), G.as_mut_ptr(), h.as_mut_ptr(), LOGN as u32, tmp_keygen.as_mut_ptr());

            const EXKLENGTH: usize = (LOGN + 40) << LOGN;
            let mut expanded_key: [fpr; EXKLENGTH] = [0; EXKLENGTH];

            expand_privkey(&mut expanded_key, &mut f, &mut g, &mut F, &mut G, LOGN as u32, &mut tmp_key);

            //let mut expanded_key: [fpr; EXKLENGTH - 1] = [0; EXKLENGTH - 1];
            //expanded_key.clone_from(<&[fpr; 703]>::try_from(&expanded_key_extra[1..]).unwrap());

            let expanded_key_c: [fpr; EXKLENGTH] = expanded_key.clone();
            let hm_c: [u16; 1024] = hm.clone();

            let (mut rng_rust, rng_c) = init_shake_with_random_context();

            sign_tree(&mut sig, &mut rng_rust, &mut expanded_key, &hm, LOGN as u32, &mut tmp);

            unsafe {
                falcon_inner_sign_tree(sig_c.as_ptr(), &rng_c as *const InnerShake256ContextC, expanded_key_c.as_ptr(), hm_c.as_ptr(), LOGN as u32, tmp_c.as_ptr());
            }

            assert_eq!(tmp, tmp_c);
            assert_eq!(sig, sig_c);
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

    fn fpr_from_fraction(top: i64, bot: i64) -> fpr {
        let numerator: fpr = fpr_of(top);
        let denominator: fpr = fpr_of(bot);

        return fpr_div(numerator, denominator);
    }

    #[allow(non_camel_case_types)]
    type fpr = u64;
}