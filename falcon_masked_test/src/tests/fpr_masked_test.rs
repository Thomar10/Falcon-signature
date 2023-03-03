#[cfg(test)]
mod tests {
    use rand::{Rng, thread_rng};
    use falcon::falcon::fpr;
    use falcon::fpr::{fpr_add as add, fpr_div as div, fpr_double as double, fpr_expm_p63 as expm_p63, fpr_floor as floor, fpr_half as half, fpr_inv as inv, fpr_lt as lt, fpr_mul as mul, fpr_neg as neg, fpr_of, fpr_rint as rint, fpr_sqrt as sqrt, fpr_sub as sub, fpr_trunc as trunc, FPR_TWO};
    use falcon_masked::fpr_masked::{fpr_add, fpr_div, fpr_double, fpr_expm_p63, fpr_floor, fpr_half, fpr_inv, fpr_lt, fpr_mul, fpr_neg, fpr_rint, fpr_sqr, fpr_sqrt, fpr_sub, fpr_trunc};

    #[test]
    fn fpr_add_test() {
        for _ in 0..100 {
            let mut shares_x = [0; 2];
            let mut shares_y = [0; 2];
            let (x, y) = create_masked(&mut shares_x, &mut shares_y);
            let add_shares = fpr_add(&shares_x, &shares_y);
            let (xx, yy) = reconstruct(&add_shares, &shares_y);
            assert_eq!(yy, y, "y");
            assert_eq!(xx, add(x, y), "x + y");
        }
    }

    #[test]
    fn fpr_expm_p63_test() {
        for _ in 0..100 {
            let mut shares_x = [0; 2];
            let mut shares_y = [0; 2];
            let (x, y) = create_masked(&mut shares_x, &mut shares_y);
            let expm_p63_shares = fpr_expm_p63(&shares_x, &shares_y);
            let (xx, yy) = reconstruct(&expm_p63_shares, &shares_y);
            assert_eq!(yy, y);
            assert_eq!(xx, expm_p63(x, y));
        }
    }

    #[test]
    fn fpr_sub_test() {
        for _ in 0..100 {
            let mut shares_x = [0; 2];
            let mut shares_y = [0; 2];
            let (x, y) = create_masked(&mut shares_x, &mut shares_y);
            let sub_shares = fpr_sub(&shares_x, &shares_y);
            let (xx, yy) = reconstruct(&sub_shares, &shares_y);
            assert_eq!(yy, y);
            assert_eq!(xx, sub(x, y));
        }
    }

    #[test]
    fn fpr_mul_test() {
        for _ in 0..100 {
            let mut shares_x = [0; 2];
            let mut shares_y = [0; 2];
            let (x, y) = create_masked(&mut shares_x, &mut shares_y);
            let mul_shares = fpr_mul(&shares_x, &shares_y);
            let (xx, yy) = reconstruct(&mul_shares, &shares_y);
            println!("yy: {:?}", shares_y);
            assert_eq!(yy, y);
            println!("Masked result: {}", fpr_to_double(xx));
            println!("Unmasked result: {}", fpr_to_double(mul(x, y)));
            //assert_eq!(fpr_to_double(xx), fpr_to_double(mul(x, y)));
            assert_eq!(xx, mul(x, y));
        }
    }

    #[test]
    fn rand_test() {
        for _ in 0..100000 {
            let mut rng = thread_rng();

            let mut x: f64 = rng.gen_range(-100f64..100f64);
            let y: f64 = rng.gen_range(-100f64..100f64);

            let mut x_fpr: fpr = f64::to_bits(x);
            let y_fpr: fpr = f64::to_bits(y);

            for _ in 0..100 {
                let res: f64 = x * y;
                let res_fpr: fpr = mul(x_fpr, y_fpr);

                assert_eq!(res, fpr_to_double(res_fpr));

                x = res;
                x_fpr = res_fpr;
            }
        }
    }

    #[test]
    fn precision_test() {
        for _ in 0..100000 {
            let mut rng = thread_rng();

            let mut x: f64 = rng.gen_range(-100f64..100f64);
            let y: f64 = rng.gen_range(-100f64..100f64);

            let mut x_fpr: fpr = f64::to_bits(x);
            let y_fpr: fpr = f64::to_bits(y);

            let mut x_fpr_control: fpr = x_fpr;
            let y_fpr_control: fpr = y_fpr;

            for _ in 0..100 {
                let res_fpr_control: fpr = mul(x_fpr_control, y_fpr_control);
                let mut res_fpr: fpr = div(x_fpr, FPR_TWO);
                res_fpr = mul(res_fpr, y_fpr);
                res_fpr = mul(res_fpr, FPR_TWO);
                res_fpr = mul(res_fpr, y_fpr);
                res_fpr = div(res_fpr, y_fpr);

                assert_eq!(res_fpr_control, res_fpr);

                x_fpr_control = res_fpr_control;
                x_fpr = res_fpr;
            }
        }
    }

    #[test]
    fn fpr_sqr_test() {
        for _ in 0..100 {
            let mut shares_x = [0; 2];
            let mut shares_y = [0; 2];
            let (x, y) = create_masked(&mut shares_x, &mut shares_y);
            let sqr_shares = fpr_sqr(&shares_x);
            let (xx, yy) = reconstruct(&sqr_shares, &shares_y);
            assert_eq!(yy, y);
            assert_eq!(xx, mul(x, x));
        }
    }

