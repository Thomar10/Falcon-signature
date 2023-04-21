#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;
    use rand::{Rng, thread_rng};

    use falcon::falcon::fpr;
    use falcon::fpr::{fpr_add as add, fpr_div as div, fpr_double as double, fpr_expm_p63 as expm_p63, fpr_floor as floor, fpr_half as half, fpr_inv as inv, fpr_lt as lt, fpr_mul as mul, fpr_neg as neg, fpr_rint as rint, fpr_sqrt as sqrt, fpr_sub as sub, fpr_trunc as trunc};
    use falcon_masked::fpr_masked::{fpr_add, fpr_div, fpr_double, fpr_expm_p63, fpr_floor, fpr_half, fpr_inv, fpr_lt, fpr_mul, fpr_neg, fpr_rint, fpr_sqr, fpr_sqrt, fpr_sub, fpr_trunc};

    const ORDER: usize = 2;

    #[test]
    fn fpr_add_test() {
        for _ in 0..100 {
            let mut shares_x = [0; ORDER];
            let mut shares_y = [0; ORDER];
            let (x, y) = create_masked(&mut shares_x, &mut shares_y);
            let add_shares: [fpr; ORDER] = fpr_add(&shares_x, &shares_y);
            let (xx, _) = reconstruct(&add_shares, &shares_y);

            check_eq_fpr(xx, add(x, y));
        }
    }

    #[test]
    fn fpr_expm_p63_test() {
        for _ in 0..100 {
            let mut shares_x = [0; ORDER];
            let mut shares_y = [0; ORDER];
            let (x, y) = create_masked(&mut shares_x, &mut shares_y);
            let expm_p63_shares = fpr_expm_p63(&shares_x, &shares_y);
            let (xx, _) = reconstruct(&expm_p63_shares, &shares_y);

            check_eq_fpr(xx, expm_p63(x, y));
        }
    }

    #[test]
    fn fpr_sub_test() {
        for _ in 0..100 {
            let mut shares_x = [0; ORDER];
            let mut shares_y = [0; ORDER];
            let (x, y) = create_masked(&mut shares_x, &mut shares_y);
            let sub_shares: [fpr; ORDER] = fpr_sub(&shares_x, &shares_y);
            let (xx, _) = reconstruct(&sub_shares, &shares_y);

            check_eq_fpr(xx, sub(x, y));
        }
    }

    #[test]
    fn fpr_mul_test() {
        for _ in 0..100 {
            let mut shares_x = [0; ORDER];
            let mut shares_y = [0; ORDER];
            let (x, y) = create_masked(&mut shares_x, &mut shares_y);
            let mul_shares: [fpr; ORDER] = fpr_mul(&shares_x, &shares_y);
            let (xx, _) = reconstruct(&mul_shares, &shares_y);

            check_eq_fpr(xx, mul(x, y));
        }
    }

    #[test]
    fn fpr_sqr_test() {
        for _ in 0..100 {
            let mut shares_x = [0; ORDER];
            let mut shares_y = [0; ORDER];
            let (x, _) = create_masked(&mut shares_x, &mut shares_y);
            let sqr_shares: [fpr; ORDER] = fpr_sqr(&shares_x);
            let (xx, _) = reconstruct(&sqr_shares, &shares_y);

            check_eq_fpr(xx, mul(x, x));
        }
    }

    #[test]
    fn fpr_neg_test() {
        for _ in 0..100 {
            let mut shares_x = [0; ORDER];
            let mut shares_y = [0; ORDER];
            let (x, _) = create_masked(&mut shares_x, &mut shares_y);
            let neg_shares: [fpr; ORDER] = fpr_neg(&shares_x);
            let (xx, _) = reconstruct(&neg_shares, &shares_y);

            check_eq_fpr(xx, neg(x));
        }
    }

    #[test]
    fn fpr_half_test() {
        for _ in 0..100 {
            let mut shares_x = [0; ORDER];
            let mut shares_y = [0; ORDER];
            let (x, _) = create_masked(&mut shares_x, &mut shares_y);
            let half_shares: [fpr; ORDER] = fpr_half(&shares_x);
            let (xx, _) = reconstruct(&half_shares, &shares_y);

            check_eq_fpr(xx, half(x));
        }
    }

    #[test]
    fn fpr_double_test() {
        for _ in 0..100 {
            let mut shares_x = [0; ORDER];
            let mut shares_y = [0; ORDER];
            let (x, _) = create_masked(&mut shares_x, &mut shares_y);
            let double_shares: [fpr; ORDER] = fpr_double(&shares_x);
            let (xx, _) = reconstruct(&double_shares, &shares_y);

            check_eq_fpr(xx, double(x));
        }
    }

    #[test]
    fn fpr_inv_test() {
        for _ in 0..100 {
            let mut shares_x = [0; ORDER];
            let mut shares_y = [0; ORDER];
            let (x, _) = create_masked(&mut shares_x, &mut shares_y);
            let inv_shares: [fpr; ORDER] = fpr_inv(&shares_x);
            let (xx, _) = reconstruct(&inv_shares, &shares_y);

            check_eq_fpr(xx, inv(x));
        }
    }

    #[test]
    fn fpr_sqrt_test() {
        for _ in 0..100 {
            let mut shares_x = [0; ORDER];
            let mut shares_y = [0; ORDER];
            let (x, _) = create_masked(&mut shares_x, &mut shares_y);
            let add_shares: [fpr; ORDER] = fpr_sqrt(&shares_x);
            let (xx, _) = reconstruct(&add_shares, &shares_y);

            check_eq_fpr(xx, sqrt(x));
        }
    }

    #[test]
    fn fpr_div_test() {
        for _ in 0..100 {
            let mut shares_x = [0; ORDER];
            let mut shares_y = [0; ORDER];
            let (x, y) = create_masked(&mut shares_x, &mut shares_y);
            let div_shares: [fpr; ORDER] = fpr_div(&shares_x, &shares_y);
            let (xx, _) = reconstruct(&div_shares, &shares_y);

            check_eq_fpr(xx, div(x, y));
        }
    }

    #[test]
    fn fpr_rint_test() {
        for _ in 0..100 {
            let mut shares_x = [0; ORDER];
            let mut shares_y = [0; ORDER];
            let (x, _) = create_masked(&mut shares_x, &mut shares_y);
            let rint_res: [i64; ORDER] = fpr_rint(&shares_x);
            assert_eq!(rint_res[0] + rint_res[1], rint(x));
        }
    }


    #[test]
    fn fpr_floor_test() {
        for _ in 0..100 {
            let mut shares_x = [0; ORDER];
            let mut shares_y = [0; ORDER];
            let (x, _) = create_masked(&mut shares_x, &mut shares_y);
            let floor_res: [i64; ORDER] = fpr_floor(&shares_x);

            assert_eq!(floor_res[0] + floor_res[1], floor(x));
        }
    }

    #[test]
    fn fpr_lt_test() {
        for _ in 0..100 {
            let mut shares_x = [0; ORDER];
            let mut shares_y = [0; ORDER];
            let (x, _) = create_masked(&mut shares_x, &mut shares_y);
            let random = create_random_fpr();
            let lt_res: i32 = fpr_lt(&shares_x, random);
            assert_eq!(lt_res, lt(x, random));
        }
    }

    pub fn check_eq_fpr(x: fpr, y: fpr) {
        assert!(approx_eq!(f64, fpr_to_double(x), fpr_to_double(y), epsilon = 0.000000003));
    }

    pub fn create_masked(x: &mut [fpr], y: &mut [fpr]) -> (fpr, fpr) {
        let x_fpr = create_random_fpr();
        let y_fpr = create_random_fpr();
        let x_random = create_random_fpr();
        let y_random = create_random_fpr();
        let first = vec![sub(x_fpr, x_random), x_random];
        let second = vec![sub(y_fpr, y_random), y_random];
        x.clone_from_slice(&first);
        y.clone_from_slice(&second);
        (x_fpr, y_fpr)
    }

    pub fn reconstruct(x: &[fpr], y: &[fpr]) -> (fpr, fpr) {
        let xx = add(x[0], x[1]);
        let yy = add(y[0], y[1]);
        (xx, yy)
    }

    pub fn create_random_fpr() -> fpr {
        let mut rng = thread_rng();
        let random: f64 = rng.gen_range(-100f64..100f64);
        return f64::to_bits(random);
    }

    pub fn fpr_to_double(x: fpr) -> f64 {
        return f64::from_bits(x);
    }
}