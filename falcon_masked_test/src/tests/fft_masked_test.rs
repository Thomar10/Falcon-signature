#[cfg(test)]
mod tests {
    use rand::{Rng, thread_rng};
    use falcon::falcon::fpr;
    use falcon::fpr::{fpr_of as u_fpr_of, fpr_sub as u_fpr_sub, fpr_add as u_fpr_add};
    use falcon_masked::fft_masked::{fft, fpc_add, fpc_mul, fpc_sub};
    use falcon::fft::{fpc_add as u_fpc_add, fpc_mul as u_fpc_mul, fpc_sub as u_fpc_sub, fft as u_fft};
    use falcon_masked::fpr_masked::fpr_add;

    #[test]
    fn test_fpc_add() {
        for _ in 0..100 {
            let (a_re, shares_a_re): (fpr, [fpr; 2]) = create_random_mask();
            let (a_im, shares_a_im): (fpr, [fpr; 2]) = create_random_mask();
            let (b_re, shares_b_re): (fpr, [fpr; 2]) = create_random_mask();
            let (b_im, shares_b_im): (fpr, [fpr; 2]) = create_random_mask();

            let (shares_res_re, shares_res_im) = fpc_add(&shares_a_re, &shares_a_im, &shares_b_re, &shares_b_im);
            let (res_re, res_im) = u_fpc_add(a_re, a_im, b_re, b_im);

            assert_eq!(reconstruct(&shares_res_re), res_re);
            assert_eq!(reconstruct(&shares_res_im), res_im);
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

            assert_eq!(reconstruct(&shares_res_re), res_re);
            assert_eq!(reconstruct(&shares_res_im), res_im);
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

            assert_eq!(reconstruct(&shares_res_re), res_re);
            assert_eq!(reconstruct(&shares_res_im), res_im);
        }
    }

    #[test]
    fn test_fft() {
        const LOGN: u32 = 4;
        const LENGTH: usize = 1 << LOGN;
        for _ in 0..100 {
            let mut f_shares: [[fpr; 2]; LENGTH] = [[0; 2]; LENGTH];
            let mut f: [fpr; LENGTH] = [0; LENGTH];

            for i in 0..LENGTH {
                let (val, masked_val) = create_random_mask();
                f[i] = val;
                f_shares[i] = masked_val;
            }

            fft(&mut f_shares, LOGN);
            u_fft(&mut f, LOGN);

            //Reconstruct
            let mut f_reconstructed: [fpr; LENGTH] = [0; LENGTH];
            for i in 0..LENGTH {
                let res = reconstruct(&f_shares[i]);
                f_reconstructed[i] = res;
            }

            assert_eq!(f_reconstructed, f);
        }
    }

    fn create_random_mask() -> (fpr, [fpr; 2]) {
        let mut arr: [fpr; 2] = [0; 2];
        let val: fpr = create_random_fpr();
        arr[1] = create_random_fpr();
        arr[0] = u_fpr_sub(val, arr[1]);
        return (val, arr);
    }

    fn reconstruct(shares: &[fpr]) -> fpr {
        return u_fpr_add(shares[0], shares[1]);
    }

    pub fn create_random_fpr() -> fpr {
        let mut rng = thread_rng();
        let random: f64 = rng.gen_range(-200f64..200f64);
        return f64::to_bits(random);
    }
}