    #[test]
    fn fpr_neg_test() {
        for _ in 0..100 {
            let mut shares_x = [0; 2];
            let mut shares_y = [0; 2];
            let (x, y) = create_masked(&mut shares_x, &mut shares_y);
            let neg_shares = fpr_neg(&shares_x);
            let (xx, yy) = reconstruct(&neg_shares, &shares_y);
            assert_eq!(yy, y);
            assert_eq!(xx, neg(x));
        }
    }

    #[test]
    fn fpr_half_test() {
        for _ in 0..100 {
            let mut shares_x = [0; 2];
            let mut shares_y = [0; 2];
            let (x, y) = create_masked(&mut shares_x, &mut shares_y);
            let half_shares = fpr_half(&shares_x);
            let (xx, yy) = reconstruct(&half_shares, &shares_y);
            assert_eq!(yy, y);
            assert_eq!(xx, half(x));
        }
    }

    #[test]
    fn fpr_double_test() {
        for _ in 0..100 {
            let mut shares_x = [0; 2];
            let mut shares_y = [0; 2];
            let (x, y) = create_masked(&mut shares_x, &mut shares_y);
            let double_shares = fpr_double(&shares_x);
            let (xx, yy) = reconstruct(&double_shares, &shares_y);
            assert_eq!(yy, y);
            assert_eq!(xx, double(x));
        }
    }

    #[test]
    fn fpr_inv_test() {
        for _ in 0..100 {
            let mut shares_x = [0; 2];
            let mut shares_y = [0; 2];
            let (x, y) = create_masked(&mut shares_x, &mut shares_y);
            let inv_shares = fpr_inv(&shares_x);
            let (xx, yy) = reconstruct(&inv_shares, &shares_y);
            assert_eq!(yy, y);
            assert_eq!(xx, inv(x));
        }
    }

    #[test]
    fn fpr_sqrt_test() {
        for _ in 0..100 {
            let mut shares_x = [0; 2];
            let mut shares_y = [0; 2];
            let (x, y) = create_masked(&mut shares_x, &mut shares_y);
            let add_shares = fpr_sqrt(&shares_x);
            let (xx, yy) = reconstruct(&add_shares, &shares_y);
            assert_eq!(yy, y);
            assert_eq!(xx, sqrt(x));
        }
    }

    #[test]
    fn fpr_div_test() {
        for _ in 0..100 {
            let mut shares_x = [0; 2];
            let mut shares_y = [0; 2];
            let (x, y) = create_masked(&mut shares_x, &mut shares_y);
            let div_shares = fpr_div(&shares_x, &shares_y);
            let (xx, _) = reconstruct(&div_shares, &shares_y);
            assert_eq!(xx, div(x, y));
        }
    }

    #[test]
    fn fpr_rint_test() {
        for _ in 0..100 {
            let mut shares_x = [0; 2];
            let mut shares_y = [0; 2];
            let (x, _) = create_masked(&mut shares_x, &mut shares_y);
            let rint_res = fpr_rint(&shares_x);
            assert_eq!(rint_res[0] + rint_res[1], rint(x));
        }
    }

    #[test]
    fn fpr_trunc_test() {
        for _ in 0..100 {
            let mut shares_x = [0; 2];
            let mut shares_y = [0; 2];
            let (x, _) = create_masked(&mut shares_x, &mut shares_y);
            let rtrunc_res = fpr_trunc(&shares_x);

            assert_eq!(rtrunc_res[0] + rtrunc_res[1], trunc(x));
        }
    }

    #[test]
    fn fpr_floor_test() {
        for _ in 0..100 {
            let mut shares_x = [0; 2];
            let mut shares_y = [0; 2];
            let (x, _) = create_masked(&mut shares_x, &mut shares_y);
            let floor_res = fpr_floor(&shares_x);

            assert_eq!(floor_res[0] + floor_res[1], floor(x));
        }
    }

    #[test]
    fn fpr_lt_test() {
        for _ in 0..100 {
            let mut shares_x = [0; 2];
            let mut shares_y = [0; 2];
            let (x, _) = create_masked(&mut shares_x, &mut shares_y);
            let random = create_random_fpr();
            let lt_res = fpr_lt(&shares_x, random);

            assert_eq!(lt_res, lt(x, random));
        }
    }

    pub fn create_masked(x: &mut [fpr], y: &mut [fpr]) -> (fpr, fpr) {
        let x_fpr = create_random_fpr();
        let y_fpr = create_random_fpr();
        println!("y_fpr: {}", fpr_to_double(y_fpr));
        let x_random = create_random_fpr();
        let y_random = create_random_fpr();
        let first = vec![sub(x_fpr, x_random), x_random];
        let second = vec![sub(y_fpr, y_random), y_random];
        println!("second: {:?}", second);
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
        println!("random: {}", random);
        return f64::to_bits(random);
        //fpr_of(f64::to_bits(random as f64) as i64)
    }

    pub fn fpr_to_double(x: fpr) -> f64 {
        return f64::from_bits(x);
    }
}