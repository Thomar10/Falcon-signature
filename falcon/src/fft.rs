use crate::fpr::{fpr_add, fpr_double, FPR_GM_TAB, fpr_half, fpr_inv, fpr_mul, fpr_neg, FPR_P2_TAB, fpr_sqr, fpr_sub, FPR_ZERO};

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

fn fpc_div(a_re: u64, a_im: u64,
           mut b_re: u64, mut b_im: u64) -> (u64, u64) {
    let mut fpct_m = fpr_add(fpr_sqr(b_re), fpr_sqr(b_im));
    fpct_m = fpr_inv(fpct_m);
    b_re = fpr_mul(b_re, fpct_m);
    b_im = fpr_mul(fpr_neg(b_im), fpct_m);
    let fpct_d_re = fpr_sub(
        fpr_mul(a_re, b_re),
        fpr_mul(a_im, b_im));
    let fpct_d_im = fpr_add(
        fpr_mul(a_re, b_im),
        fpr_mul(a_im, b_re));
    (fpct_d_re, fpct_d_im)
}

fn fpc_sqr(a_re: u64, a_im: u64) -> (u64, u64) {
    let fpct_d_re = fpr_sub(fpr_sqr(a_re), fpr_sqr(a_im));
    let fpct_d_im = fpr_double(fpr_mul(a_re, a_im));
    (fpct_d_re, fpct_d_im)
}

