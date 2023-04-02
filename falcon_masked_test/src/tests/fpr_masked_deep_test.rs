use float_cmp::approx_eq;
use rand::{random, Rng, thread_rng};

use falcon::falcon::fpr;
use falcon::fpr::{fpr, fpr_add, fpr_mul, fpr_norm64, fpr_ursh};
use falcon_masked::fpr_masked_deep::{secure_and, secure_fpr, secure_fpr_add, secure_fpr_norm, secure_mul, secure_mul_test, secure_neg, secure_or, secure_ursh};

#[test]
fn mul_fpr_test() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let x = create_random_fpr();
        let y = create_random_fpr();
        let expected = fpr_mul(x, y);

        let sharex: u64 = rng.gen_range(0..2);
        let x_sign = x >> 63;
        let s_sharex: [u64; 2] = [(x_sign as u64) ^ sharex, sharex];
        let share_ex: i16 = random();
        let x_exp = (x >> 52) & 0x7FF;
        let mut e_sharex: [i16; 2] = [((x_exp) as i16).wrapping_sub(share_ex), share_ex];
        let share_mx: i64 = rng.gen_range(0..18014398509481983);
        let x_man = x & 0xFFFFFFFFFFFFF;
        let mut m_sharex: [i128; 2] = [((x_man) as i128).wrapping_sub(share_mx as i128), share_mx as i128];


        let sharey: u64 = rng.gen_range(0..2);
        let s_sharey: [u64; 2] = [((y >> 63) as u64) ^ sharey, sharey];
        let share_ey: i16 = random();
        let mut e_sharey: [i16; 2] = [(((y >> 52) & 0x7FF) as i16).wrapping_sub(share_ey), share_ey];
        let share_my: i64 = rng.gen_range(0..18014398509481983);
        let y_man = y & 0xFFFFFFFFFFFFF;
        let mut m_sharey: [i128; 2] = [(y_man as i128).wrapping_sub(share_my as i128), share_my as i128];
        let result: [fpr; 2] = secure_mul(&s_sharex, &mut e_sharex, &mut m_sharex, &s_sharey, &mut e_sharey, &mut m_sharey);
        check_eq_fpr(expected, result[0] ^ result[1]);
    }
}


#[test]
fn mul_fpr_test2() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let x = create_random_fpr();
        let y = create_random_fpr();
        let (s, e, z) = fpr_mul_test(x, y);

        let sharex: u64 = rng.gen_range(0..2);
        let x_sign = x >> 63;
        let s_sharex: [u64; 2] = [(x_sign as u64) ^ sharex, sharex];
        let share_ex: i16 = random();
        let x_exp = (x >> 52) & 0x7FF;
        let mut e_sharex: [i16; 2] = [((x_exp) as i16).wrapping_sub(share_ex), share_ex];
        let share_mx: i64 = rng.gen_range(0..18014398509481983);
        let x_man = x & 0xFFFFFFFFFFFFF;
        let mut m_sharex: [i128; 2] = [((x_man) as i128).wrapping_sub(share_mx as i128), share_mx as i128];


        let sharey: u64 = rng.gen_range(0..2);
        let s_sharey: [u64; 2] = [((y >> 63) as u64) ^ sharey, sharey];
        let share_ey: i16 = random();
        let mut e_sharey: [i16; 2] = [(((y >> 52) & 0x7FF) as i16).wrapping_sub(share_ey), share_ey];
        let share_my: i64 = rng.gen_range(0..18014398509481983);
        let y_man = y & 0xFFFFFFFFFFFFF;
        let mut m_sharey: [i128; 2] = [(y_man as i128).wrapping_sub(share_my as i128), share_my as i128];
        let (ss, ee, zz) = secure_mul_test::<2>(&s_sharex, &mut e_sharex, &mut m_sharex, &s_sharey, &mut e_sharey, &mut m_sharey);
        assert_eq!(s, (ss[0] ^ ss[1]) as i32);
        assert_eq!(e, (ee[0].wrapping_add(ee[1])) as i32);
        assert_eq!(z, (zz[0] ^ zz[1]) as u64);
    }
}

#[test]
fn fpr_norm() {
    for _ in 0..100 {
        let x: u64 = random();
        let share_x: u64 = random();
        let x_share: [u64; 2] = [x ^ share_x, share_x];
        let e: i16 = random();
        let share_e: i16 = random();
        let e_share: [i16; 2] = [(e as i16).wrapping_sub(share_e), share_e];
        let (xx, ee) = secure_fpr_norm::<2>(&x_share, &e_share);
        let (exp_x, exp_e) = fpr_norm64(x, e as i32);
        assert_eq!(exp_x, xx[0] ^ xx[1]);
        assert_eq!(exp_e, (ee[0].wrapping_add(ee[1])) as i32);
    }
}


#[test]
fn fpr_ursh_test() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let x: u64 = random();
        let share_x: u64 = random();
        let x_share: [u64; 2] = [x ^ share_x, share_x];
        let n: i8 = rng.gen_range(0..64);
        let share_n: i8 = rng.gen_range(0..64);
        let n_share: [i8; 2] = [n ^ share_n, share_n];
        let xx = secure_ursh::<2>(&x_share, &n_share);
        let expected = fpr_ursh(x, n as i32);
        assert_eq!(expected, xx[0] ^ xx[1]);
    }
}

