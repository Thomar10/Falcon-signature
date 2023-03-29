use float_cmp::approx_eq;
use rand::{random, Rng, thread_rng};

use falcon::falcon::fpr;
use falcon::fpr::fpr_ursh;
use falcon_masked::fpr_masked_deep::{secure_and, secure_neg, secure_non_zero, secure_or, secure_ursh};

#[test]
fn fpr_or_test() {
    for _ in 0..100 {
        let mut shares_x = [0; 2];
        let mut shares_y = [0; 2];
        let (x, y) = create_masked(&mut shares_x, &mut shares_y);
        let add_shares = secure_or::<2>(&shares_x, &shares_y);
        let (xx, _) = reconstruct(&add_shares, &shares_y);

        check_eq_fpr(xx, x | y);
    }
}

#[test]
fn fpr_and_test() {
    for _ in 0..100 {
        let mut shares_x = [0; 2];
        let mut shares_y = [0; 2];
        let (x, y) = create_masked(&mut shares_x, &mut shares_y);
        let add_shares = secure_and::<2>(&shares_x, &shares_y);
        let (xx, _) = reconstruct(&add_shares, &shares_y);

        check_eq_fpr(xx, x & y);
    }
}

#[test]
fn fpr_neg_test() {
    for _ in 0..100 {
        let mut shares_x = [0; 2];
        let mut shares_y = [0; 2];
        let (x, y) = create_masked(&mut shares_x, &mut shares_y);
        let neg_shares = secure_neg::<2>(&shares_x);
        let (xx, _) = reconstruct(&neg_shares, &shares_y);
        println!("{}", xx);
        println!("{}", x.wrapping_neg());

        check_eq_fpr(xx, x.wrapping_neg());
    }
}

#[test]
fn fpr_ursh_test() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        for n in 0..64 {
            let mut shares_x = [0; 2];
            let mut shares_y = [0; 2];
            let (x, y) = create_masked(&mut shares_x, &mut shares_y);
            let share:i8 = rng.gen_range(0..64);
            let n_share = (n - share) & 63;
            let neg_shares = secure_non_zero::<2>(&shares_x, true);
            let (xx, _) = reconstruct(&neg_shares, &shares_y);
            println!("{}", xx);
            println!("{}", fpr_ursh(x, n as i32));
            println!("n {}", n);

            check_eq_fpr(xx, fpr_ursh(x, n as i32));
        }
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
    let first = vec![x_fpr ^ x_random, x_random];
    let second = vec![y_fpr ^ y_random, y_random];
    x.clone_from_slice(&first);
    y.clone_from_slice(&second);
    (x_fpr, y_fpr)
}

pub fn reconstruct(x: &[fpr], y: &[fpr]) -> (fpr, fpr) {
    let xx = x[0] ^ x[1];
    let yy = y[0] ^ y[1];
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