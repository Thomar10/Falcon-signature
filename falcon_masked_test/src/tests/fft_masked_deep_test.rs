use float_cmp::approx_eq;
use rand::{Rng, thread_rng};

use falcon::falcon::fpr;
use falcon::fft::{fpc_add, fpc_mul, fpc_sub};
use falcon_masked::fft_masked_deep::{secure_fpc_add, secure_fpc_mul, secure_fpc_sub};

#[test]
fn fpc_add_test() {
    for _ in 0..1000 {
        let x_re = create_random_fpr();
        let x_re_share = create_random_fpr();
        let y_re = create_random_fpr();
        let y_re_share = create_random_fpr();
        let x_re_mask: [fpr; 2] = [x_re ^ x_re_share, x_re_share];
        let y_re_mask: [fpr; 2] = [y_re ^ y_re_share, y_re_share];
        let x_im = create_random_fpr();
        let x_im_share = create_random_fpr();
        let y_im = create_random_fpr();
        let y_im_share = create_random_fpr();
        let x_im_mask: [fpr; 2] = [x_im ^ x_im_share, x_im_share];
        let y_im_mask: [fpr; 2] = [y_im ^ y_im_share, y_im_share];
        let (expected_re, expected_im) = fpc_add(x_re, x_im, y_re, y_im);
        let (result_re, result_im) =
            secure_fpc_add::<2>(&x_re_mask, &x_im_mask, &y_re_mask, &y_im_mask);
        check_eq_fpr(expected_re, result_re[0] ^ result_re[1]);
        check_eq_fpr(expected_im, result_im[0] ^ result_im[1]);
    }
}


#[test]
fn fpc_mul_test() {
    for _ in 0..1000 {
        let x_re = create_random_fpr();
        let x_re_share = create_random_fpr();
        let y_re = create_random_fpr();
        let y_re_share = create_random_fpr();
        let x_re_mask: [fpr; 2] = [x_re ^ x_re_share, x_re_share];
        let y_re_mask: [fpr; 2] = [y_re ^ y_re_share, y_re_share];
        let x_im = create_random_fpr();
        let x_im_share = create_random_fpr();
        let y_im = create_random_fpr();
        let y_im_share = create_random_fpr();
        let x_im_mask: [fpr; 2] = [x_im ^ x_im_share, x_im_share];
        let y_im_mask: [fpr; 2] = [y_im ^ y_im_share, y_im_share];
        let (expected_re, expected_im) = fpc_mul(x_re, x_im, y_re, y_im);
        let (result_re, result_im) =
            secure_fpc_mul::<2>(&x_re_mask, &x_im_mask, &y_re_mask, &y_im_mask);
        check_eq_fpr(expected_re, result_re[0] ^ result_re[1]);
        check_eq_fpr(expected_im, result_im[0] ^ result_im[1]);
    }
}


#[test]
fn fpc_sub_test() {
    for _ in 0..1000 {
        let x_re = create_random_fpr();
        let x_re_share = create_random_fpr();
        let y_re = create_random_fpr();
        let y_re_share = create_random_fpr();
        let x_re_mask: [fpr; 2] = [x_re ^ x_re_share, x_re_share];
        let y_re_mask: [fpr; 2] = [y_re ^ y_re_share, y_re_share];
        let x_im = create_random_fpr();
        let x_im_share = create_random_fpr();
        let y_im = create_random_fpr();
        let y_im_share = create_random_fpr();
        let x_im_mask: [fpr; 2] = [x_im ^ x_im_share, x_im_share];
        let y_im_mask: [fpr; 2] = [y_im ^ y_im_share, y_im_share];
        let (expected_re, expected_im) = fpc_sub(x_re, x_im, y_re, y_im);
        let (result_re, result_im) =
            secure_fpc_sub::<2>(&x_re_mask, &x_im_mask, &y_re_mask, &y_im_mask);
        check_eq_fpr(expected_re, result_re[0] ^ result_re[1]);
        check_eq_fpr(expected_im, result_im[0] ^ result_im[1]);
    }
}

pub fn check_eq_fpr(x: fpr, y: fpr) {
    assert!(approx_eq!(f64, fpr_to_double(x), fpr_to_double(y), epsilon = 0.000000003));
}

pub fn create_random_fpr() -> fpr {
    let mut rng = thread_rng();
    let random: f64 = rng.gen_range(-100f64..100f64);
    return f64::to_bits(random);
}

pub fn fpr_to_double(x: fpr) -> f64 {
    return f64::from_bits(x);
}