#[test]
fn secure_fpr_add_test() {
    for _ in 0..1000 {
        let x = create_random_fpr();
        let x_share = create_random_fpr();
        let y = create_random_fpr();
        let y_share = create_random_fpr();
        let x_mask: [fpr; 2] = [x ^ x_share, x_share];
        let y_mask: [fpr; 2] = [y ^ y_share, y_share];
        let expected = fpr_add(x, y);
        let result: [fpr; 2] = secure_fpr_add(&x_mask, &y_mask);
        check_eq_fpr(expected, result[0] ^ result[1]);
    }
}


#[test]
fn fpr_test() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let (s, e, z) = fpr_mul_test(create_random_fpr(), create_random_fpr());

        let expected = fpr(s, e, z);
        let share: u64 = rng.gen_range(0..2);
        let s_share: [u64; 2] = [(s as u64) ^ share, share];
        let share_e: i16 = random();
        let mut e_share: [i16; 2] = [(e as i16).wrapping_sub(share_e), share_e];
        let share: i64 = rng.gen_range(0..18014398509481983);
        let mut z_share = [(z as i64) ^ share, share];
        let result: [fpr; 2] = secure_fpr(&s_share, &mut e_share, &mut z_share);
        check_eq_fpr(expected, result[0] ^ result[1]);
    }
}

#[test]
fn fpr_test2() {
    for _ in 0..1000 {
        let (s, e, z) = (1, -47, 31513121540324152);
        let expected = fpr(s, e, z);
        let s_share: [u64; 2] = [0, 1];
        let mut e_share: [i16; 2] = [-15010, 14963];
        let mut z_share = [55249731794911175, 48339284658128127];
        let result: [fpr; 2] = secure_fpr(&s_share, &mut e_share, &mut z_share);
        // println!("expected {}", fpr_to_double(expected));
        // println!("got {}", fpr_to_double(result[0] ^ result[1]));
        check_eq_fpr(expected, result[0] ^ result[1]);
    }
}

#[test]
fn fpr_test3() {
    for _ in 0..1000 {
        let (s, e, z) = (0, -43, 16251135920610578);
        let expected = fpr(s, e, z);
        let s_share: [u64; 2] = [1, 1];
        let mut e_share: [i16; 2] = [-14433, 14390];
        let mut z_share = [34770256359083929, 18642285226283659];
        let result: [fpr; 2] = secure_fpr(&s_share, &mut e_share, &mut z_share);
        // println!("expected {}", fpr_to_double(expected));
        // println!("got {}", fpr_to_double(result[0] ^ result[1]));
        check_eq_fpr(expected, result[0] ^ result[1]);
    }
}

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
        let (x, _) = create_masked(&mut shares_x, &mut shares_y);
        let neg_shares = secure_neg::<2>(&shares_x);
        let (xx, _) = reconstruct(&neg_shares, &shares_y);

        check_eq_fpr(xx, x.wrapping_neg());
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

pub fn fpr_mul_test(x: fpr, y: fpr) -> (i32, i32, u64) {
    let (xu, yu, mut w, mut zu, zv): (u64, u64, u64, u64, u64);
    let (x0, x1, y0, y1, z0, mut z1, mut z2): (u32, u32, u32, u32, u32, u32, u32);
    let (ex, ey, d, e, s): (i32, i32, i32, i32, i32);

    xu = (x & (((1 as u64) << 52) - 1)) | ((1 as u64) << 52);
    yu = (y & (((1 as u64) << 52) - 1)) | ((1 as u64) << 52);


    x0 = (xu as u32) & 0x01FFFFFF;
    x1 = (xu >> 25) as u32;
    y0 = (yu as u32) & 0x01FFFFFF;
    y1 = (yu >> 25) as u32;
    w = (x0 as u64) * (y0 as u64);
    z0 = (w as u32) & 0x01FFFFFF;
    z1 = (w >> 25) as u32;
    w = (x0 as u64) * (y1 as u64);
    z1 += (w as u32) & 0x01FFFFFF;
    z2 = (w >> 25) as u32;
    w = (x1 as u64) * (y0 as u64);
    z1 += (w as u32) & 0x01FFFFFF;
    z2 += (w >> 25) as u32;
    zu = (x1 as u64) * (y1 as u64);
    z2 += z1 >> 25;
    z1 &= 0x01FFFFFF;
    zu += z2 as u64;

    zu |= (((z0 | z1) + 0x01FFFFFF) >> 25) as u64;

    zv = (zu >> 1) | (zu & 1);
    w = zu >> 55;
    zu ^= (zu ^ zv) & (!w).wrapping_add(1);

    ex = ((x >> 52) & 0x7FF) as i32;
    ey = ((y >> 52) & 0x7FF) as i32;
    e = ex + ey - 2100 + w as i32;

    /*
     * Sign bit is the XOR of the operand sign bits.
     */
    s = ((x ^ y) >> 63) as i32;


    d = ((ex + 0x7FF) & (ey + 0x7FF)) >> 11;
    zu &= (!d as u64).wrapping_add(1);
    (s, e, zu)
}