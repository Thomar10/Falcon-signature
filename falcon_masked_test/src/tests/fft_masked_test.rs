#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;
    use rand::{Rng, thread_rng};
    use falcon::falcon::fpr;
    use falcon::fpr::{fpr_of as u_fpr_of, fpr_sub as u_fpr_sub, fpr_add as u_fpr_add};
    use falcon_masked::fft_masked::{fft, fpc_add, fpc_mul, fpc_sub, ifft, poly_add, poly_adj_fft, poly_div_fft, poly_invnorm2_fft, poly_mul_fft, poly_muladj_fft, poly_mulconst, poly_mulselfadj_fft, poly_neg, poly_sub};
    use falcon::fft::{poly_invnorm2_fft as u_poly_invnorm2_fft, poly_div_fft as u_poly_div_fft, poly_mulconst as u_poly_mulconst, poly_mulselfadj_fft as u_poly_mulselfadj_fft, poly_muladj_fft as u_poly_muladj_fft, poly_mul_fft as u_poly_mul_fft, poly_adj_fft as u_poly_adj_fft, fpc_add as u_fpc_add, fpc_mul as u_fpc_mul, fpc_sub as u_fpc_sub, fft as u_fft, ifft as u_ifft, poly_add as u_poly_add, poly_sub as u_poly_sub, poly_neg as u_poly_neg};

    #[test]
    fn test_fpc_add() {
        for _ in 0..100 {
            let (a_re, shares_a_re): (fpr, [fpr; 2]) = create_random_mask();
            let (a_im, shares_a_im): (fpr, [fpr; 2]) = create_random_mask();
            let (b_re, shares_b_re): (fpr, [fpr; 2]) = create_random_mask();
            let (b_im, shares_b_im): (fpr, [fpr; 2]) = create_random_mask();

            let (shares_res_re, shares_res_im) = fpc_add(&shares_a_re, &shares_a_im, &shares_b_re, &shares_b_im);
            let (res_re, res_im) = u_fpc_add(a_re, a_im, b_re, b_im);

            check_fpr_eq(reconstruct(&shares_res_re), res_re);
            check_fpr_eq(reconstruct(&shares_res_im), res_im);
        }
    }

    #[test]
    fn test_fpc_mul() {
        for _ in 0..100 {
            let (a_re, shares_a_re): (fpr, [fpr; 2]) = create_random_mask();
            let (a_im, shares_a_im): (fpr, [fpr; 2]) = create_random_mask();
            let (b_re, shares_b_re): (fpr, [fpr; 2]) = create_random_mask();
            let (b_im, shares_b_im): (fpr, [fpr; 2]) = create_random_mask();

            let (shares_res_re, shares_res_im) = fpc_mul(&shares_a_re, &shares_a_im, &shares_b_re, &shares_b_im);
            let (res_re, res_im) = u_fpc_mul(a_re, a_im, b_re, b_im);

            check_fpr_eq(reconstruct(&shares_res_re), res_re);
            check_fpr_eq(reconstruct(&shares_res_im), res_im);
        }
    }

    #[test]
    fn test_fpc_sub() {
        for _ in 0..100 {
            let (a_re, shares_a_re): (fpr, [fpr; 2]) = create_random_mask();
            let (a_im, shares_a_im): (fpr, [fpr; 2]) = create_random_mask();
            let (b_re, shares_b_re): (fpr, [fpr; 2]) = create_random_mask();
            let (b_im, shares_b_im): (fpr, [fpr; 2]) = create_random_mask();

            let (shares_res_re, shares_res_im) = fpc_sub(&shares_a_re, &shares_a_im, &shares_b_re, &shares_b_im);
            let (res_re, res_im) = u_fpc_sub(a_re, a_im, b_re, b_im);

            check_fpr_eq(reconstruct(&shares_res_re), res_re);
            check_fpr_eq(reconstruct(&shares_res_im), res_im);
        }
    }

    #[test]
    fn test_fft() {
        const LOGN: u32 = 10;
        const LENGTH: usize = 1 << LOGN;
        for _ in 0..100 {
            let (mut f, mut f_shares): ([fpr; LENGTH], [[fpr; 2]; LENGTH]) = create_random_mask_arr::<LENGTH>();

            fft(&mut f_shares, LOGN);
            u_fft(&mut f, LOGN);

            //Reconstruct
            let f_reconstructed: [fpr; LENGTH] = reconstruct_arr(&f_shares);

            check_fpr_arr_eq(&f_reconstructed, &f);
        }
    }

    #[test]
    fn test_ifft() {
        const LOGN: u32 = 10;
        const LENGTH: usize = 1 << LOGN;
        for _ in 0..100 {
            let (mut f, mut f_shares): ([fpr; LENGTH], [[fpr; 2]; LENGTH]) = create_random_mask_arr::<LENGTH>();

            ifft(&mut f_shares, LOGN);
            u_ifft(&mut f, LOGN);

            //Reconstruct
            let f_reconstructed: [fpr; LENGTH] = reconstruct_arr(&f_shares);

            check_fpr_arr_eq(&f_reconstructed, &f);
        }
    }

    #[test]
    fn test_poly_add() {
        const LOGN: u32 = 10;
        const LENGTH: usize = 1 << LOGN;
        for _ in 0..100 {
            let (mut a, mut a_shares): ([fpr; LENGTH], [[fpr; 2]; LENGTH]) = create_random_mask_arr::<LENGTH>();
            let (mut b, mut b_shares): ([fpr; LENGTH], [[fpr; 2]; LENGTH]) = create_random_mask_arr::<LENGTH>();

            u_poly_add(&mut a, &b, LOGN);
            poly_add(&mut a_shares, &b_shares, LOGN);

            let a_reconstructed: [fpr; LENGTH] = reconstruct_arr(&a_shares);
            let b_reconstructed: [fpr; LENGTH] = reconstruct_arr(&b_shares);

            check_fpr_arr_eq(&a, &a_reconstructed);
            check_fpr_arr_eq(&b, &b_reconstructed);
        }
    }

    #[test]
    fn test_poly_sub() {
        const LOGN: u32 = 10;
        const LENGTH: usize = 1 << LOGN;
        for _ in 0..100 {
            let (mut a, mut a_shares): ([fpr; LENGTH], [[fpr; 2]; LENGTH]) = create_random_mask_arr::<LENGTH>();
            let (mut b, mut b_shares): ([fpr; LENGTH], [[fpr; 2]; LENGTH]) = create_random_mask_arr::<LENGTH>();

            u_poly_sub(&mut a, &b, LOGN);
            poly_sub(&mut a_shares, &b_shares, LOGN);

            let a_reconstructed: [fpr; LENGTH] = reconstruct_arr(&a_shares);
            let b_reconstructed: [fpr; LENGTH] = reconstruct_arr(&b_shares);

            check_fpr_arr_eq(&a, &a_reconstructed);
            check_fpr_arr_eq(&b, &b_reconstructed);
        }
    }

    #[test]
    fn test_poly_neg() {
        const LOGN: u32 = 10;
        const LENGTH: usize = 1 << LOGN;
        for _ in 0..100 {
            let (mut a, mut a_shares): ([fpr; LENGTH], [[fpr; 2]; LENGTH]) = create_random_mask_arr::<LENGTH>();

            u_poly_neg(&mut a, LOGN);
            poly_neg(&mut a_shares, LOGN);

            let a_reconstructed: [fpr; LENGTH] = reconstruct_arr(&a_shares);

            check_fpr_arr_eq(&a, &a_reconstructed);
        }
    }

    #[test]
    fn test_poly_adj_fft() {
        const LOGN: u32 = 10;
        const LENGTH: usize = 1 << LOGN;
        for _ in 0..100 {
            let (mut a, mut a_shares): ([fpr; LENGTH], [[fpr; 2]; LENGTH]) = create_random_mask_arr::<LENGTH>();

            u_poly_adj_fft(&mut a, LOGN);
            poly_adj_fft(&mut a_shares, LOGN);

            let a_reconstructed: [fpr; LENGTH] = reconstruct_arr(&a_shares);

            check_fpr_arr_eq(&a, &a_reconstructed);
        }
    }

    #[test]
    fn test_poly_mul_fft() {
        const LOGN: u32 = 10;
        const LENGTH: usize = 1 << LOGN;
        for _ in 0..100 {
            let (mut a, mut a_shares): ([fpr; LENGTH], [[fpr; 2]; LENGTH]) = create_random_mask_arr::<LENGTH>();
            let (mut b, mut b_shares): ([fpr; LENGTH], [[fpr; 2]; LENGTH]) = create_random_mask_arr::<LENGTH>();

            u_poly_mul_fft(&mut a, &b, LOGN);
            poly_mul_fft(&mut a_shares, &b_shares, LOGN);

            let a_reconstructed: [fpr; LENGTH] = reconstruct_arr(&a_shares);
            let b_reconstructed: [fpr; LENGTH] = reconstruct_arr(&b_shares);

            check_fpr_arr_eq(&a, &a_reconstructed);
            check_fpr_arr_eq(&b, &b_reconstructed);
        }
    }

    #[test]
    fn test_poly_muladj_fft() {
        const LOGN: u32 = 10;
        const LENGTH: usize = 1 << LOGN;
        for _ in 0..100 {
            let (mut a, mut a_shares): ([fpr; LENGTH], [[fpr; 2]; LENGTH]) = create_random_mask_arr::<LENGTH>();
            let (mut b, mut b_shares): ([fpr; LENGTH], [[fpr; 2]; LENGTH]) = create_random_mask_arr::<LENGTH>();

            u_poly_muladj_fft(&mut a, &b, LOGN);
            poly_muladj_fft(&mut a_shares, &b_shares, LOGN);

            let a_reconstructed: [fpr; LENGTH] = reconstruct_arr(&a_shares);
            let b_reconstructed: [fpr; LENGTH] = reconstruct_arr(&b_shares);

            check_fpr_arr_eq(&a, &a_reconstructed);
            check_fpr_arr_eq(&b, &b_reconstructed);
        }
    }

    #[test]
    fn test_poly_mulselfadj_fft() {
        const LOGN: u32 = 10;
        const LENGTH: usize = 1 << LOGN;
        for _ in 0..100 {
            let (mut a, mut a_shares): ([fpr; LENGTH], [[fpr; 2]; LENGTH]) = create_random_mask_arr::<LENGTH>();

            u_poly_mulselfadj_fft(&mut a, LOGN);
            poly_mulselfadj_fft(&mut a_shares, LOGN);

            let a_reconstructed: [fpr; LENGTH] = reconstruct_arr(&a_shares);

            check_fpr_arr_eq(&a, &a_reconstructed);
        }
    }

    #[test]
    fn test_poly_mulconst() {
        const LOGN: u32 = 10;
        const LENGTH: usize = 1 << LOGN;
        for _ in 0..100 {
            let (mut a, mut a_shares): ([fpr; LENGTH], [[fpr; 2]; LENGTH]) = create_random_mask_arr::<LENGTH>();
            let c: fpr = create_random_fpr();

            u_poly_mulconst(&mut a, c, LOGN);
            poly_mulconst(&mut a_shares, c, LOGN);

            let a_reconstructed: [fpr; LENGTH] = reconstruct_arr(&a_shares);

            check_fpr_arr_eq(&a, &a_reconstructed);
        }
    }

    #[test]
    fn test_poly_div_fft() {
        const LOGN: u32 = 10;
        const LENGTH: usize = 1 << LOGN;
        for _ in 0..100 {
            let (mut a, mut a_shares): ([fpr; LENGTH], [[fpr; 2]; LENGTH]) = create_random_mask_arr::<LENGTH>();
            let (mut b, mut b_shares): ([fpr; LENGTH], [[fpr; 2]; LENGTH]) = create_random_mask_arr::<LENGTH>();

            u_poly_div_fft(&mut a, &b, LOGN);
            poly_div_fft(&mut a_shares, &b_shares, LOGN);

            let a_reconstructed: [fpr; LENGTH] = reconstruct_arr(&a_shares);
            let b_reconstructed: [fpr; LENGTH] = reconstruct_arr(&b_shares);

            check_fpr_arr_eq(&a, &a_reconstructed);
            check_fpr_arr_eq(&b, &b_reconstructed);
        }
    }

    #[test]
    fn test_poly_invnorm2_fft() {
        const LOGN: u32 = 10;
        const LENGTH: usize = 1 << LOGN;
        for _ in 0..100 {
            let (mut a, mut a_shares): ([fpr; LENGTH], [[fpr; 2]; LENGTH]) = create_random_mask_arr::<LENGTH>();
            let (mut b, mut b_shares): ([fpr; LENGTH], [[fpr; 2]; LENGTH]) = create_random_mask_arr::<LENGTH>();
            let (mut d, mut d_shares): ([fpr; LENGTH], [[fpr; 2]; LENGTH]) = create_random_mask_arr::<LENGTH>();

            u_poly_invnorm2_fft(&mut d, &a, &b,LOGN);
            poly_invnorm2_fft(&mut d_shares, &a_shares, &b_shares, LOGN);

            let a_reconstructed: [fpr; LENGTH] = reconstruct_arr(&a_shares);
            let b_reconstructed: [fpr; LENGTH] = reconstruct_arr(&b_shares);
            let d_reconstructed: [fpr; LENGTH] = reconstruct_arr(&d_shares);

            check_fpr_arr_eq(&a, &a_reconstructed);
            check_fpr_arr_eq(&b, &b_reconstructed);
            check_fpr_arr_eq(&d, &d_reconstructed);
        }
    }

    fn check_fpr_eq(x: fpr, y: fpr) {
        assert!(approx_eq!(f64, fpr_to_double(x), fpr_to_double(y), epsilon = 0.000000003));
    }

    fn check_fpr_arr_eq(x: &[fpr], y: &[fpr]) {
        for i in 0..x.len() {
            assert!(approx_eq!(f64, fpr_to_double(x[i]), fpr_to_double(y[i]), epsilon = 0.000000003), "The two arrays differ in position {}\nLeft: {}\nRight: {}", i, x[i], y[i]);
        }
    }

    pub fn create_random_fpr() -> fpr {
        let mut rng = thread_rng();
        let random: f64 = rng.gen_range(-200f64..200f64);
        return f64::to_bits(random);
    }

    fn fpr_to_double(x: fpr) -> f64 {
        return f64::from_bits(x);
    }

    fn create_random_mask() -> (fpr, [fpr; 2]) {
        let mut arr: [fpr; 2] = [0; 2];
        let val: fpr = create_random_fpr();
        arr[1] = create_random_fpr();
        arr[0] = u_fpr_sub(val, arr[1]);
        return (val, arr);
    }

    fn create_random_mask_arr<const LENGTH: usize>() -> ([fpr; LENGTH], [[fpr; 2]; LENGTH]) {
        let mut f_shares: [[fpr; 2]; LENGTH]  = [[0; 2]; LENGTH];
        let mut f: [fpr; LENGTH] = [0; LENGTH];

        for i in 0..LENGTH {
            let (val, masked_val) = create_random_mask();
            f[i] = val;
            f_shares[i] = masked_val;
        }

        return (f, f_shares);
    }

    fn reconstruct(shares: &[fpr]) -> fpr {
        return u_fpr_add(shares[0], shares[1]);
    }

    fn reconstruct_arr<const LENGTH: usize>(share_arr: &[[fpr; 2]; LENGTH]) -> [fpr; LENGTH] {
        let mut reconstructed: [fpr; LENGTH] = [0; LENGTH];

        for i in 0..LENGTH {
            let res = reconstruct(&share_arr[i]);
            reconstructed[i] = res;
        }

        return reconstructed;
    }
}