fn fpc_inv(a_re: u64, a_im: u64) -> (u64, u64) {
    let mut fpct_m = fpr_add(fpr_sqr(a_re), fpr_sqr(a_im));
    fpct_m = fpr_inv(fpct_m);
    let fpct_d_re = fpr_mul(a_re, fpct_m);
    let fpct_d_im = fpr_mul(fpr_neg(a_im), fpct_m);
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

pub fn ifft(f: &mut [u64], logn: u32) {
    let (mut u, n, hn, mut t, mut m): (u32, usize, usize, usize, usize);
    n = (1 as usize) << logn;
    t = 1;
    m = n;
    hn = n >> 1;
    u = logn;
    while u > 1 {
        let (hm, dt, mut i1, mut j1): (usize, usize, usize, usize);
        hm = m >> 1;
        dt = t << 1;

        i1 = 0;
        j1 = 0;
        while j1 < hn {
            let (mut j, j2): (usize, usize);
            j2 = j1 + t;

            let (s_re, s_im): (u64, u64);
            s_re = FPR_GM_TAB[((hm + i1) << 1) + 0];
            s_im = fpr_neg(FPR_GM_TAB[((hm + i1) << 1) + 1]);
            j = j1;
            while j < j2 {
                let (mut x_re, mut x_im, y_re, y_im): (u64, u64, u64, u64);
                x_re = f[j];
                x_im = f[j + hn];
                y_re = f[j + t];
                y_im = f[j + t + hn];
                (f[j], f[j + hn]) = fpc_add(x_re, x_im, y_re, y_im);
                (x_re, x_im) = fpc_sub(x_re, x_im, y_re, y_im);
                (f[j + t], f[j + t + hn]) = fpc_mul(x_re, x_im, s_re, s_im);
                j += 1;
            }
            i1 += 1;
            j1 += dt;
        }
        t = dt;
        m = hm;
        u -= 1;
    }

    if logn > 0 {
        let ni = FPR_P2_TAB[logn as usize];
        let mut u = 0;
        while u < n {
            f[u] = fpr_mul(f[u], ni);
            u += 1;
        }
    }
}

pub fn poly_add(a: &mut [u64], b: &mut [u64], logn: u32) {
    let n: usize;

    n = (1 as usize) << logn;
    for u in 0..n {
        a[u] = fpr_add(a[u], b[u]);
    }
}

pub fn poly_sub(a: &mut [u64], b: &mut [u64], logn: u32) {
    let n: usize;

    n = (1 as usize) << logn;
    for u in 0..n {
        a[u] = fpr_sub(a[u], b[u]);
    }
}

pub fn poly_neg(a: &mut [u64], logn: u32) {
    let n: usize;

    n = (1 as usize) << logn;
    for u in 0..n {
        a[u] = fpr_neg(a[u]);
    }
}

pub fn poly_adj_fft(a: &mut [u64], logn: u32) {
    let n: usize;

    n = (1 as usize) << logn;
    for u in (n >> 1)..n {
        a[u] = fpr_neg(a[u]);
    }
}

pub fn poly_mul_fft(a: &mut [u64], b: &mut [u64], logn: u32) {
    let (n, hn): (usize, usize);

    n = (1 as usize) << logn;
    hn = n >> 1;
    for u in 0..hn {
        (a[u], a[u + hn]) = fpc_mul(a[u], a[u + hn], b[u], b[u + hn]);
    }
}

pub fn poly_muladj_fft(a: &mut [u64], b: &mut [u64], logn: u32) {
    let (n, hn): (usize, usize);

    n = (1 as usize) << logn;
    hn = n >> 1;
    for u in 0..hn {
        (a[u], a[u + hn]) = fpc_mul(a[u], a[u + hn], b[u], fpr_neg(b[u + hn]));
    }
}

pub fn poly_mulselfadj_fft(a: &mut [u64], logn: u32) {
    let (n, hn): (usize, usize);

    n = (1 as usize) << logn;
    hn = n >> 1;
    for u in 0..hn {
        a[u] = fpr_add(fpr_sqr(a[u]), fpr_sqr(a[u + hn]));
        a[u + hn] = FPR_ZERO;
    }
}

pub fn poly_mulconst(a: &mut [u64], x: u64, logn: u32) {
    let n: usize;

    n = (1 as usize) << logn;
    for u in 0..n {
        a[u] = fpr_mul(a[u], x);
    }
}

pub fn poly_div_fft(a: &mut [u64], b: &mut [u64], logn: u32) {
    let (n, hn): (usize, usize);

    n = (1 as usize) << logn;
    hn = n >> 1;
    for u in 0..hn {
        (a[u], a[u + hn]) = fpc_div(a[u], a[u + hn], b[u], b[u + hn]);
    }
}

pub fn poly_invnorm2_fft(d: &mut [u64], a: &mut [u64], b: &mut [u64], logn: u32) {
    let (n, hn): (usize, usize);

    n = (1 as usize) << logn;
    hn = n >> 1;

    for u in 0..hn {
        d[u] = fpr_inv(fpr_add(
            fpr_add(fpr_sqr(a[u]), fpr_sqr(a[u + hn])),
            fpr_add(fpr_sqr(b[u]), fpr_sqr(b[u + hn]))));
    }
}

#[allow(non_snake_case)]
pub fn poly_add_muladj_fft(d: &mut [u64], F: &mut [u64], G: &mut [u64], f: &mut [u64], g: &mut [u64], logn: u32) {
    let (n, hn): (usize, usize);
    n = (1 as usize) << logn;
    hn = n >> 1;
    for u in 0..hn {
        let (a_re, a_im) = fpc_mul(F[u], F[u + hn], f[u], fpr_neg(f[u + hn]));
        let (b_re, b_im) = fpc_mul(G[u], G[u + hn], g[u], fpr_neg(g[u + hn]));
        d[u] = fpr_add(a_re, b_re);
        d[u + hn] = fpr_add(a_im, b_im);
    }
}

pub fn poly_mul_autoadj_fft(a: &mut [u64], b: &mut [u64], logn: u32) {
    let (n, hn): (usize, usize);
    n = (1 as usize) << logn;
    hn = n >> 1;
    for u in 0..hn {
        a[u] = fpr_mul(a[u], b[u]);
        a[u + hn] = fpr_mul(a[u + hn], b[u]);
    }
}

pub fn poly_div_autoadj_fft(a: &mut [u64], b: &mut [u64], logn: u32) {
    let (n, hn): (usize, usize);
    n = (1 as usize) << logn;
    hn = n >> 1;
    for u in 0..hn {
        let ib = fpr_inv(b[u]);
        a[u] = fpr_mul(a[u], ib);
        a[u + hn] = fpr_mul(a[u + hn], ib);
    }
}

#[allow(non_snake_case)]
pub fn poly_LDL_fft(g00: &mut [u64], g01: &mut [u64], g11: &mut [u64], logn: u32) {
    let (n, hn): (usize, usize);
    n = (1 as usize) << logn;
    hn = n >> 1;
    for u in 0..hn {
        let g00_re = g00[u];
        let g00_im = g00[u + hn];
        let mut g01_re = g01[u];
        let mut g01_im = g01[u + hn];
        let g11_re = g11[u];
        let g11_im = g11[u + hn];
        let (mu_re, mu_im) = fpc_div(g01_re, g01_im, g00_re, g00_im);
        (g01_re, g01_im) = fpc_mul(mu_re, mu_im, g01_re, fpr_neg(g01_im));
        (g11[u], g11[u + hn]) = fpc_sub(g11_re, g11_im, g01_re, g01_im);
        g01[u] = mu_re;
        g01[u + hn] = fpr_neg(mu_im);
    }
}

#[allow(non_snake_case)]
pub fn poly_LDLmv_fft(d11: &mut [u64], l10: &mut [u64], g00: &mut [u64], g01: &mut [u64], g11: &mut [u64], logn: u32) {
    let (n, hn): (usize, usize);
    n = (1 as usize) << logn;
    hn = n >> 1;
    for u in 0..hn {
        let g00_re = g00[u];
        let g00_im = g00[u + hn];
        let mut g01_re = g01[u];
        let mut g01_im = g01[u + hn];
        let g11_re = g11[u];
        let g11_im = g11[u + hn];
        let (mu_re, mu_im) = fpc_div(g01_re, g01_im, g00_re, g00_im);
        (g01_re, g01_im) = fpc_mul(mu_re, mu_im, g01_re, fpr_neg(g01_im));
        (d11[u], d11[u + hn]) = fpc_sub(g11_re, g11_im, g01_re, g01_im);
        l10[u] = mu_re;
        l10[u + hn] = fpr_neg(mu_im);
    }
}

pub fn poly_split_fft(f0: &mut [u64], f1: &mut [u64], f: &mut [u64], logn: u32) {
    let (n, hn, qn): (usize, usize, usize);
    n = (1 as usize) << logn;
    hn = n >> 1;
    qn = hn >> 1;

    f0[0] = f[0];
    f1[0] = f[hn];
    for u in 0..qn {
        let (a_re, a_im, b_re, b_im): (u64, u64, u64, u64);

        a_re = f[(u << 1) + 0];
        a_im = f[(u << 1) + 0 + hn];
        b_re = f[(u << 1) + 1];
        b_im = f[(u << 1) + 1 + hn];

        let (mut t_re, mut t_im) = fpc_add(a_re, a_im, b_re, b_im);
        f0[u] = fpr_half(t_re);
        f0[u + qn] = fpr_half(t_im);

        (t_re, t_im) = fpc_sub(a_re, a_im, b_re, b_im);
        (t_re, t_im) = fpc_mul(t_re, t_im,
                               FPR_GM_TAB[((u + hn) << 1) + 0],
                               fpr_neg(FPR_GM_TAB[((u + hn) << 1) + 1]));
        f1[u] = fpr_half(t_re);
        f1[u + qn] = fpr_half(t_im);
    }
}

pub fn poly_merge_fft(f0: &mut [u64], f1: &mut [u64], f: &mut [u64], logn: u32) {
    let (n, hn, qn): (usize, usize, usize);
    n = (1 as usize) << logn;
    hn = n >> 1;
    qn = hn >> 1;

    f0[0] = f[0];
    f1[0] = f[hn];
    for u in 0..qn {
        let (a_re, a_im): (u64, u64);

        a_re = f0[u];
        a_im = f0[u + qn];
        let (b_re, b_im) = fpc_mul(f1[u], f1[u + qn],
                                   FPR_GM_TAB[((u + hn) << 1) + 0],
                                   FPR_GM_TAB[((u + hn) << 1) + 1]);
        let (mut t_re, mut t_im) = fpc_add(a_re, a_im, b_re, b_im);
        f[(u << 1) + 0] = t_re;
        f[(u << 1) + 0 + hn] = t_im;
        (t_re, t_im) = fpc_sub(a_re, a_im, b_re, b_im);
        f[(u << 1) + 1] = t_re;
        f[(u << 1) + 1 + hn] = t_im;
    }
}



