use crate::fpr::{fpr_add, FPR_GM_TAB, fpr_mul, fpr_sub};

fn fpc_add(a_re: u64, a_im: u64,
           b_re: u64, b_im: u64) -> (u64, u64) {
    let fpct_re: u64 = fpr_add(a_re, b_re);
    let fpct_im: u64 = fpr_add(a_im, b_im);
    (fpct_re, fpct_im)
}

fn fpc_sub(a_re: u64, a_im: u64,
           b_re: u64, b_im: u64) -> (u64, u64) {
    let fpct_re: u64 = fpr_sub(a_re, b_re);
    let fpct_im: u64 = fpr_sub(a_im, b_im);
    (fpct_re, fpct_im)
}

fn fpc_mul(a_re: u64, a_im: u64,
           b_re: u64, b_im: u64) -> (u64, u64) {
    let fpct_d_re = fpr_sub(
        fpr_mul(a_re, b_re),
        fpr_mul(a_im, b_im));
    let fpct_d_im = fpr_add(
        fpr_mul(a_re, b_im),
        fpr_mul(a_im, b_re));
    (fpct_d_re, fpct_d_im)
}

pub fn fft(f: &mut [u64], logn: u32) {
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
            let (s_re, s_im): (u64, u64);
            s_re = FPR_GM_TAB[((m + i1) << 1) + 0];
            s_im = FPR_GM_TAB[((m + i1) << 1) + 1];
            j = j1;
            while j < j2 {
                let (x_re, x_im, mut y_re, mut y_im): (u64, u64, u64, u64);
                x_re = f[j];
                x_im = f[j + hn];
                y_re = f[j + ht];
                y_im = f[j + ht + hn];
                (y_re, y_im) = fpc_mul(y_re, y_im, s_re, s_im);
                (f[j], f[j + hn]) = fpc_add(x_re, x_im, y_re, y_im);
                (f[j + ht], f[j + ht + hn]) = fpc_sub(x_re, x_im, y_re, y_im);

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