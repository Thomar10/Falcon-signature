#[cfg(test)]
mod tests {
    use rand::Rng;
    use crate::falcon_c::sign_c::{ffLDL_fft_inner_func as ffLDL_fft_inner_c, ffLDL_treesize_func as ffLDL_treesize_c};
    use crate::sign::{ffLDL_fft_inner, ffLDL_treesize};

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

    #[allow(non_camel_case_types)]
    type fpr = u64;
}