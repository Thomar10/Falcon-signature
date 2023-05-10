use falcon::falcon::fpr;
use randomness::random::RngBoth;


use crate::fpr_masked::FPR_GM_TAB;
use crate::fpr_masked_deep::{secure_fpr_add, secure_fpr_sub, secure_mul};

pub fn secure_fpc_add<const ORDER: usize>(a_re: &[fpr], a_im: &[fpr], b_re: &[fpr], b_im: &[fpr], rng: &mut RngBoth) -> ([fpr; ORDER], [fpr; ORDER]) {
    let fpct_re: [fpr; ORDER] = secure_fpr_add(a_re, b_re, rng);
    let fpct_im: [fpr; ORDER] = secure_fpr_add(a_im, b_im, rng);
    return (fpct_re, fpct_im);
}

pub fn secure_fpc_sub<const ORDER: usize>(a_re: &[fpr], a_im: &[fpr], b_re: &[fpr], b_im: &[fpr], rng: &mut RngBoth) -> ([fpr; ORDER], [fpr; ORDER]) {
    let fpct_re: [fpr; ORDER] = secure_fpr_sub(a_re, b_re, rng);
    let fpct_im: [fpr; ORDER] = secure_fpr_sub(a_im, b_im, rng);
    return (fpct_re, fpct_im);
}

pub fn secure_fpc_mul<const ORDER: usize>(a_re: &[fpr], a_im: &[fpr], b_re: &[fpr], b_im: &[fpr], rng: &mut RngBoth) -> ([fpr; ORDER], [fpr; ORDER]) {
    let fpct_d_re: [fpr; ORDER] = secure_fpr_sub(
        &secure_mul::<ORDER>(a_re, b_re, rng),
        &secure_mul::<ORDER>(a_im, b_im, rng), rng);
    let fpct_d_im: [fpr; ORDER] = secure_fpr_add(
        &secure_mul::<ORDER>(a_re, b_im, rng),
        &secure_mul::<ORDER>(a_im, b_re, rng), rng);
    return (fpct_d_re, fpct_d_im);
}

pub fn secure_fft<const ORDER: usize>(f: &mut [[fpr; ORDER]], logn: u32, rng: &mut RngBoth) {
    let mut u: u32 = 1;
    let mut m: usize = 2;
    let (mut t, n, hn): (usize, usize, usize);

    n = (1 as usize) << logn;
    hn = n >> 1;
    t = hn;

    while u < logn {
        let (ht, hm, mut i1, mut j1): (usize, usize, usize, usize);
        ht = t >> 1;
        hm = m >> 1;
        i1 = 0;
        j1 = 0;
        while i1 < hm {
            let (mut j, j2): (usize, usize);
            j2 = j1 + ht;
            let (s_re, s_im): ([fpr; 2], [fpr; 2]);
            s_re = [FPR_GM_TAB[((m + i1) << 1) + 0], 0];
            s_im = [FPR_GM_TAB[((m + i1) << 1) + 1], 0];
            j = j1;

            while j < j2 {
                let (x_re, x_im, mut y_re, mut y_im): ([fpr; ORDER], [fpr; ORDER], [fpr; ORDER], [fpr; ORDER]);
                x_re = f[j];
                x_im = f[j + hn];
                y_re = f[j + ht];
                y_im = f[j + ht + hn];
                (y_re, y_im) = secure_fpc_mul(&y_re, &y_im, &s_re, &s_im, rng);
                (f[j], f[j + hn]) = secure_fpc_add(&x_re, &x_im, &y_re, &y_im, rng);
                (f[j + ht], f[j + ht + hn]) = secure_fpc_sub(&x_re, &x_im, &y_re, &y_im, rng);

                j += 1;
            }

            i1 += 1;
            j1 += t;
        }
        u += 1;
        m <<= 1;
        t = ht;
    }
}