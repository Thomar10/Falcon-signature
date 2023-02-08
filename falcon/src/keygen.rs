use crate::fft::{fft, poly_adj_fft, poly_invnorm2_fft};
use crate::fpr::{fpr_add, fpr_mul, fpr_of, FPR_ONE, FPR_PTWO31, FPR_ZERO};
use crate::shake::{i_shake256_extract, InnerShake256Context};

pub struct SmallPrimes {
    pub(crate) p: u32,
    pub(crate) g: u32,
    pub(crate) s: u32,
}

pub struct BitLength {
    pub(crate) avg: i32,
    pub(crate) std: i32,
}

macro_rules! mkn {
    ($arg:expr) => {
        (1usize << ($arg))
    }
}

#[inline(always)]
pub fn modp_set(x: i32, p: u32) -> u32 {
    let w = x as u32;
    w.wrapping_add(p & (!(w >> 31)).wrapping_add(1))
}


#[inline(always)]
pub fn modp_norm(x: u32, p: u32) -> i32 {
    x.wrapping_sub(p & (((x.wrapping_sub((p + 1) >> 1)) >> 31).wrapping_sub(1))) as i32
}

#[inline(always)]
pub fn modp_ninv31(p: u32) -> u32 {
    let mut y = 2u32.wrapping_sub(p);
    y = y.wrapping_mul(2u32.wrapping_sub(p.wrapping_mul(y)));
    y = y.wrapping_mul(2u32.wrapping_sub(p.wrapping_mul(y)));
    y = y.wrapping_mul(2u32.wrapping_sub(p.wrapping_mul(y)));
    y = y.wrapping_mul(2u32.wrapping_sub(p.wrapping_mul(y)));
    0x7FFFFFFF & (!y).wrapping_add(1)
}

#[inline(always)]
#[allow(non_snake_case)]
pub fn modp_R(p: u32) -> u32 {
    (1u32 << 31).wrapping_sub(p)
}

#[inline(always)]
pub fn modp_add(a: u32, b: u32, p: u32) -> u32 {
    let d = a.wrapping_add(b).wrapping_sub(p);
    d.wrapping_add(p & (!(d >> 31)).wrapping_add(1))
}

#[inline(always)]
pub fn modp_sub(a: u32, b: u32, p: u32) -> u32 {
    let d = a.wrapping_sub(b);
    d.wrapping_add(p & (!(d >> 31)).wrapping_add(1))
}

#[inline(always)]
pub fn modp_montymul(a: u32, b: u32, p: u32, p0i: u32) -> u32 {
    let z = a as u64 * b as u64;
    let w = ((z.wrapping_mul(p0i as u64)) & 0x7FFFFFFF) * p as u64;
    let d = (((z.wrapping_add(w)) >> 31) as u32).wrapping_sub(p);
    d.wrapping_add(p & (!(d >> 31)).wrapping_add(1))
}

#[allow(non_snake_case)]
pub fn modp_R2(p: u32, p0i: u32) -> u32 {
    let mut z = modp_R(p);
    z = modp_add(z, z, p);


    z = modp_montymul(z, z, p, p0i);
    z = modp_montymul(z, z, p, p0i);
    z = modp_montymul(z, z, p, p0i);
    z = modp_montymul(z, z, p, p0i);
    z = modp_montymul(z, z, p, p0i);


    z = z.wrapping_add(p & (!(z & 1)).wrapping_add(1)) >> 1;
    z
}

#[allow(non_snake_case)]
pub fn modp_Rx(mut x: u32, p: u32, p0i: u32, R2: u32) -> u32 {
    x = x.wrapping_sub(1);
    let mut r = R2;
    let mut z = modp_R(p);
    let mut i: u32 = 0;
    while (1u32.wrapping_shl(i)) <= x {
        if (x & (1u32.wrapping_shl(i))) != 0 {
            z = modp_montymul(z, r, p, p0i);
        }
        r = modp_montymul(r, r, p, p0i);
        i += 1;
    }
    z
}

pub fn modp_div(a: u32, b: u32, p: u32, p0i: u32, r: u32) -> u32 {
    let e = p - 2;
    let mut z = r;
    for i in (0..=30).rev() {
        let z2;

        z = modp_montymul(z, z, p, p0i);
        z2 = modp_montymul(z, b, p, p0i);
        z ^= (z ^ z2) & (!((e.wrapping_shr(i)) & 1) as u32).wrapping_add(1);
    }

    z = modp_montymul(z, 1, p, p0i);
    modp_montymul(a, z, p, p0i)
}

pub fn modp_mkgm2(gm: &mut [u32], igm: &mut [u32], logn: u32, mut g: u32, p: u32, p0i: u32) {
    let n: usize = 1 << logn;
    let (mut x1, mut x2): (u32, u32);

    let r2 = modp_R2(p, p0i);
    g = modp_montymul(g, r2, p, p0i);
    for _ in logn..10 {
        g = modp_montymul(g, g, p, p0i);
    }

    let ig = modp_div(r2, g, p, p0i, modp_R(p));
    let k = 10 - logn;
    x2 = modp_R(p);
    x1 = x2.clone();
    for u in 0..n {
        let v: usize;

        v = REV10[u << k];
        gm[v] = x1;
        igm[v] = x2;
        x1 = modp_montymul(x1, g, p, p0i);
        x2 = modp_montymul(x2, ig, p, p0i);
    }
}

pub fn modp_mkgm2_index(tmp: &mut [u32], gm_index: usize, igm_index: usize, logn: u32, mut g: u32, p: u32, p0i: u32) {
    let n: usize = 1 << logn;
    let (mut x1, mut x2): (u32, u32);

    let r2 = modp_R2(p, p0i);
    g = modp_montymul(g, r2, p, p0i);
    for _ in logn..10 {
        g = modp_montymul(g, g, p, p0i);
    }

    let ig = modp_div(r2, g, p, p0i, modp_R(p));
    let k = 10 - logn;
    x2 = modp_R(p);
    x1 = x2.clone();
    for u in 0..n {
        let v: usize;

        v = REV10[u << k];
        tmp[v + gm_index] = x1;
        tmp[v + igm_index] = x2;
        x1 = modp_montymul(x1, g, p, p0i);
        x2 = modp_montymul(x2, ig, p, p0i);
    }
}

#[allow(non_snake_case)]
pub fn modp_NTT2_ext(a: &mut [u32], stride: usize, gm: &mut [u32], logn: u32, p: u32, p0i: u32) {
    if logn == 0 {
        return;
    }
    let n: usize = 1 << logn;
    let mut t = n;
    let mut m = 1;
    while m < n {
        let ht: usize;

        ht = t >> 1;
        let mut u = 0;
        let mut v1 = 0;
        while u < m {
            let s: u32 = gm[m + u];
            let mut r1Index = v1 * stride;
            let mut r2Index = ht * stride + r1Index;
            for _ in 0..ht {
                let (x, y): (u32, u32);
                x = a[r1Index];
                y = modp_montymul(a[r2Index], s, p, p0i);
                a[r1Index] = modp_add(x, y, p);
                a[r2Index] = modp_sub(x, y, p);
                r1Index += stride;
                r2Index += stride;
            }
            u += 1;
            v1 += t;
        }
        t = ht;
        m <<= 1;
    }
}

#[allow(non_snake_case)]
pub fn modp_NTT2_ext_index(a: &mut [u32], stride: usize, a_index: usize, gm_index: usize, logn: u32, p: u32, p0i: u32) {
    if logn == 0 {
        return;
    }
    let n: usize = 1 << logn;
    let mut t = n;
    let mut m = 1;
    while m < n {
        let ht: usize;

        ht = t >> 1;
        let mut u = 0;
        let mut v1 = 0;
        while u < m {
            let s: u32 = a[m + u + gm_index];
            let mut r1Index = v1 * stride;
            let mut r2Index = ht * stride + r1Index;
            for _ in 0..ht {
                let (x, y): (u32, u32);
                x = a[r1Index + a_index];
                y = modp_montymul(a[r2Index + a_index], s, p, p0i);
                a[r1Index + a_index] = modp_add(x, y, p);
                a[r2Index + a_index] = modp_sub(x, y, p);
                r1Index += stride;
                r2Index += stride;
            }
            u += 1;
            v1 += t;
        }
        t = ht;
        m <<= 1;
    }
}

#[allow(non_snake_case)]
pub fn modp_iNTT2_ext(a: &mut [u32], stride: usize, igm: &mut [u32], logn: u32, p: u32, p0i: u32) {
    if logn == 0 {
        return;
    }
    let n: usize = 1 << logn;
    let mut t = 1;
    let mut m = n;
    while m > 1 {
        let hm = m >> 1;
        let dt = t << 1;
        let mut u = 0;
        let mut v1 = 0;
        while u < hm {
            let s: u32 = igm[hm + u];
            let mut r1Index = v1 * stride;
            let mut r2Index = t * stride + r1Index;
            for _ in 0..t {
                let (x, y): (u32, u32);
                x = a[r1Index];
                y = a[r2Index];
                a[r1Index] = modp_add(x, y, p);
                a[r2Index] = modp_montymul(
                    modp_sub(x, y, p), s, p, p0i);
                r1Index += stride;
                r2Index += stride;
            }
            u += 1;
            v1 += dt;
        }
        t = dt;
        m >>= 1;
    }

    let ni: u32 = 1 << (31 - logn);
    let mut r: usize = 0;
    for _ in 0..n {
        a[r] = modp_montymul(a[r], ni, p, p0i);
        r += stride;
    }
}

#[allow(non_snake_case)]
pub fn modp_iNTT2_ext_index(a: &mut [u32], stride: usize, a_index: usize, igm_index: usize, logn: u32, p: u32, p0i: u32) {
    if logn == 0 {
        return;
    }
    let n: usize = 1 << logn;
    let mut t = 1;
    let mut m = n;
    while m > 1 {
        let hm = m >> 1;
        let dt = t << 1;
        let mut u = 0;
        let mut v1 = 0;
        while u < hm {
            let s: u32 = a[hm + u + igm_index];
            let mut r1Index = v1 * stride;
            let mut r2Index = t * stride + r1Index;
            for _ in 0..t {
                let (x, y): (u32, u32);
                x = a[r1Index + a_index];
                y = a[r2Index + a_index];
                a[r1Index + a_index] = modp_add(x, y, p);
                a[r2Index + a_index] = modp_montymul(
                    modp_sub(x, y, p), s, p, p0i);
                r1Index += stride;
                r2Index += stride;
            }
            u += 1;
            v1 += dt;
        }
        t = dt;
        m >>= 1;
    }

    let ni: u32 = 1 << (31 - logn);
    let mut r: usize = 0;
    for _ in 0..n {
        a[r + a_index] = modp_montymul(a[r + a_index], ni, p, p0i);
        r += stride;
    }
}


#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn modp_NTT2_index(a: &mut [u32], a_index: usize, gm_index: usize, logn: u32, p: u32, p0i: u32) {
    modp_NTT2_ext_index(a, 1, a_index, gm_index, logn, p, p0i);
}


#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn modp_NTT2(a: &mut [u32], gm: &mut [u32], logn: u32, p: u32, p0i: u32) {
    modp_NTT2_ext(a, 1, gm, logn, p, p0i);
}

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn modp_iNTT2(a: &mut [u32], igm: &mut [u32], logn: u32, p: u32, p0i: u32) {
    modp_iNTT2_ext(a, 1, igm, logn, p, p0i);
}

pub fn modp_poly_rec_res(f: &mut [u32], logn: u32, p: u32, p0i: u32, r2: u32) {
    let hn: usize = 1 << (logn - 1);
    for u in 0..hn {
        let w0 = f[(u << 1)];
        let w1 = f[(u << 1) + 1];
        f[u] = modp_montymul(modp_montymul(w0, w1, p, p0i), r2, p, p0i);
    }
}

pub fn zint_sub(a: &mut [u32], b: &mut [u32], len: usize, ctl: u32) -> u32 {
    let mut cc: u32 = 0;
    let m = (!ctl).wrapping_add(1);
    for u in 0..len {
        let mut aw = a[u];
        let w = aw.wrapping_sub(b[u]).wrapping_sub(cc);
        cc = w >> 31;
        aw ^= ((w & 0x7FFFFFFF) ^ aw) & m;
        a[u] = aw;
    }
    cc
}

pub fn zint_sub_index(a: &mut [u32], a_index: usize, b: &mut [u32], len: usize, ctl: u32) -> u32 {
    let mut cc: u32 = 0;
    let m = (!ctl).wrapping_add(1);
    for u in 0..len {
        let mut aw = a[u + a_index];
        let w = aw.wrapping_sub(b[u]).wrapping_sub(cc);
        cc = w >> 31;
        aw ^= ((w & 0x7FFFFFFF) ^ aw) & m;
        a[u] = aw;
    }
    cc
}

pub fn zint_sub_index_one_arr(a: &mut [u32], a_index: usize, b_index: usize, len: usize, ctl: u32) -> u32 {
    let mut cc: u32 = 0;
    let m = (!ctl).wrapping_add(1);
    for u in 0..len {
        let mut aw = a[u + a_index];
        let w = aw.wrapping_sub(a[u + b_index]).wrapping_sub(cc);
        cc = w >> 31;
        aw ^= ((w & 0x7FFFFFFF) ^ aw) & m;
        a[u + a_index] = aw;
    }
    cc
}

pub fn zint_mul_small(m: &mut [u32], mlen: usize, x: u32) -> u32 {
    let mut cc: u32 = 0;
    for u in 0..mlen {
        let z: u64 = (m[u] as u64) * (x as u64) + (cc as u64);
        m[u] = (z as u32) & 0x7FFFFFFF;
        cc = (z >> 31) as u32;
    }
    cc
}

pub fn zint_mul_small_index(m: &mut [u32], m_index: usize, mlen: usize, x: u32) -> u32 {
    let mut cc: u32 = 0;
    for u in 0..mlen {
        let z: u64 = (m[u + m_index] as u64) * (x as u64) + (cc as u64);
        m[u + m_index] = (z as u32) & 0x7FFFFFFF;
        cc = (z >> 31) as u32;
    }
    cc
}

pub fn zint_mod_small_unsigned(d: &mut [u32], dlen: usize, p: u32, p0i: u32, r2: u32) -> u32 {
    let mut x = 0;
    let mut u = dlen;
    while u > 0 {
        u -= 1;
        let mut w: u32;
        x = modp_montymul(x, r2, p, p0i);
        w = d[u].wrapping_sub(p);
        w = w.wrapping_add(p & (!(w >> 31)).wrapping_add(1));
        x = modp_add(x, w, p);
    }
    x
}


pub fn zint_mod_small_unsigned_index(d: &mut [u32], d_index: usize, dlen: usize, p: u32, p0i: u32, r2: u32) -> u32 {
    let mut x = 0;
    let mut u = dlen;
    while u > 0 {
        u -= 1;
        let mut w: u32;
        x = modp_montymul(x, r2, p, p0i);
        w = d[u + d_index].wrapping_sub(p);
        w = w.wrapping_add(p & (!(w >> 31)).wrapping_add(1));
        x = modp_add(x, w, p);
    }
    x
}


pub fn zint_mod_small_signed(d: &mut [u32], dlen: usize, p: u32, p0i: u32, r2: u32, rx: u32) -> u32 {
    if dlen == 0 {
        return 0;
    }
    let mut z = zint_mod_small_unsigned(d, dlen, p, p0i, r2);
    z = modp_sub(z, rx & (!(d[dlen - 1] >> 30)).wrapping_add(1), p);
    z
}

pub fn zint_mod_small_signed_index(d: &mut [u32], d_index: usize, dlen: usize, p: u32, p0i: u32, r2: u32, rx: u32) -> u32 {
    if dlen == 0 {
        return 0;
    }
    let mut z = zint_mod_small_unsigned_index(d, d_index, dlen, p, p0i, r2);
    z = modp_sub(z, rx & (!(d[dlen - 1 + d_index] >> 30)).wrapping_add(1), p);
    z
}

/*
 * Add y*s to x. x and y initially have length 'len' words; the new x
 * has length 'len+1' words. 's' must fit on 31 bits. x[] and y[] must
 * not overlap.
 */
pub fn zint_add_mul_small(x: &mut [u32], y: &mut [u32], len: usize, s: u32) {
    let mut cc = 0;
    for u in 0..len {
        let xw = x[u];
        let yw = y[u];
        let z = (yw as u64) * (s as u64) + (xw as u64) + (cc as u64);
        x[u] = (z as u32) & 0x7FFFFFFF;
        cc = (z >> 31) as u32;
    }
    x[len] = cc;
}

pub fn zint_add_mul_small_index(x: &mut [u32], x_index: usize, y: &mut [u32], len: usize, s: u32) {
    let mut cc = 0;
    for u in 0..len {
        let xw = x[u + x_index];
        let yw = y[u];
        let z = (yw as u64) * (s as u64) + (xw as u64) + (cc as u64);
        x[u + x_index] = (z as u32) & 0x7FFFFFFF;
        cc = (z >> 31) as u32;
    }
    x[len + x_index] = cc;
}

pub fn zint_add_mul_small_index_one_arr(x: &mut [u32], x_index: usize, y_index: usize, len: usize, s: u32) {
    let mut cc = 0;
    for u in 0..len {
        let xw = x[u + x_index];
        let yw = x[u + y_index];
        let z = (yw as u64) * (s as u64) + (xw as u64) + (cc as u64);
        x[u + x_index] = (z as u32) & 0x7FFFFFFF;
        cc = (z >> 31) as u32;
    }
    x[len + x_index] = cc;
}

pub fn zint_norm_zero(x: &mut [u32], p: &mut [u32], len: usize) {
    let mut u = len;
    let mut r: u32 = 0;
    let mut bb = 0;
    while u > 0 {
        u -= 1;
        let (wx, wp, mut cc): (u32, u32, u32);

        wx = x[u];
        wp = (p[u] >> 1) | (bb << 30);
        bb = p[u] & 1;

        cc = wp.wrapping_sub(wx);
        cc = ((!cc).wrapping_add(1) >> 31) | (!(cc >> 31)).wrapping_add(1);

        r |= cc & ((r & 1).wrapping_sub(1));
    }

    zint_sub(x, p, len, r >> 31);
}

pub fn zint_norm_zero_index(x: &mut [u32], x_index: usize, p: &mut [u32], len: usize) {
    let mut u = len;
    let mut r: u32 = 0;
    let mut bb = 0;
    while u > 0 {
        u -= 1;
        let (wx, wp, mut cc): (u32, u32, u32);

        wx = x[u + x_index];
        wp = (p[u] >> 1) | (bb << 30);
        bb = p[u] & 1;

        cc = wp.wrapping_sub(wx);
        cc = ((!cc).wrapping_add(1) >> 31) | (!(cc >> 31)).wrapping_add(1);

        r |= cc & ((r & 1).wrapping_sub(1));
    }


    zint_sub_index(x, x_index, p, len, r >> 31);
}

pub fn zint_norm_zero_index_one_arr(x: &mut [u32], x_index: usize, p_index: usize, len: usize) {
    let mut u = len;
    let mut r: u32 = 0;
    let mut bb = 0;
    while u > 0 {
        u -= 1;
        let (wx, wp, mut cc): (u32, u32, u32);

        wx = x[u + x_index];
        wp = (x[u + p_index] >> 1) | (bb << 30);
        bb = x[u + p_index] & 1;

        cc = wp.wrapping_sub(wx);
        cc = ((!cc).wrapping_add(1) >> 31) | (!(cc >> 31)).wrapping_add(1);

        r |= cc & ((r & 1).wrapping_sub(1));
    }


    zint_sub_index_one_arr(x, x_index, p_index, len, r >> 31);
}

/**
 * Rebuild integers from their RNS representation. There are 'num'
 * integers, and each consists in 'xlen' words. 'xx' points at that
 * first word of the first integer; subsequent integers are accessed
 * by adding 'xstride' repeatedly.
 */
#[allow(non_snake_case)]
pub fn zint_rebuild_CRT(xx: &mut [u32], xlen: usize, xstride: usize, num: u64, primes: &[SmallPrimes; 522], normalized_signed: bool, tmp: &mut [u32]) {
    let mut xIndex;
    tmp[0] = primes[0].p;
    for u in 1..xlen {
        let (p, p0i, s, r2): (u32, u32, u32, u32);

        p = primes[u].p;
        s = primes[u].s;

        p0i = modp_ninv31(p);
        r2 = modp_R2(p, p0i);
        xIndex = 0;
        for _ in 0..num {
            let (xp, xq, xr): (u32, u32, u32);

            xp = xx[xIndex + u];
            xq = zint_mod_small_unsigned_index(xx, xIndex, u, p, p0i, r2);
            xr = modp_montymul(s, modp_sub(xp, xq, p), p, p0i);
            zint_add_mul_small_index(xx, xIndex, tmp, u, xr);
            xIndex += xstride;
        }
        tmp[u] = zint_mul_small(tmp, u, p);
    }
    if normalized_signed {
        xIndex = 0;
        for _ in 0..num {
            zint_norm_zero_index(xx, xIndex, tmp, xlen);
            xIndex += xstride;
        }
    }
}

#[allow(non_snake_case)]
pub fn zint_rebuild_CRT_index(xx: &mut [u32], xx_index: usize, xlen: usize, xstride: usize, num: u64, primes: &[SmallPrimes; 522], normalized_signed: bool, tmp_index: usize) {
    let mut xIndex;
    xx[tmp_index] = primes[0].p;
    for u in 1..xlen {
        let (p, p0i, s, r2): (u32, u32, u32, u32);

        p = primes[u].p;
        s = primes[u].s;

        p0i = modp_ninv31(p);
        r2 = modp_R2(p, p0i);
        xIndex = xx_index;
        for _ in 0..num {
            let (xp, xq, xr): (u32, u32, u32);

            xp = xx[xIndex + u];
            xq = zint_mod_small_unsigned_index(xx, xIndex, u, p, p0i, r2);
            xr = modp_montymul(s, modp_sub(xp, xq, p), p, p0i);
            zint_add_mul_small_index_one_arr(xx, xIndex, tmp_index, u, xr);
            xIndex += xstride;
        }
        xx[u + tmp_index] = zint_mul_small_index(xx, tmp_index, u, p);
    }
    if normalized_signed {
        xIndex = xx_index;
        for _ in 0..num {
            zint_norm_zero_index_one_arr(xx, xIndex, tmp_index, xlen);
            xIndex += xstride;
        }
    }
}

pub fn zint_negate(a: &mut [u32], len: usize, ctl: u32) {
    let mut cc = ctl;
    let m = (!ctl).wrapping_add(1) >> 1;
    for u in 0..len {
        let mut aw;
        aw = a[u];
        aw = (aw ^ m).wrapping_add(cc);
        a[u] = aw & 0x7FFFFFFF;
        cc = aw >> 31;
    }
}

pub fn zint_negate_index(a: &mut [u32], a_index: usize, len: usize, ctl: u32) {
    let mut cc = ctl;
    let m = (!ctl).wrapping_add(1) >> 1;
    for u in 0..len {
        let mut aw;
        aw = a[u + a_index];
        aw = (aw ^ m).wrapping_add(cc);
        a[u + a_index] = aw & 0x7FFFFFFF;
        cc = aw >> 31;
    }
}

pub fn zint_co_reduce(a: &mut [u32], b: &mut [u32], len: usize, xa: i64, xb: i64, ya: i64, yb: i64) -> u32 {
    let mut cca: i64 = 0;
    let mut ccb: i64 = 0;
    for u in 0..len {
        let (wa, wb): (u32, u32);
        let (za, zb): (u64, u64);

        wa = a[u];
        wb = b[u];
        za = (wa as u64).wrapping_mul(xa as u64).wrapping_add((wb as u64).wrapping_mul(xb as u64)).wrapping_add(cca as u64);
        zb = (wa as u64).wrapping_mul(ya as u64).wrapping_add((wb as u64).wrapping_mul(yb as u64)).wrapping_add(ccb as u64);
        if u > 0 {
            a[u - 1] = (za as u32) & 0x7FFFFFFF;
            b[u - 1] = (zb as u32) & 0x7FFFFFFF;
        }
        cca = (za as i64) >> 31;
        ccb = (zb as i64) >> 31;
    }
    a[len - 1] = cca as u32;
    b[len - 1] = ccb as u32;

    let nega = ((cca as u64) >> 63) as u32;
    let negb = ((ccb as u64) >> 63) as u32;
    zint_negate(a, len, nega);
    zint_negate(b, len, negb);
    nega | (negb << 1)
}

pub fn zint_co_reduce_index(a: &mut [u32], a_index: usize, b_index: usize, len: usize, xa: i64, xb: i64, ya: i64, yb: i64) -> u32 {
    let mut cca: i64 = 0;
    let mut ccb: i64 = 0;
    for u in 0..len {
        let (wa, wb): (u32, u32);
        let (za, zb): (u64, u64);

        wa = a[u + a_index];
        wb = a[u + b_index];
        za = (wa as u64).wrapping_mul(xa as u64).wrapping_add((wb as u64).wrapping_mul(xb as u64)).wrapping_add(cca as u64);
        zb = (wa as u64).wrapping_mul(ya as u64).wrapping_add((wb as u64).wrapping_mul(yb as u64)).wrapping_add(ccb as u64);
        if u > 0 {
            a[u - 1 + a_index] = (za as u32) & 0x7FFFFFFF;
            a[u - 1 + b_index] = (zb as u32) & 0x7FFFFFFF;
        }
        cca = (za as i64) >> 31;
        ccb = (zb as i64) >> 31;
    }
    a[len - 1 + a_index] = cca as u32;
    a[len - 1 + b_index] = ccb as u32;

    let nega = ((cca as u64) >> 63) as u32;
    let negb = ((ccb as u64) >> 63) as u32;
    zint_negate_index(a, a_index, len, nega);
    zint_negate_index(a, b_index, len, negb);
    nega | (negb << 1)
}

pub fn zint_co_reduce_index_tmp(a: &mut [u32], a_index: usize, b_index: usize, tmp_index: usize, len: usize, xa: i64, xb: i64, ya: i64, yb: i64) -> u32 {
    let mut cca: i64 = 0;
    let mut ccb: i64 = 0;
    for u in 0..len {
        let (wa, wb): (u32, u32);
        let (za, zb): (u64, u64);

        wa = a[u + a_index];
        wb = a[u + b_index];
        za = (wa as u64).wrapping_mul(xa as u64).wrapping_add((wb as u64).wrapping_mul(xb as u64)).wrapping_add(cca as u64);
        zb = (wa as u64).wrapping_mul(ya as u64).wrapping_add((wb as u64).wrapping_mul(yb as u64)).wrapping_add(ccb as u64);
        if u > 0 {
            a[u - 1 + a_index] = (za as u32) & 0x7FFFFFFF;
            a[u - 1 + b_index] = (zb as u32) & 0x7FFFFFFF;
        }
        cca = (za as i64) >> 31;
        ccb = (zb as i64) >> 31;
    }
    a[len - 1 + a_index] = cca as u32;
    a[len - 1 + b_index] = ccb as u32;

    let nega = ((cca as u64) >> 63) as u32;
    let negb = ((ccb as u64) >> 63) as u32;
    zint_negate_index(a, a_index + tmp_index, len, nega);
    zint_negate_index(a, b_index + tmp_index, len, negb);
    nega | (negb << 1)
}

pub fn zint_finish_mod(a: &mut [u32], len: usize, m: &mut [u32], neg: u32) {
    let mut cc = 0;
    for u in 0..len {
        cc = (a[u].wrapping_sub(m[u]).wrapping_sub(cc)) >> 31;
    }


    let xm = (!neg).wrapping_add(1) >> 1;
    let ym = (!(neg | (1 - cc))).wrapping_add(1);
    cc = neg;
    for u in 0..len {
        let (mut aw, mw): (u32, u32);

        aw = a[u];
        mw = (m[u] ^ xm) & ym;
        aw = aw.wrapping_sub(mw).wrapping_sub(cc);
        a[u] = aw & 0x7FFFFFFF;
        cc = aw >> 31;
    }
}

pub fn zint_finish_mod_index_m(a: &mut [u32], a_index: usize, len: usize, m_index: usize, neg: u32) {
    let mut cc = 0;
    for u in 0..len {
        cc = (a[u + a_index].wrapping_sub(a[u + m_index]).wrapping_sub(cc)) >> 31;
    }


    let xm = (!neg).wrapping_add(1) >> 1;
    let ym = (!(neg | (1 - cc))).wrapping_add(1);
    cc = neg;
    for u in 0..len {
        let (mut aw, mw): (u32, u32);

        aw = a[u + a_index];
        mw = (a[u + m_index] ^ xm) & ym;
        aw = aw.wrapping_sub(mw).wrapping_sub(cc);
        a[u + a_index] = aw & 0x7FFFFFFF;
        cc = aw >> 31;
    }
}


pub fn zint_finish_mod_index(a: &mut [u32], a_index: usize, len: usize, m: &mut [u32], neg: u32) {
    let mut cc = 0;
    for u in 0..len {
        cc = (a[u + a_index].wrapping_sub(m[u]).wrapping_sub(cc)) >> 31;
    }


    let xm = (!neg).wrapping_add(1) >> 1;
    let ym = (!(neg | (1 - cc))).wrapping_add(1);
    cc = neg;
    for u in 0..len {
        let (mut aw, mw): (u32, u32);

        aw = a[u + a_index];
        mw = (m[u] ^ xm) & ym;
        aw = aw.wrapping_sub(mw).wrapping_sub(cc);
        a[u + a_index] = aw & 0x7FFFFFFF;
        cc = aw >> 31;
    }
}

pub fn zint_co_reduce_mod(a: &mut [u32], b: &mut [u32], m: &mut [u32], len: usize, m0i: u32, xa: i64, xb: i64, ya: i64, yb: i64) {
    let mut cca: i64 = 0;
    let mut ccb: i64 = 0;
    let fa = (a[0].wrapping_mul(xa as u32).wrapping_add(b[0].wrapping_mul(xb as u32)).wrapping_mul(m0i)) & 0x7FFFFFFF;
    let fb = (a[0].wrapping_mul(ya as u32).wrapping_add(b[0].wrapping_mul(yb as u32)).wrapping_mul(m0i)) & 0x7FFFFFFF;
    for u in 0..len {
        let (wa, wb): (u32, u32);
        let (za, zb): (u64, u64);

        wa = a[u];
        wb = b[u];

        za = (wa as u64).wrapping_mul(xa as u64).wrapping_add((wb as u64).wrapping_mul(xb as u64))
            .wrapping_add((m[u] as u64).wrapping_mul(fa as u64)).wrapping_add(cca as u64);
        zb = (wa as u64).wrapping_mul(ya as u64).wrapping_add((wb as u64).wrapping_mul(yb as u64))
            .wrapping_add((m[u] as u64).wrapping_mul(fb as u64)).wrapping_add(ccb as u64);

        if u > 0 {
            a[u - 1] = (za as u32) & 0x7FFFFFFF;
            b[u - 1] = (zb as u32) & 0x7FFFFFFF;
        }
        cca = (za as i64) >> 31;
        ccb = (zb as i64) >> 31;
    }
    a[len - 1] = cca as u32;
    b[len - 1] = ccb as u32;


    zint_finish_mod(a, len, m, ((cca as u64) >> 63) as u32);
    zint_finish_mod(b, len, m, ((ccb as u64) >> 63) as u32);
}


pub fn zint_co_reduce_mod_index(a: &mut [u32], a_index: usize, b: &mut [u32], b_index: usize, m: &mut [u32], len: usize, m0i: u32, xa: i64, xb: i64, ya: i64, yb: i64) {
    let mut cca: i64 = 0;
    let mut ccb: i64 = 0;
    let fa = (a[a_index].wrapping_mul(xa as u32).wrapping_add(b[b_index].wrapping_mul(xb as u32)).wrapping_mul(m0i)) & 0x7FFFFFFF;
    let fb = (a[a_index].wrapping_mul(ya as u32).wrapping_add(b[b_index].wrapping_mul(yb as u32)).wrapping_mul(m0i)) & 0x7FFFFFFF;
    for u in 0..len {
        let (wa, wb): (u32, u32);
        let (za, zb): (u64, u64);

        wa = a[u + a_index];
        wb = b[u + b_index];

        za = (wa as u64).wrapping_mul(xa as u64).wrapping_add((wb as u64).wrapping_mul(xb as u64))
            .wrapping_add((m[u] as u64).wrapping_mul(fa as u64)).wrapping_add(cca as u64);
        zb = (wa as u64).wrapping_mul(ya as u64).wrapping_add((wb as u64).wrapping_mul(yb as u64))
            .wrapping_add((m[u] as u64).wrapping_mul(fb as u64)).wrapping_add(ccb as u64);

        if u > 0 {
            a[u - 1 + a_index] = (za as u32) & 0x7FFFFFFF;
            b[u - 1 + b_index] = (zb as u32) & 0x7FFFFFFF;
        }
        cca = (za as i64) >> 31;
        ccb = (zb as i64) >> 31;
    }
    a[len - 1 + a_index] = cca as u32;
    b[len - 1 + b_index] = ccb as u32;


    zint_finish_mod_index(a, a_index, len, m, ((cca as u64) >> 63) as u32);
    zint_finish_mod_index(b, b_index, len, m, ((ccb as u64) >> 63) as u32);
}

pub fn zint_co_reduce_mod_index_tmp(a: &mut [u32], a_index: usize, b_index: usize, m_index: usize, len: usize, m0i: u32, xa: i64, xb: i64, ya: i64, yb: i64) {
    let mut cca: i64 = 0;
    let mut ccb: i64 = 0;
    let fa = (a[a_index].wrapping_mul(xa as u32).wrapping_add(a[b_index].wrapping_mul(xb as u32)).wrapping_mul(m0i)) & 0x7FFFFFFF;
    let fb = (a[a_index].wrapping_mul(ya as u32).wrapping_add(a[b_index].wrapping_mul(yb as u32)).wrapping_mul(m0i)) & 0x7FFFFFFF;
    for u in 0..len {
        let (wa, wb): (u32, u32);
        let (za, zb): (u64, u64);

        wa = a[u + a_index];
        wb = a[u + b_index];

        za = (wa as u64).wrapping_mul(xa as u64).wrapping_add((wb as u64).wrapping_mul(xb as u64))
            .wrapping_add((a[u + m_index] as u64).wrapping_mul(fa as u64)).wrapping_add(cca as u64);
        zb = (wa as u64).wrapping_mul(ya as u64).wrapping_add((wb as u64).wrapping_mul(yb as u64))
            .wrapping_add((a[u + m_index] as u64).wrapping_mul(fb as u64)).wrapping_add(ccb as u64);

        if u > 0 {
            a[u - 1 + a_index] = (za as u32) & 0x7FFFFFFF;
            a[u - 1 + b_index] = (zb as u32) & 0x7FFFFFFF;
        }
        cca = (za as i64) >> 31;
        ccb = (zb as i64) >> 31;
    }
    a[len - 1 + a_index] = cca as u32;
    a[len - 1 + b_index] = ccb as u32;


    zint_finish_mod_index_m(a, a_index, len, m_index, ((cca as u64) >> 63) as u32);
    zint_finish_mod_index_m(a, b_index, len, m_index, ((ccb as u64) >> 63) as u32);
}

pub fn zint_bezout_index(data: &mut [u32], u_index: usize, v_index: usize, x_index: usize, y_index: usize, len: usize, tmp_index: usize) -> bool {
    let (u1_index, v1_index, a_index, b_index): (usize, usize, usize, usize);
    let (x0i, y0i): (u32, u32);
    let (mut num, mut rc): (u32, u32);
    let mut j: usize;

    if len == 0 {
        return false;
    }


    u1_index = tmp_index;
    v1_index = u1_index + len;
    a_index = v1_index + len;
    b_index = a_index + len;


    x0i = modp_ninv31(data[x_index]);
    y0i = modp_ninv31(data[y_index]);

    data.copy_within(x_index..x_index + len, a_index);
    data.copy_within(y_index..y_index + len, b_index);
    data[u_index] = 1;
    data[(1 + u_index)..(1 + u_index + len - 1)].fill(0);
    data[v_index..(v_index + len)].fill(0);
    data.copy_within(y_index..y_index + len, u1_index);
    data.copy_within(x_index..x_index + len, v1_index);
    data[v1_index] -= 1;


    num = 62 * (len as u32) + 30;
    while num >= 30 {
        let (mut c0, mut c1): (u32, u32);
        let (mut a0, mut a1, mut b0, mut b1): (u32, u32, u32, u32);
        let (mut a_hi, mut b_hi): (u64, u64);
        let (mut a_lo, mut b_lo): (u32, u32);
        let (mut pa, mut pb, mut qa, mut qb): (i64, i64, i64, i64);
        let r: u32;


        c0 = (!1u32).wrapping_add(1);
        c1 = (!1u32).wrapping_add(1);
        a0 = 0;
        a1 = 0;
        b0 = 0;
        b1 = 0;
        j = len;
        while j > 0 {
            j -= 1;
            let (aw, bw): (u32, u32);

            aw = data[a_index + j];
            bw = data[b_index + j];
            a0 ^= (a0 ^ aw) & c0;
            a1 ^= (a1 ^ aw) & c1;
            b0 ^= (b0 ^ bw) & c0;
            b1 ^= (b1 ^ bw) & c1;
            c1 = c0;
            c0 &= (((aw | bw).wrapping_add(0x7FFFFFFF)) >> 31).wrapping_sub(1u32);
        }


        a1 |= a0 & c1;
        a0 &= !c1;
        b1 |= b0 & c1;
        b0 &= !c1;
        a_hi = ((a0 as u64) << 31) + a1 as u64;
        b_hi = ((b0 as u64) << 31) + b1 as u64;
        a_lo = data[a_index];
        b_lo = data[b_index];

        pa = 1;
        pb = 0;
        qa = 0;
        qb = 1;
        for i in 0..31 {
            let (rt, oa, ob, c_ab, c_ba, c_a): (u32, u32, u32, u32, u32, u32);
            let rz: u64;

            rz = b_hi.wrapping_sub(a_hi);
            rt = ((rz ^ ((a_hi ^ b_hi)
                & (a_hi ^ rz))) >> 63) as u32;

            oa = (a_lo >> i) & 1;
            ob = (b_lo >> i) & 1;
            c_ab = oa & ob & rt;
            c_ba = oa & ob & !rt;
            c_a = c_ab | (oa ^ 1);

            a_lo = a_lo.wrapping_sub(b_lo & (!c_ab).wrapping_add(1));
            a_hi = a_hi.wrapping_sub(b_hi & (!(c_ab as u64)).wrapping_add(1));
            pa -= qa & -(c_ab as i64);
            pb -= qb & -(c_ab as i64);
            b_lo = b_lo.wrapping_sub(a_lo & (!c_ba).wrapping_add(1));
            b_hi = b_hi.wrapping_sub(a_hi & (!(c_ba as u64)).wrapping_add(1));
            qa -= pa & -(c_ba as i64);
            qb -= pb & -(c_ba as i64);

            a_lo = a_lo.wrapping_add(a_lo & (c_a.wrapping_sub(1)));
            pa += pa & ((c_a as i64) - 1);
            pb += pb & ((c_a as i64) - 1);
            a_hi ^= (a_hi ^ (a_hi >> 1)) & (!(c_a as u64)).wrapping_add(1);
            b_lo = b_lo.wrapping_add(b_lo & (!c_a).wrapping_add(1));
            qa += qa & -(c_a as i64);
            qb += qb & -(c_a as i64);
            b_hi ^= (b_hi ^ (b_hi >> 1)) & ((c_a as u64).wrapping_sub(1));
        }

        r = zint_co_reduce_index_tmp(data, a_index, b_index, tmp_index, len, pa, pb, qa, qb);
        pa -= (pa + pa) & -((r & 1) as i64);
        pb -= (pb + pb) & -((r & 1) as i64);
        qa -= (qa + qa) & -((r >> 1) as i64);
        qb -= (qb + qb) & -((r >> 1) as i64);

        zint_co_reduce_mod_index_tmp(data, u_index, u1_index, y_index, len, y0i, pa, pb, qa, qb);
        zint_co_reduce_mod_index_tmp(data, v_index, v1_index, x_index, len, x0i, pa, pb, qa, qb);

        num -= 30;
    }


    rc = data[a_index] ^ 1;
    for j in 1..len {
        rc |= data[j + a_index];
    }
    ((1 - ((rc | (!rc).wrapping_add(1)) >> 31)) & data[x_index] & data[y_index]) != 0
}


pub fn zint_bezout(u: &mut [u32], v: &mut [u32], x: &mut [u32], y: &mut [u32], len: usize, tmp: &mut [u32]) -> i32 {
    let (u1_index, v1_index, a_index, b_index): (usize, usize, usize, usize);
    let (x0i, y0i): (u32, u32);
    let (mut num, mut rc): (u32, u32);
    let mut j: usize;

    if len == 0 {
        return 0;
    }


    u1_index = 0;
    v1_index = len;
    a_index = 2 * len;
    b_index = 3 * len;


    x0i = modp_ninv31(x[0]);
    y0i = modp_ninv31(y[0]);

    tmp[a_index..b_index].copy_from_slice(&x);
    tmp[b_index..(b_index + len)].copy_from_slice(&y);
    u[0] = 1;
    u[1..len].fill(0);
    v.fill(0);
    tmp[u1_index..v1_index].copy_from_slice(&y);
    tmp[v1_index..a_index].copy_from_slice(&x);
    tmp[v1_index] -= 1;


    num = 62 * (len as u32) + 30;
    while num >= 30 {
        let (mut c0, mut c1): (u32, u32);
        let (mut a0, mut a1, mut b0, mut b1): (u32, u32, u32, u32);
        let (mut a_hi, mut b_hi): (u64, u64);
        let (mut a_lo, mut b_lo): (u32, u32);
        let (mut pa, mut pb, mut qa, mut qb): (i64, i64, i64, i64);
        let r: u32;


        c0 = (!1u32).wrapping_add(1);
        c1 = (!1u32).wrapping_add(1);
        a0 = 0;
        a1 = 0;
        b0 = 0;
        b1 = 0;
        j = len;
        while j > 0 {
            j -= 1;
            let (aw, bw): (u32, u32);

            aw = tmp[a_index + j];
            bw = tmp[b_index + j];
            a0 ^= (a0 ^ aw) & c0;
            a1 ^= (a1 ^ aw) & c1;
            b0 ^= (b0 ^ bw) & c0;
            b1 ^= (b1 ^ bw) & c1;
            c1 = c0;
            c0 &= (((aw | bw).wrapping_add(0x7FFFFFFF)) >> 31).wrapping_sub(1u32);
        }


        a1 |= a0 & c1;
        a0 &= !c1;
        b1 |= b0 & c1;
        b0 &= !c1;
        a_hi = ((a0 as u64) << 31) + a1 as u64;
        b_hi = ((b0 as u64) << 31) + b1 as u64;
        a_lo = tmp[a_index];
        b_lo = tmp[b_index];


        pa = 1;
        pb = 0;
        qa = 0;
        qb = 1;
        for i in 0..31 {
            let (rt, oa, ob, c_ab, c_ba, c_a): (u32, u32, u32, u32, u32, u32);
            let rz: u64;

            rz = b_hi.wrapping_sub(a_hi);
            rt = ((rz ^ ((a_hi ^ b_hi)
                & (a_hi ^ rz))) >> 63) as u32;

            oa = (a_lo >> i) & 1;
            ob = (b_lo >> i) & 1;
            c_ab = oa & ob & rt;
            c_ba = oa & ob & !rt;
            c_a = c_ab | (oa ^ 1);

            a_lo = a_lo.wrapping_sub(b_lo & (!c_ab).wrapping_add(1));
            a_hi = a_hi.wrapping_sub(b_hi & (!(c_ab as u64)).wrapping_add(1));
            pa -= qa & -(c_ab as i64);
            pb -= qb & -(c_ab as i64);
            b_lo = b_lo.wrapping_sub(a_lo & (!c_ba).wrapping_add(1));
            b_hi = b_hi.wrapping_sub(a_hi & (!(c_ba as u64)).wrapping_add(1));
            qa -= pa & -(c_ba as i64);
            qb -= pb & -(c_ba as i64);

            a_lo = a_lo.wrapping_add(a_lo & (c_a.wrapping_sub(1)));
            pa += pa & ((c_a as i64) - 1);
            pb += pb & ((c_a as i64) - 1);
            a_hi ^= (a_hi ^ (a_hi >> 1)) & (!(c_a as u64)).wrapping_add(1);
            b_lo = b_lo.wrapping_add(b_lo & (!c_a).wrapping_add(1));
            qa += qa & -(c_a as i64);
            qb += qb & -(c_a as i64);
            b_hi ^= (b_hi ^ (b_hi >> 1)) & ((c_a as u64).wrapping_sub(1));
        }


        r = zint_co_reduce_index(tmp, a_index, b_index, len, pa, pb, qa, qb);
        pa -= (pa + pa) & -((r & 1) as i64);
        pb -= (pb + pb) & -((r & 1) as i64);
        qa -= (qa + qa) & -((r >> 1) as i64);
        qb -= (qb + qb) & -((r >> 1) as i64);

        zint_co_reduce_mod_index(u, 0, tmp, u1_index, y, len, y0i, pa, pb, qa, qb);
        zint_co_reduce_mod_index(v, 0, tmp, v1_index, x, len, x0i, pa, pb, qa, qb);

        num -= 30;
    }


    rc = tmp[a_index] ^ 1;
    for j in 1..len {
        rc |= tmp[j + a_index];
    }
    ((1 - ((rc | (!rc).wrapping_add(1)) >> 31)) & x[0] & y[0]) as i32
}

pub fn zint_add_scaled_mul_small(x: &mut [u32], xlen: usize, y: &mut [u32], ylen: usize, k: i32, sch: usize, scl: u32) {
    let mut cc: i32;
    let mut tw: u32;
    if ylen == 0 {
        return;
    }
    let ysign = (!(y[ylen - 1] >> 30)).wrapping_add(1) >> 1;
    tw = 0;
    cc = 0;
    for u in sch..xlen {
        let v: usize = u - sch;
        let wy = if v < ylen { y[v] } else { ysign };
        let wys: u32 = ((wy << scl) & 0x7FFFFFFF) | tw;
        tw = wy >> (31 - scl);

        let z = ((wys as i64) * (k as i64) + (x[u] as i64) + (cc as i64)) as u64;
        x[u] = (z as u32) & 0x7FFFFFFF;

        let ccu = (z >> 31) as u32;
        cc = ccu as i32;
    }
}

pub fn zint_add_scaled_mul_small_index(x: &mut [u32], x_index: usize, xlen: usize, y: &mut [u32], y_index: usize, ylen: usize, k: i32, sch: usize, scl: u32) {
    let mut cc: i32;
    let mut tw: u32;
    if ylen == 0 {
        return;
    }
    let ysign = (!(y[ylen - 1 + y_index] >> 30)).wrapping_add(1) >> 1;
    tw = 0;
    cc = 0;
    for u in sch..xlen {
        let v: usize = u - sch;
        let wy = if v < ylen { y[v + y_index] } else { ysign };
        let wys: u32 = ((wy << scl) & 0x7FFFFFFF) | tw;
        tw = wy >> (31 - scl);

        let z = ((wys as i64) * (k as i64) + (x[u + x_index] as i64) + (cc as i64)) as u64;
        x[u + x_index] = (z as u32) & 0x7FFFFFFF;

        let ccu = (z >> 31) as u32;
        cc = ccu as i32;
    }
}

pub fn zint_sub_scaled(x: &mut [u32], xlen: usize, y: &mut [u32], ylen: usize, sch: usize, scl: u32) {
    if ylen == 0 {
        return;
    }

    let ysign: u32 = (!(y[ylen - 1] >> 30)).wrapping_add(1) >> 1;
    let mut tw = 0;
    let mut cc = 0;
    for u in sch..xlen {
        let v: usize;
        let (w, wy, wys): (u32, u32, u32);

        /*
         * Get the next word of y (scaled).
         */
        v = (u as usize) - sch;
        wy = if v < ylen { y[v] } else { ysign };
        wys = ((wy << scl) & 0x7FFFFFFF) | tw;
        tw = wy >> (31 - scl);

        w = x[u] - wys - cc;
        x[u] = w & 0x7FFFFFFF;
        cc = w >> 31;
    }
}

pub fn zint_sub_scaled_index(x: &mut [u32], x_index: usize, xlen: usize, y: &mut [u32], y_index: usize, ylen: usize, sch: usize, scl: u32) {
    if ylen == 0 {
        return;
    }

    let ysign: u32 = (!(y[ylen - 1 + y_index] >> 30)).wrapping_add(1) >> 1;
    let mut tw = 0;
    let mut cc = 0;
    for u in sch..xlen {
        let v: usize;
        let (w, wy, wys): (u32, u32, u32);

        /*
         * Get the next word of y (scaled).
         */
        v = (u as usize) - sch;
        wy = if v < ylen { y[v + y_index] } else { ysign };
        wys = ((wy << scl) & 0x7FFFFFFF) | tw;
        tw = wy >> (31 - scl);

        w = x[u + x_index] - wys - cc;
        x[u + x_index] = w & 0x7FFFFFFF;
        cc = w >> 31;
    }
}

pub fn zint_one_to_plain(x: &mut [u32]) -> i32 {
    let mut w: u32 = x[0];
    w |= (w & 0x40000000) << 1;
    w as i32
}

pub fn zint_one_to_plain_index(x: &mut [u32], index: usize) -> i32 {
    let mut w: u32 = x[index];
    w |= (w & 0x40000000) << 1;
    w as i32
}


pub fn poly_big_to_fp(d: &mut [u64], f: &mut [u32], flen: usize, fstride: usize, logn: u32) {
    let n = mkn!(logn);
    if flen == 0 {
        for u in 0..n {
            d[u] = FPR_ZERO;
        }
        return;
    }
    let mut f_index = 0;
    for u in 0..n {
        let neg = (!(f[flen - 1 + f_index] >> 30)).wrapping_add(1);
        let xm = neg >> 1;
        let mut cc = neg & 1;
        let mut x: u64 = FPR_ZERO;
        let mut fsc: u64 = FPR_ONE;
        for v in 0..flen {
            let mut w: u32;

            w = (f[f_index + v] ^ xm) + cc;
            cc = w >> 31;
            w &= 0x7FFFFFFF;
            w = w.wrapping_sub((w << 1) & neg);
            x = fpr_add(x, fpr_mul(fpr_of(w as i32 as i64), fsc));
            fsc = fpr_mul(fsc, FPR_PTWO31);
        }
        f_index += fstride;
        d[u] = x;
    }
}

pub fn poly_big_to_small(d: &mut [i8], s: &mut [u32], lim: i32, logn: u32) -> i32 {
    let n = mkn!(logn);
    for u in 0..n {
        let z = zint_one_to_plain_index(s, u);
        if z < -lim || z > lim {
            return 0;
        }
        d[u] = z as i8;
    }
    1
}

#[allow(non_snake_case)]
pub fn poly_sub_scaled(F: &mut [u32], Flen: usize, Fstride: usize, f: &mut [u32], flen: usize, fstride: usize, k: &mut [i32], sch: u32, scl: u32, logn: u32) {
    let n = mkn!(logn);
    for u in 0..n {
        let mut kf: i32;
        let mut x_index: usize;
        let mut y_index: usize;

        kf = -k[u];
        x_index = u * Fstride;
        y_index = 0;
        for v in 0..n {
            zint_add_scaled_mul_small_index(
                F, x_index, Flen, f, y_index, flen, kf, sch as usize, scl);
            if u + v == n - 1 {
                x_index = 0;
                kf = -kf;
            } else {
                x_index += Fstride;
            }
            y_index += fstride;
        }
    }
}

#[allow(non_snake_case)]
pub fn poly_sub_scaled_ntt(F: &mut [u32], Flen: usize, Fstride: usize, f: &mut [u32], flen: usize, fstride: usize, k: &mut [i32], sch: u32, scl: u32, logn: u32, tmp: &mut [u32]) {
    let (gm_index, igm_index, fk_index, t1_index, mut x_index, mut y_index): (usize, usize, usize, usize, usize, usize);
    let tlen: usize;
    let n = mkn!(logn);
    tlen = flen + 1;
    gm_index = 0;
    igm_index = n;
    fk_index = 2 * n;
    t1_index = 2 * n + n * tlen;

    for u in 0..tlen {
        let (p, p0i, r2, rx): (u32, u32, u32, u32);

        p = PRIMES[u].p;
        p0i = modp_ninv31(p);
        r2 = modp_R2(p, p0i);
        rx = modp_Rx(flen as u32, p, p0i, r2);
        modp_mkgm2_index(tmp, gm_index, igm_index, logn, PRIMES[u].g, p, p0i);

        for v in 0..n {
            tmp[v + t1_index] = modp_set(k[v], p);
        }
        modp_NTT2_index(tmp, t1_index, gm_index, logn, p, p0i);
        y_index = 0;
        x_index = fk_index + u;
        for _ in 0..n {
            tmp[x_index] = zint_mod_small_signed_index(f, y_index, flen, p, p0i, r2, rx);
            y_index += fstride;
            x_index += tlen;
        }
        modp_NTT2_ext_index(tmp, tlen, fk_index + u, gm_index, logn, p, p0i);
        for v in 0..n {
            tmp[x_index] = modp_montymul(
                modp_montymul(tmp[v + t1_index], tmp[x_index], p, p0i), r2, p, p0i);
            x_index += tlen;
        }
        modp_iNTT2_ext_index(tmp, tlen, fk_index + u, igm_index, logn, p, p0i);
    }
    zint_rebuild_CRT_index(tmp, fk_index, tlen, tlen, n as u64, &PRIMES, true, t1_index);
    x_index = 0;
    y_index = fk_index;
    for _ in 0..n {
        zint_sub_scaled_index(F, x_index, Flen, tmp, y_index, tlen, sch as usize, scl);
        x_index += Fstride;
        y_index += tlen;
    }
}

#[inline(always)]
pub fn get_rng_u64(rng: &mut InnerShake256Context) -> u64 {
    let tmp = i_shake256_extract(rng, 8);
    (tmp[0] as u64)
        | ((tmp[1] as u64) << 8)
        | ((tmp[2] as u64) << 16)
        | ((tmp[3] as u64) << 24)
        | ((tmp[4] as u64) << 32)
        | ((tmp[5] as u64) << 40)
        | ((tmp[6] as u64) << 48)
        | ((tmp[7] as u64) << 56)
}

pub fn mkgauss(rng: &mut InnerShake256Context, logn: u32) -> i32 {
    let g: u32 = 1u32.wrapping_shl(10u32.wrapping_sub(logn));
    let mut val: i32 = 0;
    for _ in 0..g {
        let mut r: u64;
        let (mut f, mut v, neg): (u32, u32, u32);

        r = get_rng_u64(rng);
        neg = (r >> 63) as u32;
        r &= !(1u64 << 63);
        f = ((r.wrapping_sub(GAUSS_1024_12289[0])) >> 63) as u32;

        v = 0;
        r = get_rng_u64(rng);
        r &= !(1u64 << 63);
        for k in 1..GAUSS_1024_12289.len() {
            let t: u32 = (((r.wrapping_sub(GAUSS_1024_12289[k])) >> 63) ^ 1) as u32;
            v |= (k as u32) & (!(t & (f ^ 1))).wrapping_add(1);
            f |= t;
        }

        v = (v ^ (!neg).wrapping_add(1)).wrapping_add(neg);

        val += v as i32;
    }
    val
}

pub fn poly_small_sqnorm(f: &mut [i8], logn: u32) -> u32 {
    let n = mkn!(logn);
    let mut s = 0;
    let mut ng = 0;
    for u in 0..n {
        let z: i32 = f[u] as i32;
        s += z.wrapping_mul(z) as u32;
        ng |= s;
    }
    s | (!(ng >> 31)).wrapping_add(1)
}

// pub fn align_fpr(base: &mut [()], data: &mut [()]) -> &mut [u64] {
//     panic!("TISSEMAND")
// }
//
// pub fn align_u32(base: &mut [()], data: &mut [()]) -> &mut [u32] {
//     panic!("TISSEMAND")
// }

pub fn poly_small_to_fp(x: &mut [u64], f: &mut [i8], logn: u32) {
    let n = mkn!(logn);
    for u in 0..n {
        x[u] = fpr_of(f[u] as i64);
    }
}

// TODO USE SPLIT_AT_MUT INSTEAD
pub fn make_fg_step_index(data: &mut [u32], data_index: usize, logn: u32, depth: usize, in_ntt: bool, out_ntt: bool) {
    let n = 1usize << logn;
    let hn = n >> 1;
    let slen = MAX_BL_SMALL[depth];
    let tlen = MAX_BL_SMALL[depth + 1];

    let fd_index = data_index;
    let gd_index = fd_index + hn * tlen;
    let fs_index = gd_index + hn * tlen;
    let gs_index = fs_index + n * slen;
    let gm_index = gs_index + n * slen;
    let igm_index = gm_index + n;
    let t1_index = igm_index + n;
    data[data_index..].copy_within(0..(2 * n * slen), 2 * hn * tlen);

    for u in 0..slen {
        let (p, p0i, r2): (u32, u32, u32);
        let mut x_index: usize;

        p = PRIMES[u].p;
        p0i = modp_ninv31(p);
        r2 = modp_R2(p, p0i);
        // let (_, gm_inter) = data.split_at_mut(gm_index);
        // let (gm, igm) = data[gm_index..].split_at_mut(gm_index - 1);
        // modp_mkgm2(gm, igm, logn, PRIMES[u].g, p, p0i);
        modp_mkgm2_index(data, gm_index, igm_index, logn, PRIMES[u].g, p, p0i);
        x_index = fs_index + u;
        for v in 0..n {
            data[t1_index + v] = data[x_index];
            x_index += slen;
        }
        if !in_ntt {
            modp_NTT2_index(data, t1_index, gm_index, logn, p, p0i);
        }
        x_index = fd_index + u;
        for v in 0..hn {
            let w0: u32 = data[t1_index + (v << 1)];
            let w1: u32 = data[t1_index + (v << 1) + 1];
            data[x_index] = modp_montymul(
                modp_montymul(w0, w1, p, p0i), r2, p, p0i);
            x_index += tlen;
        }
        if in_ntt {
            modp_iNTT2_ext_index(data, slen, fs_index + u, igm_index, logn, p, p0i);
        }

        x_index = gs_index + u;
        for v in 0..n {
            data[t1_index + v] = data[x_index];
            x_index += slen;
        }
        if !in_ntt {
            modp_NTT2_index(data, t1_index, gm_index, logn, p, p0i);
        }

        x_index = gd_index + u;
        for v in 0..hn {
            let w0 = data[t1_index + (v << 1)];
            let w1 = data[t1_index + (v << 1) + 1];
            data[x_index] = modp_montymul(
                modp_montymul(w0, w1, p, p0i), r2, p, p0i);
            x_index += tlen;
        }
        if in_ntt {
            modp_iNTT2_ext_index(data, slen, gs_index + u, igm_index, logn, p, p0i);
        }

        if !out_ntt {
            modp_iNTT2_ext_index(data, tlen, fd_index + u, igm_index, logn - 1, p, p0i);
            modp_iNTT2_ext_index(data, tlen, gd_index + u, igm_index, logn - 1, p, p0i);
        }
    }
    zint_rebuild_CRT_index(data, fs_index, slen, slen, n as u64, &PRIMES, true, gm_index);
    zint_rebuild_CRT_index(data, gs_index, slen, slen, n as u64, &PRIMES, true, gm_index);

    for u in slen..tlen {
        let p = PRIMES[u].p;
        let p0i = modp_ninv31(p);
        let r2 = modp_R2(p, p0i);
        let rx = modp_Rx(slen as u32, p, p0i, r2);
        modp_mkgm2_index(data, gm_index, igm_index, logn, PRIMES[u].g, p, p0i);
        let mut x_index = fs_index;
        for v in 0..n {
            data[t1_index + v] = zint_mod_small_signed_index(data, x_index, slen, p, p0i, r2, rx);
            x_index += slen;
        }

        modp_NTT2_index(data, t1_index, gm_index, logn, p, p0i);
        x_index = fd_index + u;
        for v in 0..hn {
            let w0 = data[t1_index + (v << 1)];
            let w1 = data[t1_index + (v << 1) + 1];

            data[x_index] = modp_montymul(
                modp_montymul(w0, w1, p, p0i), r2, p, p0i);
            x_index += tlen;
        }
        x_index = gs_index;
        for v in 0..n {
            data[t1_index + v] = zint_mod_small_signed_index(data, x_index, slen, p, p0i, r2, rx);
            x_index += slen;
        }

        modp_NTT2_index(data, t1_index, gm_index, logn, p, p0i);
        x_index = gd_index + u;
        for v in 0..hn {
            let w0 = data[t1_index + (v << 1)];
            let w1 = data[t1_index + (v << 1) + 1];
            data[x_index] = modp_montymul(
                modp_montymul(w0, w1, p, p0i), r2, p, p0i);
            x_index += tlen;
        }

        if !out_ntt {
            modp_iNTT2_ext_index(data, tlen, fd_index + u, igm_index, logn - 1, p, p0i);
            modp_iNTT2_ext_index(data, tlen, gd_index + u, igm_index, logn - 1, p, p0i);
        }
    }
}

// TODO USE SPLIT_AT_MUT INSTEAD
pub fn make_fg_step(data: &mut [u32], logn: u32, depth: usize, in_ntt: bool, out_ntt: bool) {
    let n = 1usize << logn;
    let hn = n >> 1;
    let slen = MAX_BL_SMALL[depth];
    let tlen = MAX_BL_SMALL[depth + 1];

    let fd_index = 0;
    let gd_index = fd_index + hn * tlen;
    let fs_index = gd_index + hn * tlen;
    let gs_index = fs_index + n * slen;
    let gm_index = gs_index + n * slen;
    let igm_index = gm_index + n;
    let t1_index = igm_index + n;
    data.copy_within(0..(2 * n * slen), fs_index);

    for u in 0..slen {
        let (p, p0i, r2): (u32, u32, u32);
        let mut x_index: usize;

        p = PRIMES[u].p;
        p0i = modp_ninv31(p);
        r2 = modp_R2(p, p0i);
        // let (_, gm_inter) = data.split_at_mut(gm_index);
        // let (gm, igm) = data[gm_index..].split_at_mut(gm_index - 1);
        // modp_mkgm2(gm, igm, logn, PRIMES[u].g, p, p0i);
        modp_mkgm2_index(data, gm_index, igm_index, logn, PRIMES[u].g, p, p0i);
        x_index = fs_index + u;
        for v in 0..n {
            data[t1_index + v] = data[x_index];
            x_index += slen;
        }
        if !in_ntt {
            modp_NTT2_index(data, t1_index, gm_index, logn, p, p0i);
        }
        x_index = fd_index + u;
        for v in 0..hn {
            let w0: u32 = data[t1_index + (v << 1)];
            let w1: u32 = data[t1_index + (v << 1) + 1];
            data[x_index] = modp_montymul(
                modp_montymul(w0, w1, p, p0i), r2, p, p0i);
            x_index += tlen;
        }
        if in_ntt {
            modp_iNTT2_ext_index(data, slen, fs_index + u, igm_index, logn, p, p0i);
        }

        x_index = gs_index + u;
        for v in 0..n {
            data[t1_index + v] = data[x_index];
            x_index += slen;
        }
        if !in_ntt {
            modp_NTT2_index(data, t1_index, gm_index, logn, p, p0i);
        }

        x_index = gd_index + u;
        for v in 0..hn {
            let w0 = data[t1_index + (v << 1)];
            let w1 = data[t1_index + (v << 1) + 1];
            data[x_index] = modp_montymul(
                modp_montymul(w0, w1, p, p0i), r2, p, p0i);
            x_index += tlen;
        }
        if in_ntt {
            modp_iNTT2_ext_index(data, slen, gs_index + u, igm_index, logn, p, p0i);
        }

        if !out_ntt {
            modp_iNTT2_ext_index(data, tlen, fd_index + u, igm_index, logn - 1, p, p0i);
            modp_iNTT2_ext_index(data, tlen, gd_index + u, igm_index, logn - 1, p, p0i);
        }
    }
    zint_rebuild_CRT_index(data, fs_index, slen, slen, n as u64, &PRIMES, true, gm_index);
    zint_rebuild_CRT_index(data, gs_index, slen, slen, n as u64, &PRIMES, true, gm_index);

    for u in slen..tlen {
        let p = PRIMES[u].p;
        let p0i = modp_ninv31(p);
        let r2 = modp_R2(p, p0i);
        let rx = modp_Rx(slen as u32, p, p0i, r2);
        modp_mkgm2_index(data, gm_index, igm_index, logn, PRIMES[u].g, p, p0i);
        let mut x_index = fs_index;
        for v in 0..n {
            data[t1_index + v] = zint_mod_small_signed_index(data, x_index, slen, p, p0i, r2, rx);
            x_index += slen;
        }

        modp_NTT2_index(data, t1_index, gm_index, logn, p, p0i);
        x_index = fd_index + u;
        for v in 0..hn {
            let w0 = data[t1_index + (v << 1)];
            let w1 = data[t1_index + (v << 1) + 1];

            data[x_index] = modp_montymul(
                modp_montymul(w0, w1, p, p0i), r2, p, p0i);
            x_index += tlen;
        }
        x_index = gs_index;
        for v in 0..n {
            data[t1_index + v] = zint_mod_small_signed_index(data, x_index, slen, p, p0i, r2, rx);
            x_index += slen;
        }

        modp_NTT2_index(data, t1_index, gm_index, logn, p, p0i);
        x_index = gd_index + u;
        for v in 0..hn {
            let w0 = data[t1_index + (v << 1)];
            let w1 = data[t1_index + (v << 1) + 1];
            data[x_index] = modp_montymul(
                modp_montymul(w0, w1, p, p0i), r2, p, p0i);
            x_index += tlen;
        }

        if !out_ntt {
            modp_iNTT2_ext_index(data, tlen, fd_index + u, igm_index, logn - 1, p, p0i);
            modp_iNTT2_ext_index(data, tlen, gd_index + u, igm_index, logn - 1, p, p0i);
        }
    }
}

pub fn make_fg(data: &mut [u32], f: &mut [i8], g: &mut [i8], logn: u32, depth: u32, out_ntt: bool) {
    let n = mkn!(logn);
    let p0 = PRIMES[0].p;
    let (ft, gt) = data.split_at_mut(n);
    for u in 0..n {
        ft[u] = modp_set(f[u] as i32, p0);
        gt[u] = modp_set(g[u] as i32, p0);
    }

    if depth == 0 && out_ntt {
        let p = PRIMES[0].p;
        let p0i = modp_ninv31(p);
        // Intermediate split to avoid borrow errors :)
        let (gtt, b) = gt.split_at_mut(n);
        let (gm, igm) = b.split_at_mut(n);
        modp_mkgm2(gm, igm, logn, PRIMES[0].g, p, p0i);
        modp_NTT2(ft, gm, logn, p, p0i);
        modp_NTT2(gtt, gm, logn, p, p0i);
        return;
    }
    for d in 0..depth {
        make_fg_step(data, logn - d, d as usize, d != 0, (d + 1) < depth || out_ntt);
    }
}

pub fn make_fg_index(data: &mut [u32], data_index: usize, f: &mut [i8], g: &mut [i8], logn: u32, depth: u32, out_ntt: bool) {
    let n = mkn!(logn);
    let p0 = PRIMES[0].p;
    let ft_index = data_index;
    let gt_index = ft_index + n;
    for u in 0..n {
        data[u + ft_index] = modp_set(f[u] as i32, p0);
        data[u + gt_index] = modp_set(g[u] as i32, p0);
    }

    if depth == 0 && out_ntt {
        let p = PRIMES[0].p;
        let p0i = modp_ninv31(p);
        // Intermediate split to avoid borrow errors :)
        let gm_index = gt_index + n;
        let igm_index = gm_index + n;
        modp_mkgm2_index(data, gm_index, igm_index, logn, PRIMES[0].g, p, p0i);
        modp_NTT2_index(data, ft_index, gm_index, logn, p, p0i);
        modp_NTT2_index(data, gt_index, gm_index, logn, p, p0i);
        return;
    }
    for d in 0..depth {
        make_fg_step_index(data, data_index, logn - d, d as usize, d != 0, (d + 1) < depth || out_ntt);
    }
}

#[allow(non_snake_case)]
pub fn solve_ntru_deepest(logn_top: u32, f: &mut [i8], g: &mut [i8], tmp: &mut [u32]) -> i32
{
    let len = MAX_BL_SMALL[logn_top as usize];
    let Fp_index = 0;
    let Gp_index = Fp_index + len;
    let fp_index = Gp_index + len;
    let gp_index = fp_index + len;
    let t1_index = gp_index + len;
    make_fg_index(tmp, fp_index, f, g, logn_top, logn_top, false);

    zint_rebuild_CRT_index(tmp, fp_index, len, len, 2, &PRIMES, false, t1_index);

    if !zint_bezout_index(tmp, Gp_index, Fp_index, fp_index, gp_index, len, t1_index) {
        return 0;
    }
    let q = 12289;

    if zint_mul_small_index(tmp, Fp_index, len, q) != 0
        || zint_mul_small_index(tmp, Gp_index, len, q) != 0
    {
        return 0;
    }
    1
}

#[allow(non_snake_case)]
pub fn solve_ntru_intermediate(logn_top: u32, f: &mut [i8], g: &mut [i8], depth: u32, tmp: &mut [u32]) -> i32 {
    let logn = logn_top - depth;
    let n = mkn!(logn);
    let hn = n >> 1;

    let slen = MAX_BL_SMALL[depth as usize];
    let dlen = MAX_BL_SMALL[(depth + 1) as usize];
    let llen = MAX_BL_LARGE[depth as usize];

    let mut Fd_index = 0;
    let mut Gd_index = Fd_index + dlen * hn;

    let mut ft_index = Gd_index + dlen * hn;
    make_fg_index(tmp, ft_index, f, g, logn_top, depth, true);

    let mut Ft_index = 0;
    let mut Gt_index = Ft_index + n * llen;
    let mut t1_index = Gt_index + n * llen;


    // 	memmove(2 * hn * tlen (fs), data, 2 * n * slen * sizeof *data);
    // tmp[t1_index..].copy_within(0..(2 * n * slen), t1_index);
    // tmp.copy_within(0..2 * n *slen, t1_index);
    ft_index = t1_index;
    let gt_index = ft_index + slen * n;
    t1_index = gt_index + slen * n;

    Fd_index = t1_index;
    Gd_index = Fd_index + hn * dlen;

    for u in 0..llen {
        let p = PRIMES[u].p;
        let p0i = modp_ninv31(p);
        let r2 = modp_R2(p, p0i);
        let rx = modp_Rx(dlen as u32, p, p0i, r2);

        let mut xs = Fd_index;
        let mut ys = Gd_index;
        let mut xd = Ft_index + u;
        let mut yd = Gt_index + u;
        for _ in 0..hn {
            tmp[xd] = zint_mod_small_signed_index(tmp, xs, dlen, p, p0i, r2, rx);
            tmp[yd] = zint_mod_small_signed_index(tmp, ys, dlen, p, p0i, r2, rx);
            xs += dlen;
            ys += dlen;
            xd += llen;
            yd += llen;
        }
    }

    for u in 0..llen {
        let p = PRIMES[u].p;
        let p0i = modp_ninv31(p);
        let r2 = modp_R2(p, p0i);

        if u == slen {
            zint_rebuild_CRT_index(tmp, ft_index, slen, slen, n as u64, primes, true, t1_index);
            zint_rebuild_CRT_index(tmp, gt_index, slen, slen, n as u64, primes, true, t1_index);
        }

        let gm_index = t1_index;
        let igm_index = gm_index + n;
        let fx_index = igm_index + n;
        let gx_index = fx_index + n;

        modp_mkgm2_index(tmp, gm_index, igm_index, logn, PRIMES[u].g, p, p0i);

        if u < slen {
            let mut x = ft_index + u;
            let mut y = gt_index + u;
            for _ in 0..n {
                tmp[fx_index] = tmp[x];
                tmp[gx_index] = tmp[y];
                x += slen;
                y += slen;
            }
            modp_iNTT2_ext_index(tmp, slen, ft_index + u, igm_index, logn, p, p0i);
            modp_iNTT2_ext_index(tmp, slen, gt_index + u, igm_index, logn, p, p0i);
        } else {
            let rx = modp_Rx(slen as u32, p, p0i, R2);
            let mut x = ft_index;
            let mut y = gt_index;
            for v in 0..n {
                tmp[fx_index + v] = zint_mod_small_signed_index(tmp, x, slen,
                                                                p, p0i, R2, Rx);
                tmp[gx_index + v] = zint_mod_small_signed_index(tmp, y, slen,
                                                                p, p0i, R2, Rx);
                x += slen;
                y += slen;
            }
            modp_NTT2_index(tmp, fx_index, gm_index, logn, p, p0i);
            modp_NTT2_index(tmp, gx_index, gm_index, logn, p, p0i);
        }

        let Fp_index = gx + n;
        let Gp_index = Fp_index + hn;
        let mut x = Ft_index + u;
        let mut y = Gt_index + u;
        for v in 0..hn {
            tmp[Fp_index + v] = tmp[x];
            tmp[Gp_index + v] = tmp[y];
            x += llen;
            y += llen;
        }
        modp_NTT2_index(tmp, Fp_index, gm_index, logn - 1, p, p0i);
        modp_NTT2_index(tmp, Gp_index, gm_index, logn - 1, p, p0i);

        let mut x = Ft_index + u;
        let mut y = Gt_index + u;
        for v in 0..hn {
            let ftA = tmp[fx_index + (v << 1)];
            let ftB = tmp[fx_index + (v << 1) + 1];
            let gtA = tmp[gx_index + (v << 1)];
            let gtB = tmp[gx_index + (v << 1) + 1];
            let mFp = modp_montymul(tmp[Fp_index + v], r2, p, p0i);
            let mGp = modp_montymul(tmp[Gp_index + v], r2, p, p0i);
            tmp[x] = modp_montymul(gtB, mFp, p, p0i);
            tmp[x + llen] = modp_montymul(gtA, mFp, p, p0i);
            tmp[y] = modp_montymul(ftB, mGp, p, p0i);
            tmp[y + llen] = modp_montymul(ftA, mGp, p, p0i);

            x += (llen << 1);
            y += (llen << 1);
        }
        modp_iNTT2_ext_index(tmp, llen, Ft_index + u, igm_index, logn, p, p0i);
        modp_iNTT2_ext_index(tmp, llen, Gt_index + u, igm_index, logn, p, p0i);
    }
    zint_rebuild_CRT_index(tmp, Ft_index, llen, llen, n as u64, primes, true, t1_index);
    zint_rebuild_CRT_index(tmp, Gt_index, llen, llen, n as u64, primes, true, t1_index);

    // TODO CRY
    // let rt3 = align_fpr(tmp, t1_index);
    let rt4 = t1_index + n;
    let rt5 = rt4 + n;
    let rt1 = rt5 + (n >> 1);
    // let k = align_u32(tmp, rt1);
    // let rt2 = align_fpr(tmp, k + n);
    if rt2 < (rt1 + n) {
        rt2 = rt1 + n;
    }
    t1 = k + n;

    let rlen = if (slen > 10) { 10 } else { slen };
    // poly_big_to_fp(rt3, ft + slen - rlen, rlen, slen, logn);
    // poly_big_to_fp(rt4, gt + slen - rlen, rlen, slen, logn);
    // scale_fg = 31 * (int)(slen - rlen);

    let minbl_fg = BITLENGTH[depth].avg - 6 * BITLENGTH[depth].std;
    let maxbl_fg = BITLENGTH[depth].avg + 6 * BITLENGTH[depth].std;

    // fft(rt3, logn);
    // fft(rt4, logn);
    // poly_invnorm2_fft(rt5, rt3, rt4, logn);
    // poly_adj_fft(rt3, logn);
    // poly_adj_fft(rt4, logn);

    let FGlen = llen;
    let maxbl_FG = 31 * llen as i32;

    let mut scale_k = maxbl_FG - minbl_fg;
    loop {
        if scale_k <= 0 {
            break;
        }
        scale_k -= 25;
        if scale_k < 0 {
            scale_k = 0;
        }
    }
    if FGlen < slen {
        for u in 0..n {
            let mut sw = (!(tmp[Ft_index + FGlen - 1] >> 30) >> 1).wrapping_add(1);
            for v in FGlen..slen {
                tmp[Ft_index + v] = sw;
            }
            let mut sw = (!(tmp[Gt_index + FGlen - 1] >> 30) >> 1).wrapping_add(1);
            for v in FGlen..slen {
                tmp[Gt_index + v] = sw;
            }
            Ft_index += llen;
            Gt_index += llen;
        }
    }

    // TODO DEATH
    let mut x = 0;
    let mut y = 0;
    for u in 0..(n<< 1) {
        // memmove(x, y, slen * sizeof *y);
        x += slen;
        y += llen;
    }
    0
}

pub fn solve_ntru_binary_depth1(logn_top: u32, f: &mut [i8], g: &mut [i8], tmp: &mut [u32]) -> i32 {
    0
}

pub fn solve_ntru_binary_depth0(logn_top: u32, f: &mut [i8], g: &mut [i8], tmp: &mut [u32]) -> i32 {
    0
}

#[allow(non_snake_case)]
pub fn solve_ntru(logn_top: u32, F: &mut [i8], G: &mut [i8], f: &mut [i8], g: &mut [i8], lim: i32, tmp: &mut [u32]) -> i32 {
    0
}

pub fn poly_small_mkgauss(rng: *const InnerShake256Context, f: &mut [i8], logn: u32) {}

#[allow(non_snake_case)]
pub fn keygen(rng: *const InnerShake256Context, f: &mut [i8], g: &mut [i8], F: &mut [i8], G: &mut [i8], h: *const u16, logn: u32, tmp: &mut [u8]) {}

pub(crate) static MAX_BL_SMALL: [usize; 11] = [
    1, 1, 2, 2, 4, 7, 14, 27, 53, 106, 209
];

pub(crate) static MAX_BL_LARGE: [usize; 10] = [
    2, 2, 5, 7, 12, 21, 40, 78, 157, 308
];

pub(crate) static GAUSS_1024_12289: [u64; 27] = [
    1283868770400643928, 6416574995475331444, 4078260278032692663,
    2353523259288686585, 1227179971273316331, 575931623374121527,
    242543240509105209, 91437049221049666, 30799446349977173,
    9255276791179340, 2478152334826140, 590642893610164,
    125206034929641, 23590435911403, 3948334035941,
    586753615614, 77391054539, 9056793210,
    940121950, 86539696, 7062824,
    510971, 32764, 1862,
    94, 4, 0
];

pub static DEPTH_INT_FG: i32 = 4;

pub(crate) static BITLENGTH: [BitLength; 11] = [
    BitLength { avg: 4, std: 0 },
    BitLength { avg: 11, std: 1 },
    BitLength { avg: 24, std: 1 },
    BitLength { avg: 50, std: 1 },
    BitLength { avg: 102, std: 1 },
    BitLength { avg: 202, std: 2 },
    BitLength { avg: 401, std: 4 },
    BitLength { avg: 794, std: 5 },
    BitLength { avg: 1577, std: 8 },
    BitLength { avg: 3138, std: 13 },
    BitLength { avg: 6308, std: 25 }
];


static REV10: [usize; 1024] = [
    0, 512, 256, 768, 128, 640, 384, 896, 64, 576, 320, 832,
    192, 704, 448, 960, 32, 544, 288, 800, 160, 672, 416, 928,
    96, 608, 352, 864, 224, 736, 480, 992, 16, 528, 272, 784,
    144, 656, 400, 912, 80, 592, 336, 848, 208, 720, 464, 976,
    48, 560, 304, 816, 176, 688, 432, 944, 112, 624, 368, 880,
    240, 752, 496, 1008, 8, 520, 264, 776, 136, 648, 392, 904,
    72, 584, 328, 840, 200, 712, 456, 968, 40, 552, 296, 808,
    168, 680, 424, 936, 104, 616, 360, 872, 232, 744, 488, 1000,
    24, 536, 280, 792, 152, 664, 408, 920, 88, 600, 344, 856,
    216, 728, 472, 984, 56, 568, 312, 824, 184, 696, 440, 952,
    120, 632, 376, 888, 248, 760, 504, 1016, 4, 516, 260, 772,
    132, 644, 388, 900, 68, 580, 324, 836, 196, 708, 452, 964,
    36, 548, 292, 804, 164, 676, 420, 932, 100, 612, 356, 868,
    228, 740, 484, 996, 20, 532, 276, 788, 148, 660, 404, 916,
    84, 596, 340, 852, 212, 724, 468, 980, 52, 564, 308, 820,
    180, 692, 436, 948, 116, 628, 372, 884, 244, 756, 500, 1012,
    12, 524, 268, 780, 140, 652, 396, 908, 76, 588, 332, 844,
    204, 716, 460, 972, 44, 556, 300, 812, 172, 684, 428, 940,
    108, 620, 364, 876, 236, 748, 492, 1004, 28, 540, 284, 796,
    156, 668, 412, 924, 92, 604, 348, 860, 220, 732, 476, 988,
    60, 572, 316, 828, 188, 700, 444, 956, 124, 636, 380, 892,
    252, 764, 508, 1020, 2, 514, 258, 770, 130, 642, 386, 898,
    66, 578, 322, 834, 194, 706, 450, 962, 34, 546, 290, 802,
    162, 674, 418, 930, 98, 610, 354, 866, 226, 738, 482, 994,
    18, 530, 274, 786, 146, 658, 402, 914, 82, 594, 338, 850,
    210, 722, 466, 978, 50, 562, 306, 818, 178, 690, 434, 946,
    114, 626, 370, 882, 242, 754, 498, 1010, 10, 522, 266, 778,
    138, 650, 394, 906, 74, 586, 330, 842, 202, 714, 458, 970,
    42, 554, 298, 810, 170, 682, 426, 938, 106, 618, 362, 874,
    234, 746, 490, 1002, 26, 538, 282, 794, 154, 666, 410, 922,
    90, 602, 346, 858, 218, 730, 474, 986, 58, 570, 314, 826,
    186, 698, 442, 954, 122, 634, 378, 890, 250, 762, 506, 1018,
    6, 518, 262, 774, 134, 646, 390, 902, 70, 582, 326, 838,
    198, 710, 454, 966, 38, 550, 294, 806, 166, 678, 422, 934,
    102, 614, 358, 870, 230, 742, 486, 998, 22, 534, 278, 790,
    150, 662, 406, 918, 86, 598, 342, 854, 214, 726, 470, 982,
    54, 566, 310, 822, 182, 694, 438, 950, 118, 630, 374, 886,
    246, 758, 502, 1014, 14, 526, 270, 782, 142, 654, 398, 910,
    78, 590, 334, 846, 206, 718, 462, 974, 46, 558, 302, 814,
    174, 686, 430, 942, 110, 622, 366, 878, 238, 750, 494, 1006,
    30, 542, 286, 798, 158, 670, 414, 926, 94, 606, 350, 862,
    222, 734, 478, 990, 62, 574, 318, 830, 190, 702, 446, 958,
    126, 638, 382, 894, 254, 766, 510, 1022, 1, 513, 257, 769,
    129, 641, 385, 897, 65, 577, 321, 833, 193, 705, 449, 961,
    33, 545, 289, 801, 161, 673, 417, 929, 97, 609, 353, 865,
    225, 737, 481, 993, 17, 529, 273, 785, 145, 657, 401, 913,
    81, 593, 337, 849, 209, 721, 465, 977, 49, 561, 305, 817,
    177, 689, 433, 945, 113, 625, 369, 881, 241, 753, 497, 1009,
    9, 521, 265, 777, 137, 649, 393, 905, 73, 585, 329, 841,
    201, 713, 457, 969, 41, 553, 297, 809, 169, 681, 425, 937,
    105, 617, 361, 873, 233, 745, 489, 1001, 25, 537, 281, 793,
    153, 665, 409, 921, 89, 601, 345, 857, 217, 729, 473, 985,
    57, 569, 313, 825, 185, 697, 441, 953, 121, 633, 377, 889,
    249, 761, 505, 1017, 5, 517, 261, 773, 133, 645, 389, 901,
    69, 581, 325, 837, 197, 709, 453, 965, 37, 549, 293, 805,
    165, 677, 421, 933, 101, 613, 357, 869, 229, 741, 485, 997,
    21, 533, 277, 789, 149, 661, 405, 917, 85, 597, 341, 853,
    213, 725, 469, 981, 53, 565, 309, 821, 181, 693, 437, 949,
    117, 629, 373, 885, 245, 757, 501, 1013, 13, 525, 269, 781,
    141, 653, 397, 909, 77, 589, 333, 845, 205, 717, 461, 973,
    45, 557, 301, 813, 173, 685, 429, 941, 109, 621, 365, 877,
    237, 749, 493, 1005, 29, 541, 285, 797, 157, 669, 413, 925,
    93, 605, 349, 861, 221, 733, 477, 989, 61, 573, 317, 829,
    189, 701, 445, 957, 125, 637, 381, 893, 253, 765, 509, 1021,
    3, 515, 259, 771, 131, 643, 387, 899, 67, 579, 323, 835,
    195, 707, 451, 963, 35, 547, 291, 803, 163, 675, 419, 931,
    99, 611, 355, 867, 227, 739, 483, 995, 19, 531, 275, 787,
    147, 659, 403, 915, 83, 595, 339, 851, 211, 723, 467, 979,
    51, 563, 307, 819, 179, 691, 435, 947, 115, 627, 371, 883,
    243, 755, 499, 1011, 11, 523, 267, 779, 139, 651, 395, 907,
    75, 587, 331, 843, 203, 715, 459, 971, 43, 555, 299, 811,
    171, 683, 427, 939, 107, 619, 363, 875, 235, 747, 491, 1003,
    27, 539, 283, 795, 155, 667, 411, 923, 91, 603, 347, 859,
    219, 731, 475, 987, 59, 571, 315, 827, 187, 699, 443, 955,
    123, 635, 379, 891, 251, 763, 507, 1019, 7, 519, 263, 775,
    135, 647, 391, 903, 71, 583, 327, 839, 199, 711, 455, 967,
    39, 551, 295, 807, 167, 679, 423, 935, 103, 615, 359, 871,
    231, 743, 487, 999, 23, 535, 279, 791, 151, 663, 407, 919,
    87, 599, 343, 855, 215, 727, 471, 983, 55, 567, 311, 823,
    183, 695, 439, 951, 119, 631, 375, 887, 247, 759, 503, 1015,
    15, 527, 271, 783, 143, 655, 399, 911, 79, 591, 335, 847,
    207, 719, 463, 975, 47, 559, 303, 815, 175, 687, 431, 943,
    111, 623, 367, 879, 239, 751, 495, 1007, 31, 543, 287, 799,
    159, 671, 415, 927, 95, 607, 351, 863, 223, 735, 479, 991,
    63, 575, 319, 831, 191, 703, 447, 959, 127, 639, 383, 895,
    255, 767, 511, 1023
];

pub(crate) static PRIMES: [SmallPrimes; 522] = [
    SmallPrimes { p: 2147473409, g: 383167813, s: 10239 },
    SmallPrimes { p: 2147389441, g: 211808905, s: 471403745 },
    SmallPrimes { p: 2147387393, g: 37672282, s: 1329335065 },
    SmallPrimes { p: 2147377153, g: 1977035326, s: 968223422 },
    SmallPrimes { p: 2147358721, g: 1067163706, s: 132460015 },
    SmallPrimes { p: 2147352577, g: 1606082042, s: 598693809 },
    SmallPrimes { p: 2147346433, g: 2033915641, s: 1056257184 },
    SmallPrimes { p: 2147338241, g: 1653770625, s: 421286710 },
    SmallPrimes { p: 2147309569, g: 631200819, s: 1111201074 },
    SmallPrimes { p: 2147297281, g: 2038364663, s: 1042003613 },
    SmallPrimes { p: 2147295233, g: 1962540515, s: 19440033 },
    SmallPrimes { p: 2147239937, g: 2100082663, s: 353296760 },
    SmallPrimes { p: 2147235841, g: 1991153006, s: 1703918027 },
    SmallPrimes { p: 2147217409, g: 516405114, s: 1258919613 },
    SmallPrimes { p: 2147205121, g: 409347988, s: 1089726929 },
    SmallPrimes { p: 2147196929, g: 927788991, s: 1946238668 },
    SmallPrimes { p: 2147178497, g: 1136922411, s: 1347028164 },
    SmallPrimes { p: 2147100673, g: 868626236, s: 701164723 },
    SmallPrimes { p: 2147082241, g: 1897279176, s: 617820870 },
    SmallPrimes { p: 2147074049, g: 1888819123, s: 158382189 },
    SmallPrimes { p: 2147051521, g: 25006327, s: 522758543 },
    SmallPrimes { p: 2147043329, g: 327546255, s: 37227845 },
    SmallPrimes { p: 2147039233, g: 766324424, s: 1133356428 },
    SmallPrimes { p: 2146988033, g: 1862817362, s: 73861329 },
    SmallPrimes { p: 2146963457, g: 404622040, s: 653019435 },
    SmallPrimes { p: 2146959361, g: 1936581214, s: 995143093 },
    SmallPrimes { p: 2146938881, g: 1559770096, s: 634921513 },
    SmallPrimes { p: 2146908161, g: 422623708, s: 1985060172 },
    SmallPrimes { p: 2146885633, g: 1751189170, s: 298238186 },
    SmallPrimes { p: 2146871297, g: 578919515, s: 291810829 },
    SmallPrimes { p: 2146846721, g: 1114060353, s: 915902322 },
    SmallPrimes { p: 2146834433, g: 2069565474, s: 47859524 },
    SmallPrimes { p: 2146818049, g: 1552824584, s: 646281055 },
    SmallPrimes { p: 2146775041, g: 1906267847, s: 1597832891 },
    SmallPrimes { p: 2146756609, g: 1847414714, s: 1228090888 },
    SmallPrimes { p: 2146744321, g: 1818792070, s: 1176377637 },
    SmallPrimes { p: 2146738177, g: 1118066398, s: 1054971214 },
    SmallPrimes { p: 2146736129, g: 52057278, s: 933422153 },
    SmallPrimes { p: 2146713601, g: 592259376, s: 1406621510 },
    SmallPrimes { p: 2146695169, g: 263161877, s: 1514178701 },
    SmallPrimes { p: 2146656257, g: 685363115, s: 384505091 },
    SmallPrimes { p: 2146650113, g: 927727032, s: 537575289 },
    SmallPrimes { p: 2146646017, g: 52575506, s: 1799464037 },
    SmallPrimes { p: 2146643969, g: 1276803876, s: 1348954416 },
    SmallPrimes { p: 2146603009, g: 814028633, s: 1521547704 },
    SmallPrimes { p: 2146572289, g: 1846678872, s: 1310832121 },
    SmallPrimes { p: 2146547713, g: 919368090, s: 1019041349 },
    SmallPrimes { p: 2146508801, g: 671847612, s: 38582496 },
    SmallPrimes { p: 2146492417, g: 283911680, s: 532424562 },
    SmallPrimes { p: 2146490369, g: 1780044827, s: 896447978 },
    SmallPrimes { p: 2146459649, g: 327980850, s: 1327906900 },
    SmallPrimes { p: 2146447361, g: 1310561493, s: 958645253 },
    SmallPrimes { p: 2146441217, g: 412148926, s: 287271128 },
    SmallPrimes { p: 2146437121, g: 293186449, s: 2009822534 },
    SmallPrimes { p: 2146430977, g: 179034356, s: 1359155584 },
    SmallPrimes { p: 2146418689, g: 1517345488, s: 1790248672 },
    SmallPrimes { p: 2146406401, g: 1615820390, s: 1584833571 },
    SmallPrimes { p: 2146404353, g: 826651445, s: 607120498 },
    SmallPrimes { p: 2146379777, g: 3816988, s: 1897049071 },
    SmallPrimes { p: 2146363393, g: 1221409784, s: 1986921567 },
    SmallPrimes { p: 2146355201, g: 1388081168, s: 849968120 },
    SmallPrimes { p: 2146336769, g: 1803473237, s: 1655544036 },
    SmallPrimes { p: 2146312193, g: 1023484977, s: 273671831 },
    SmallPrimes { p: 2146293761, g: 1074591448, s: 467406983 },
    SmallPrimes { p: 2146283521, g: 831604668, s: 1523950494 },
    SmallPrimes { p: 2146203649, g: 712865423, s: 1170834574 },
    SmallPrimes { p: 2146154497, g: 1764991362, s: 1064856763 },
    SmallPrimes { p: 2146142209, g: 627386213, s: 1406840151 },
    SmallPrimes { p: 2146127873, g: 1638674429, s: 2088393537 },
    SmallPrimes { p: 2146099201, g: 1516001018, s: 690673370 },
    SmallPrimes { p: 2146093057, g: 1294931393, s: 315136610 },
    SmallPrimes { p: 2146091009, g: 1942399533, s: 973539425 },
    SmallPrimes { p: 2146078721, g: 1843461814, s: 2132275436 },
    SmallPrimes { p: 2146060289, g: 1098740778, s: 360423481 },
    SmallPrimes { p: 2146048001, g: 1617213232, s: 1951981294 },
    SmallPrimes { p: 2146041857, g: 1805783169, s: 2075683489 },
    SmallPrimes { p: 2146019329, g: 272027909, s: 1753219918 },
    SmallPrimes { p: 2145986561, g: 1206530344, s: 2034028118 },
    SmallPrimes { p: 2145976321, g: 1243769360, s: 1173377644 },
    SmallPrimes { p: 2145964033, g: 887200839, s: 1281344586 },
    SmallPrimes { p: 2145906689, g: 1651026455, s: 906178216 },
    SmallPrimes { p: 2145875969, g: 1673238256, s: 1043521212 },
    SmallPrimes { p: 2145871873, g: 1226591210, s: 1399796492 },
    SmallPrimes { p: 2145841153, g: 1465353397, s: 1324527802 },
    SmallPrimes { p: 2145832961, g: 1150638905, s: 554084759 },
    SmallPrimes { p: 2145816577, g: 221601706, s: 427340863 },
    SmallPrimes { p: 2145785857, g: 608896761, s: 316590738 },
    SmallPrimes { p: 2145755137, g: 1712054942, s: 1684294304 },
    SmallPrimes { p: 2145742849, g: 1302302867, s: 724873116 },
    SmallPrimes { p: 2145728513, g: 516717693, s: 431671476 },
    SmallPrimes { p: 2145699841, g: 524575579, s: 1619722537 },
    SmallPrimes { p: 2145691649, g: 1925625239, s: 982974435 },
    SmallPrimes { p: 2145687553, g: 463795662, s: 1293154300 },
    SmallPrimes { p: 2145673217, g: 771716636, s: 881778029 },
    SmallPrimes { p: 2145630209, g: 1509556977, s: 837364988 },
    SmallPrimes { p: 2145595393, g: 229091856, s: 851648427 },
    SmallPrimes { p: 2145587201, g: 1796903241, s: 635342424 },
    SmallPrimes { p: 2145525761, g: 715310882, s: 1677228081 },
    SmallPrimes { p: 2145495041, g: 1040930522, s: 200685896 },
    SmallPrimes { p: 2145466369, g: 949804237, s: 1809146322 },
    SmallPrimes { p: 2145445889, g: 1673903706, s: 95316881 },
    SmallPrimes { p: 2145390593, g: 806941852, s: 1428671135 },
    SmallPrimes { p: 2145372161, g: 1402525292, s: 159350694 },
    SmallPrimes { p: 2145361921, g: 2124760298, s: 1589134749 },
    SmallPrimes { p: 2145359873, g: 1217503067, s: 1561543010 },
    SmallPrimes { p: 2145355777, g: 338341402, s: 83865711 },
    SmallPrimes { p: 2145343489, g: 1381532164, s: 641430002 },
    SmallPrimes { p: 2145325057, g: 1883895478, s: 1528469895 },
    SmallPrimes { p: 2145318913, g: 1335370424, s: 65809740 },
    SmallPrimes { p: 2145312769, g: 2000008042, s: 1919775760 },
    SmallPrimes { p: 2145300481, g: 961450962, s: 1229540578 },
    SmallPrimes { p: 2145282049, g: 910466767, s: 1964062701 },
    SmallPrimes { p: 2145232897, g: 816527501, s: 450152063 },
    SmallPrimes { p: 2145218561, g: 1435128058, s: 1794509700 },
    SmallPrimes { p: 2145187841, g: 33505311, s: 1272467582 },
    SmallPrimes { p: 2145181697, g: 269767433, s: 1380363849 },
    SmallPrimes { p: 2145175553, g: 56386299, s: 1316870546 },
    SmallPrimes { p: 2145079297, g: 2106880293, s: 1391797340 },
    SmallPrimes { p: 2145021953, g: 1347906152, s: 720510798 },
    SmallPrimes { p: 2145015809, g: 206769262, s: 1651459955 },
    SmallPrimes { p: 2145003521, g: 1885513236, s: 1393381284 },
    SmallPrimes { p: 2144960513, g: 1810381315, s: 31937275 },
    SmallPrimes { p: 2144944129, g: 1306487838, s: 2019419520 },
    SmallPrimes { p: 2144935937, g: 37304730, s: 1841489054 },
    SmallPrimes { p: 2144894977, g: 1601434616, s: 157985831 },
    SmallPrimes { p: 2144888833, g: 98749330, s: 2128592228 },
    SmallPrimes { p: 2144880641, g: 1772327002, s: 2076128344 },
    SmallPrimes { p: 2144864257, g: 1404514762, s: 2029969964 },
    SmallPrimes { p: 2144827393, g: 801236594, s: 406627220 },
    SmallPrimes { p: 2144806913, g: 349217443, s: 1501080290 },
    SmallPrimes { p: 2144796673, g: 1542656776, s: 2084736519 },
    SmallPrimes { p: 2144778241, g: 1210734884, s: 1746416203 },
    SmallPrimes { p: 2144759809, g: 1146598851, s: 716464489 },
    SmallPrimes { p: 2144757761, g: 286328400, s: 1823728177 },
    SmallPrimes { p: 2144729089, g: 1347555695, s: 1836644881 },
    SmallPrimes { p: 2144727041, g: 1795703790, s: 520296412 },
    SmallPrimes { p: 2144696321, g: 1302475157, s: 852964281 },
    SmallPrimes { p: 2144667649, g: 1075877614, s: 504992927 },
    SmallPrimes { p: 2144573441, g: 198765808, s: 1617144982 },
    SmallPrimes { p: 2144555009, g: 321528767, s: 155821259 },
    SmallPrimes { p: 2144550913, g: 814139516, s: 1819937644 },
    SmallPrimes { p: 2144536577, g: 571143206, s: 962942255 },
    SmallPrimes { p: 2144524289, g: 1746733766, s: 2471321 },
    SmallPrimes { p: 2144512001, g: 1821415077, s: 124190939 },
    SmallPrimes { p: 2144468993, g: 917871546, s: 1260072806 },
    SmallPrimes { p: 2144458753, g: 378417981, s: 1569240563 },
    SmallPrimes { p: 2144421889, g: 175229668, s: 1825620763 },
    SmallPrimes { p: 2144409601, g: 1699216963, s: 351648117 },
    SmallPrimes { p: 2144370689, g: 1071885991, s: 958186029 },
    SmallPrimes { p: 2144348161, g: 1763151227, s: 540353574 },
    SmallPrimes { p: 2144335873, g: 1060214804, s: 919598847 },
    SmallPrimes { p: 2144329729, g: 663515846, s: 1448552668 },
    SmallPrimes { p: 2144327681, g: 1057776305, s: 590222840 },
    SmallPrimes { p: 2144309249, g: 1705149168, s: 1459294624 },
    SmallPrimes { p: 2144296961, g: 325823721, s: 1649016934 },
    SmallPrimes { p: 2144290817, g: 738775789, s: 447427206 },
    SmallPrimes { p: 2144243713, g: 962347618, s: 893050215 },
    SmallPrimes { p: 2144237569, g: 1655257077, s: 900860862 },
    SmallPrimes { p: 2144161793, g: 242206694, s: 1567868672 },
    SmallPrimes { p: 2144155649, g: 769415308, s: 1247993134 },
    SmallPrimes { p: 2144137217, g: 320492023, s: 515841070 },
    SmallPrimes { p: 2144120833, g: 1639388522, s: 770877302 },
    SmallPrimes { p: 2144071681, g: 1761785233, s: 964296120 },
    SmallPrimes { p: 2144065537, g: 419817825, s: 204564472 },
    SmallPrimes { p: 2144028673, g: 666050597, s: 2091019760 },
    SmallPrimes { p: 2144010241, g: 1413657615, s: 1518702610 },
    SmallPrimes { p: 2143952897, g: 1238327946, s: 475672271 },
    SmallPrimes { p: 2143940609, g: 307063413, s: 1176750846 },
    SmallPrimes { p: 2143918081, g: 2062905559, s: 786785803 },
    SmallPrimes { p: 2143899649, g: 1338112849, s: 1562292083 },
    SmallPrimes { p: 2143891457, g: 68149545, s: 87166451 },
    SmallPrimes { p: 2143885313, g: 921750778, s: 394460854 },
    SmallPrimes { p: 2143854593, g: 719766593, s: 133877196 },
    SmallPrimes { p: 2143836161, g: 1149399850, s: 1861591875 },
    SmallPrimes { p: 2143762433, g: 1848739366, s: 1335934145 },
    SmallPrimes { p: 2143756289, g: 1326674710, s: 102999236 },
    SmallPrimes { p: 2143713281, g: 808061791, s: 1156900308 },
    SmallPrimes { p: 2143690753, g: 388399459, s: 1926468019 },
    SmallPrimes { p: 2143670273, g: 1427891374, s: 1756689401 },
    SmallPrimes { p: 2143666177, g: 1912173949, s: 986629565 },
    SmallPrimes { p: 2143645697, g: 2041160111, s: 371842865 },
    SmallPrimes { p: 2143641601, g: 1279906897, s: 2023974350 },
    SmallPrimes { p: 2143635457, g: 720473174, s: 1389027526 },
    SmallPrimes { p: 2143621121, g: 1298309455, s: 1732632006 },
    SmallPrimes { p: 2143598593, g: 1548762216, s: 1825417506 },
    SmallPrimes { p: 2143567873, g: 620475784, s: 1073787233 },
    SmallPrimes { p: 2143561729, g: 1932954575, s: 949167309 },
    SmallPrimes { p: 2143553537, g: 354315656, s: 1652037534 },
    SmallPrimes { p: 2143541249, g: 577424288, s: 1097027618 },
    SmallPrimes { p: 2143531009, g: 357862822, s: 478640055 },
    SmallPrimes { p: 2143522817, g: 2017706025, s: 1550531668 },
    SmallPrimes { p: 2143506433, g: 2078127419, s: 1824320165 },
    SmallPrimes { p: 2143488001, g: 613475285, s: 1604011510 },
    SmallPrimes { p: 2143469569, g: 1466594987, s: 502095196 },
    SmallPrimes { p: 2143426561, g: 1115430331, s: 1044637111 },
    SmallPrimes { p: 2143383553, g: 9778045, s: 1902463734 },
    SmallPrimes { p: 2143377409, g: 1557401276, s: 2056861771 },
    SmallPrimes { p: 2143363073, g: 652036455, s: 1965915971 },
    SmallPrimes { p: 2143260673, g: 1464581171, s: 1523257541 },
    SmallPrimes { p: 2143246337, g: 1876119649, s: 764541916 },
    SmallPrimes { p: 2143209473, g: 1614992673, s: 1920672844 },
    SmallPrimes { p: 2143203329, g: 981052047, s: 2049774209 },
    SmallPrimes { p: 2143160321, g: 1847355533, s: 728535665 },
    SmallPrimes { p: 2143129601, g: 965558457, s: 603052992 },
    SmallPrimes { p: 2143123457, g: 2140817191, s: 8348679 },
    SmallPrimes { p: 2143100929, g: 1547263683, s: 694209023 },
    SmallPrimes { p: 2143092737, g: 643459066, s: 1979934533 },
    SmallPrimes { p: 2143082497, g: 188603778, s: 2026175670 },
    SmallPrimes { p: 2143062017, g: 1657329695, s: 377451099 },
    SmallPrimes { p: 2143051777, g: 114967950, s: 979255473 },
    SmallPrimes { p: 2143025153, g: 1698431342, s: 1449196896 },
    SmallPrimes { p: 2143006721, g: 1862741675, s: 1739650365 },
    SmallPrimes { p: 2142996481, g: 756660457, s: 996160050 },
    SmallPrimes { p: 2142976001, g: 927864010, s: 1166847574 },
    SmallPrimes { p: 2142965761, g: 905070557, s: 661974566 },
    SmallPrimes { p: 2142916609, g: 40932754, s: 1787161127 },
    SmallPrimes { p: 2142892033, g: 1987985648, s: 675335382 },
    SmallPrimes { p: 2142885889, g: 797497211, s: 1323096997 },
    SmallPrimes { p: 2142871553, g: 2068025830, s: 1411877159 },
    SmallPrimes { p: 2142861313, g: 1217177090, s: 1438410687 },
    SmallPrimes { p: 2142830593, g: 409906375, s: 1767860634 },
    SmallPrimes { p: 2142803969, g: 1197788993, s: 359782919 },
    SmallPrimes { p: 2142785537, g: 643817365, s: 513932862 },
    SmallPrimes { p: 2142779393, g: 1717046338, s: 218943121 },
    SmallPrimes { p: 2142724097, g: 89336830, s: 416687049 },
    SmallPrimes { p: 2142707713, g: 5944581, s: 1356813523 },
    SmallPrimes { p: 2142658561, g: 887942135, s: 2074011722 },
    SmallPrimes { p: 2142638081, g: 151851972, s: 1647339939 },
    SmallPrimes { p: 2142564353, g: 1691505537, s: 1483107336 },
    SmallPrimes { p: 2142533633, g: 1989920200, s: 1135938817 },
    SmallPrimes { p: 2142529537, g: 959263126, s: 1531961857 },
    SmallPrimes { p: 2142527489, g: 453251129, s: 1725566162 },
    SmallPrimes { p: 2142502913, g: 1536028102, s: 182053257 },
    SmallPrimes { p: 2142498817, g: 570138730, s: 701443447 },
    SmallPrimes { p: 2142416897, g: 326965800, s: 411931819 },
    SmallPrimes { p: 2142363649, g: 1675665410, s: 1517191733 },
    SmallPrimes { p: 2142351361, g: 968529566, s: 1575712703 },
    SmallPrimes { p: 2142330881, g: 1384953238, s: 1769087884 },
    SmallPrimes { p: 2142314497, g: 1977173242, s: 1833745524 },
    SmallPrimes { p: 2142289921, g: 95082313, s: 1714775493 },
    SmallPrimes { p: 2142283777, g: 109377615, s: 1070584533 },
    SmallPrimes { p: 2142277633, g: 16960510, s: 702157145 },
    SmallPrimes { p: 2142263297, g: 553850819, s: 431364395 },
    SmallPrimes { p: 2142208001, g: 241466367, s: 2053967982 },
    SmallPrimes { p: 2142164993, g: 1795661326, s: 1031836848 },
    SmallPrimes { p: 2142097409, g: 1212530046, s: 712772031 },
    SmallPrimes { p: 2142087169, g: 1763869720, s: 822276067 },
    SmallPrimes { p: 2142078977, g: 644065713, s: 1765268066 },
    SmallPrimes { p: 2142074881, g: 112671944, s: 643204925 },
    SmallPrimes { p: 2142044161, g: 1387785471, s: 1297890174 },
    SmallPrimes { p: 2142025729, g: 783885537, s: 1000425730 },
    SmallPrimes { p: 2142011393, g: 905662232, s: 1679401033 },
    SmallPrimes { p: 2141974529, g: 799788433, s: 468119557 },
    SmallPrimes { p: 2141943809, g: 1932544124, s: 449305555 },
    SmallPrimes { p: 2141933569, g: 1527403256, s: 841867925 },
    SmallPrimes { p: 2141931521, g: 1247076451, s: 743823916 },
    SmallPrimes { p: 2141902849, g: 1199660531, s: 401687910 },
    SmallPrimes { p: 2141890561, g: 150132350, s: 1720336972 },
    SmallPrimes { p: 2141857793, g: 1287438162, s: 663880489 },
    SmallPrimes { p: 2141833217, g: 618017731, s: 1819208266 },
    SmallPrimes { p: 2141820929, g: 999578638, s: 1403090096 },
    SmallPrimes { p: 2141786113, g: 81834325, s: 1523542501 },
    SmallPrimes { p: 2141771777, g: 120001928, s: 463556492 },
    SmallPrimes { p: 2141759489, g: 122455485, s: 2124928282 },
    SmallPrimes { p: 2141749249, g: 141986041, s: 940339153 },
    SmallPrimes { p: 2141685761, g: 889088734, s: 477141499 },
    SmallPrimes { p: 2141673473, g: 324212681, s: 1122558298 },
    SmallPrimes { p: 2141669377, g: 1175806187, s: 1373818177 },
    SmallPrimes { p: 2141655041, g: 1113654822, s: 296887082 },
    SmallPrimes { p: 2141587457, g: 991103258, s: 1585913875 },
    SmallPrimes { p: 2141583361, g: 1401451409, s: 1802457360 },
    SmallPrimes { p: 2141575169, g: 1571977166, s: 712760980 },
    SmallPrimes { p: 2141546497, g: 1107849376, s: 1250270109 },
    SmallPrimes { p: 2141515777, g: 196544219, s: 356001130 },
    SmallPrimes { p: 2141495297, g: 1733571506, s: 1060744866 },
    SmallPrimes { p: 2141483009, g: 321552363, s: 1168297026 },
    SmallPrimes { p: 2141458433, g: 505818251, s: 733225819 },
    SmallPrimes { p: 2141360129, g: 1026840098, s: 948342276 },
    SmallPrimes { p: 2141325313, g: 945133744, s: 2129965998 },
    SmallPrimes { p: 2141317121, g: 1871100260, s: 1843844634 },
    SmallPrimes { p: 2141286401, g: 1790639498, s: 1750465696 },
    SmallPrimes { p: 2141267969, g: 1376858592, s: 186160720 },
    SmallPrimes { p: 2141255681, g: 2129698296, s: 1876677959 },
    SmallPrimes { p: 2141243393, g: 2138900688, s: 1340009628 },
    SmallPrimes { p: 2141214721, g: 1933049835, s: 1087819477 },
    SmallPrimes { p: 2141212673, g: 1898664939, s: 1786328049 },
    SmallPrimes { p: 2141202433, g: 990234828, s: 940682169 },
    SmallPrimes { p: 2141175809, g: 1406392421, s: 993089586 },
    SmallPrimes { p: 2141165569, g: 1263518371, s: 289019479 },
    SmallPrimes { p: 2141073409, g: 1485624211, s: 507864514 },
    SmallPrimes { p: 2141052929, g: 1885134788, s: 311252465 },
    SmallPrimes { p: 2141040641, g: 1285021247, s: 280941862 },
    SmallPrimes { p: 2141028353, g: 1527610374, s: 375035110 },
    SmallPrimes { p: 2141011969, g: 1400626168, s: 164696620 },
    SmallPrimes { p: 2140999681, g: 632959608, s: 966175067 },
    SmallPrimes { p: 2140997633, g: 2045628978, s: 1290889438 },
    SmallPrimes { p: 2140993537, g: 1412755491, s: 375366253 },
    SmallPrimes { p: 2140942337, g: 719477232, s: 785367828 },
    SmallPrimes { p: 2140925953, g: 45224252, s: 836552317 },
    SmallPrimes { p: 2140917761, g: 1157376588, s: 1001839569 },
    SmallPrimes { p: 2140887041, g: 278480752, s: 2098732796 },
    SmallPrimes { p: 2140837889, g: 1663139953, s: 924094810 },
    SmallPrimes { p: 2140788737, g: 802501511, s: 2045368990 },
    SmallPrimes { p: 2140766209, g: 1820083885, s: 1800295504 },
    SmallPrimes { p: 2140764161, g: 1169561905, s: 2106792035 },
    SmallPrimes { p: 2140696577, g: 127781498, s: 1885987531 },
    SmallPrimes { p: 2140684289, g: 16014477, s: 1098116827 },
    SmallPrimes { p: 2140653569, g: 665960598, s: 1796728247 },
    SmallPrimes { p: 2140594177, g: 1043085491, s: 377310938 },
    SmallPrimes { p: 2140579841, g: 1732838211, s: 1504505945 },
    SmallPrimes { p: 2140569601, g: 302071939, s: 358291016 },
    SmallPrimes { p: 2140567553, g: 192393733, s: 1909137143 },
    SmallPrimes { p: 2140557313, g: 406595731, s: 1175330270 },
    SmallPrimes { p: 2140549121, g: 1748850918, s: 525007007 },
    SmallPrimes { p: 2140477441, g: 499436566, s: 1031159814 },
    SmallPrimes { p: 2140469249, g: 1886004401, s: 1029951320 },
    SmallPrimes { p: 2140426241, g: 1483168100, s: 1676273461 },
    SmallPrimes { p: 2140420097, g: 1779917297, s: 846024476 },
    SmallPrimes { p: 2140413953, g: 522948893, s: 1816354149 },
    SmallPrimes { p: 2140383233, g: 1931364473, s: 1296921241 },
    SmallPrimes { p: 2140366849, g: 1917356555, s: 147196204 },
    SmallPrimes { p: 2140354561, g: 16466177, s: 1349052107 },
    SmallPrimes { p: 2140348417, g: 1875366972, s: 1860485634 },
    SmallPrimes { p: 2140323841, g: 456498717, s: 1790256483 },
    SmallPrimes { p: 2140321793, g: 1629493973, s: 150031888 },
    SmallPrimes { p: 2140315649, g: 1904063898, s: 395510935 },
    SmallPrimes { p: 2140280833, g: 1784104328, s: 831417909 },
    SmallPrimes { p: 2140250113, g: 256087139, s: 697349101 },
    SmallPrimes { p: 2140229633, g: 388553070, s: 243875754 },
    SmallPrimes { p: 2140223489, g: 747459608, s: 1396270850 },
    SmallPrimes { p: 2140200961, g: 507423743, s: 1895572209 },
    SmallPrimes { p: 2140162049, g: 580106016, s: 2045297469 },
    SmallPrimes { p: 2140149761, g: 712426444, s: 785217995 },
    SmallPrimes { p: 2140137473, g: 1441607584, s: 536866543 },
    SmallPrimes { p: 2140119041, g: 346538902, s: 1740434653 },
    SmallPrimes { p: 2140090369, g: 282642885, s: 21051094 },
    SmallPrimes { p: 2140076033, g: 1407456228, s: 319910029 },
    SmallPrimes { p: 2140047361, g: 1619330500, s: 1488632070 },
    SmallPrimes { p: 2140041217, g: 2089408064, s: 2012026134 },
    SmallPrimes { p: 2140008449, g: 1705524800, s: 1613440760 },
    SmallPrimes { p: 2139924481, g: 1846208233, s: 1280649481 },
    SmallPrimes { p: 2139906049, g: 989438755, s: 1185646076 },
    SmallPrimes { p: 2139867137, g: 1522314850, s: 372783595 },
    SmallPrimes { p: 2139842561, g: 1681587377, s: 216848235 },
    SmallPrimes { p: 2139826177, g: 2066284988, s: 1784999464 },
    SmallPrimes { p: 2139824129, g: 480888214, s: 1513323027 },
    SmallPrimes { p: 2139789313, g: 847937200, s: 858192859 },
    SmallPrimes { p: 2139783169, g: 1642000434, s: 1583261448 },
    SmallPrimes { p: 2139770881, g: 940699589, s: 179702100 },
    SmallPrimes { p: 2139768833, g: 315623242, s: 964612676 },
    SmallPrimes { p: 2139666433, g: 331649203, s: 764666914 },
    SmallPrimes { p: 2139641857, g: 2118730799, s: 1313764644 },
    SmallPrimes { p: 2139635713, g: 519149027, s: 519212449 },
    SmallPrimes { p: 2139598849, g: 1526413634, s: 1769667104 },
    SmallPrimes { p: 2139574273, g: 551148610, s: 820739925 },
    SmallPrimes { p: 2139568129, g: 1386800242, s: 472447405 },
    SmallPrimes { p: 2139549697, g: 813760130, s: 1412328531 },
    SmallPrimes { p: 2139537409, g: 1615286260, s: 1609362979 },
    SmallPrimes { p: 2139475969, g: 1352559299, s: 1696720421 },
    SmallPrimes { p: 2139455489, g: 1048691649, s: 1584935400 },
    SmallPrimes { p: 2139432961, g: 836025845, s: 950121150 },
    SmallPrimes { p: 2139424769, g: 1558281165, s: 1635486858 },
    SmallPrimes { p: 2139406337, g: 1728402143, s: 1674423301 },
    SmallPrimes { p: 2139396097, g: 1727715782, s: 1483470544 },
    SmallPrimes { p: 2139383809, g: 1092853491, s: 1741699084 },
    SmallPrimes { p: 2139369473, g: 690776899, s: 1242798709 },
    SmallPrimes { p: 2139351041, g: 1768782380, s: 2120712049 },
    SmallPrimes { p: 2139334657, g: 1739968247, s: 1427249225 },
    SmallPrimes { p: 2139332609, g: 1547189119, s: 623011170 },
    SmallPrimes { p: 2139310081, g: 1346827917, s: 1605466350 },
    SmallPrimes { p: 2139303937, g: 369317948, s: 828392831 },
    SmallPrimes { p: 2139301889, g: 1560417239, s: 1788073219 },
    SmallPrimes { p: 2139283457, g: 1303121623, s: 595079358 },
    SmallPrimes { p: 2139248641, g: 1354555286, s: 573424177 },
    SmallPrimes { p: 2139240449, g: 60974056, s: 885781403 },
    SmallPrimes { p: 2139222017, g: 355573421, s: 1221054839 },
    SmallPrimes { p: 2139215873, g: 566477826, s: 1724006500 },
    SmallPrimes { p: 2139150337, g: 871437673, s: 1609133294 },
    SmallPrimes { p: 2139144193, g: 1478130914, s: 1137491905 },
    SmallPrimes { p: 2139117569, g: 1854880922, s: 964728507 },
    SmallPrimes { p: 2139076609, g: 202405335, s: 756508944 },
    SmallPrimes { p: 2139062273, g: 1399715741, s: 884826059 },
    SmallPrimes { p: 2139045889, g: 1051045798, s: 1202295476 },
    SmallPrimes { p: 2139033601, g: 1707715206, s: 632234634 },
    SmallPrimes { p: 2139006977, g: 2035853139, s: 231626690 },
    SmallPrimes { p: 2138951681, g: 183867876, s: 838350879 },
    SmallPrimes { p: 2138945537, g: 1403254661, s: 404460202 },
    SmallPrimes { p: 2138920961, g: 310865011, s: 1282911681 },
    SmallPrimes { p: 2138910721, g: 1328496553, s: 103472415 },
    SmallPrimes { p: 2138904577, g: 78831681, s: 993513549 },
    SmallPrimes { p: 2138902529, g: 1319697451, s: 1055904361 },
    SmallPrimes { p: 2138816513, g: 384338872, s: 1706202469 },
    SmallPrimes { p: 2138810369, g: 1084868275, s: 405677177 },
    SmallPrimes { p: 2138787841, g: 401181788, s: 1964773901 },
    SmallPrimes { p: 2138775553, g: 1850532988, s: 1247087473 },
    SmallPrimes { p: 2138767361, g: 874261901, s: 1576073565 },
    SmallPrimes { p: 2138757121, g: 1187474742, s: 993541415 },
    SmallPrimes { p: 2138748929, g: 1782458888, s: 1043206483 },
    SmallPrimes { p: 2138744833, g: 1221500487, s: 800141243 },
    SmallPrimes { p: 2138738689, g: 413465368, s: 1450660558 },
    SmallPrimes { p: 2138695681, g: 739045140, s: 342611472 },
    SmallPrimes { p: 2138658817, g: 1355845756, s: 672674190 },
    SmallPrimes { p: 2138644481, g: 608379162, s: 1538874380 },
    SmallPrimes { p: 2138632193, g: 1444914034, s: 686911254 },
    SmallPrimes { p: 2138607617, g: 484707818, s: 1435142134 },
    SmallPrimes { p: 2138591233, g: 539460669, s: 1290458549 },
    SmallPrimes { p: 2138572801, g: 2093538990, s: 2011138646 },
    SmallPrimes { p: 2138552321, g: 1149786988, s: 1076414907 },
    SmallPrimes { p: 2138546177, g: 840688206, s: 2108985273 },
    SmallPrimes { p: 2138533889, g: 209669619, s: 198172413 },
    SmallPrimes { p: 2138523649, g: 1975879426, s: 1277003968 },
    SmallPrimes { p: 2138490881, g: 1351891144, s: 1976858109 },
    SmallPrimes { p: 2138460161, g: 1817321013, s: 1979278293 },
    SmallPrimes { p: 2138429441, g: 1950077177, s: 203441928 },
    SmallPrimes { p: 2138400769, g: 908970113, s: 628395069 },
    SmallPrimes { p: 2138398721, g: 219890864, s: 758486760 },
    SmallPrimes { p: 2138376193, g: 1306654379, s: 977554090 },
    SmallPrimes { p: 2138351617, g: 298822498, s: 2004708503 },
    SmallPrimes { p: 2138337281, g: 441457816, s: 1049002108 },
    SmallPrimes { p: 2138320897, g: 1517731724, s: 1442269609 },
    SmallPrimes { p: 2138290177, g: 1355911197, s: 1647139103 },
    SmallPrimes { p: 2138234881, g: 531313247, s: 1746591962 },
    SmallPrimes { p: 2138214401, g: 1899410930, s: 781416444 },
    SmallPrimes { p: 2138202113, g: 1813477173, s: 1622508515 },
    SmallPrimes { p: 2138191873, g: 1086458299, s: 1025408615 },
    SmallPrimes { p: 2138183681, g: 1998800427, s: 827063290 },
    SmallPrimes { p: 2138173441, g: 1921308898, s: 749670117 },
    SmallPrimes { p: 2138103809, g: 1620902804, s: 2126787647 },
    SmallPrimes { p: 2138099713, g: 828647069, s: 1892961817 },
    SmallPrimes { p: 2138085377, g: 179405355, s: 1525506535 },
    SmallPrimes { p: 2138060801, g: 615683235, s: 1259580138 },
    SmallPrimes { p: 2138044417, g: 2030277840, s: 1731266562 },
    SmallPrimes { p: 2138042369, g: 2087222316, s: 1627902259 },
    SmallPrimes { p: 2138032129, g: 126388712, s: 1108640984 },
    SmallPrimes { p: 2138011649, g: 715026550, s: 1017980050 },
    SmallPrimes { p: 2137993217, g: 1693714349, s: 1351778704 },
    SmallPrimes { p: 2137888769, g: 1289762259, s: 1053090405 },
    SmallPrimes { p: 2137853953, g: 199991890, s: 1254192789 },
    SmallPrimes { p: 2137833473, g: 941421685, s: 896995556 },
    SmallPrimes { p: 2137817089, g: 750416446, s: 1251031181 },
    SmallPrimes { p: 2137792513, g: 798075119, s: 368077456 },
    SmallPrimes { p: 2137786369, g: 878543495, s: 1035375025 },
    SmallPrimes { p: 2137767937, g: 9351178, s: 1156563902 },
    SmallPrimes { p: 2137755649, g: 1382297614, s: 1686559583 },
    SmallPrimes { p: 2137724929, g: 1345472850, s: 1681096331 },
    SmallPrimes { p: 2137704449, g: 834666929, s: 630551727 },
    SmallPrimes { p: 2137673729, g: 1646165729, s: 1892091571 },
    SmallPrimes { p: 2137620481, g: 778943821, s: 48456461 },
    SmallPrimes { p: 2137618433, g: 1730837875, s: 1713336725 },
    SmallPrimes { p: 2137581569, g: 805610339, s: 1378891359 },
    SmallPrimes { p: 2137538561, g: 204342388, s: 1950165220 },
    SmallPrimes { p: 2137526273, g: 1947629754, s: 1500789441 },
    SmallPrimes { p: 2137516033, g: 719902645, s: 1499525372 },
    SmallPrimes { p: 2137491457, g: 230451261, s: 556382829 },
    SmallPrimes { p: 2137440257, g: 979573541, s: 412760291 },
    SmallPrimes { p: 2137374721, g: 927841248, s: 1954137185 },
    SmallPrimes { p: 2137362433, g: 1243778559, s: 861024672 },
    SmallPrimes { p: 2137313281, g: 1341338501, s: 980638386 },
    SmallPrimes { p: 2137311233, g: 937415182, s: 1793212117 },
    SmallPrimes { p: 2137255937, g: 795331324, s: 1410253405 },
    SmallPrimes { p: 2137243649, g: 150756339, s: 1966999887 },
    SmallPrimes { p: 2137182209, g: 163346914, s: 1939301431 },
    SmallPrimes { p: 2137171969, g: 1952552395, s: 758913141 },
    SmallPrimes { p: 2137159681, g: 570788721, s: 218668666 },
    SmallPrimes { p: 2137147393, g: 1896656810, s: 2045670345 },
    SmallPrimes { p: 2137141249, g: 358493842, s: 518199643 },
    SmallPrimes { p: 2137139201, g: 1505023029, s: 674695848 },
    SmallPrimes { p: 2137133057, g: 27911103, s: 830956306 },
    SmallPrimes { p: 2137122817, g: 439771337, s: 1555268614 },
    SmallPrimes { p: 2137116673, g: 790988579, s: 1871449599 },
    SmallPrimes { p: 2137110529, g: 432109234, s: 811805080 },
    SmallPrimes { p: 2137102337, g: 1357900653, s: 1184997641 },
    SmallPrimes { p: 2137098241, g: 515119035, s: 1715693095 },
    SmallPrimes { p: 2137090049, g: 408575203, s: 2085660657 },
    SmallPrimes { p: 2137085953, g: 2097793407, s: 1349626963 },
    SmallPrimes { p: 2137055233, g: 1556739954, s: 1449960883 },
    SmallPrimes { p: 2137030657, g: 1545758650, s: 1369303716 },
    SmallPrimes { p: 2136987649, g: 332602570, s: 103875114 },
    SmallPrimes { p: 2136969217, g: 1499989506, s: 1662964115 },
    SmallPrimes { p: 2136924161, g: 857040753, s: 4738842 },
    SmallPrimes { p: 2136895489, g: 1948872712, s: 570436091 },
    SmallPrimes { p: 2136893441, g: 58969960, s: 1568349634 },
    SmallPrimes { p: 2136887297, g: 2127193379, s: 273612548 },
    SmallPrimes { p: 2136850433, g: 111208983, s: 1181257116 },
    SmallPrimes { p: 2136809473, g: 1627275942, s: 1680317971 },
    SmallPrimes { p: 2136764417, g: 1574888217, s: 14011331 },
    SmallPrimes { p: 2136741889, g: 14011055, s: 1129154251 },
    SmallPrimes { p: 2136727553, g: 35862563, s: 1838555253 },
    SmallPrimes { p: 2136721409, g: 310235666, s: 1363928244 },
    SmallPrimes { p: 2136698881, g: 1612429202, s: 1560383828 },
    SmallPrimes { p: 2136649729, g: 1138540131, s: 800014364 },
    SmallPrimes { p: 2136606721, g: 602323503, s: 1433096652 },
    SmallPrimes { p: 2136563713, g: 182209265, s: 1919611038 },
    SmallPrimes { p: 2136555521, g: 324156477, s: 165591039 },
    SmallPrimes { p: 2136549377, g: 195513113, s: 217165345 },
    SmallPrimes { p: 2136526849, g: 1050768046, s: 939647887 },
    SmallPrimes { p: 2136508417, g: 1886286237, s: 1619926572 },
    SmallPrimes { p: 2136477697, g: 609647664, s: 35065157 },
    SmallPrimes { p: 2136471553, g: 679352216, s: 1452259468 },
    SmallPrimes { p: 2136457217, g: 128630031, s: 824816521 },
    SmallPrimes { p: 2136422401, g: 19787464, s: 1526049830 },
    SmallPrimes { p: 2136420353, g: 698316836, s: 1530623527 },
    SmallPrimes { p: 2136371201, g: 1651862373, s: 1804812805 },
    SmallPrimes { p: 2136334337, g: 326596005, s: 336977082 },
    SmallPrimes { p: 2136322049, g: 63253370, s: 1904972151 },
    SmallPrimes { p: 2136297473, g: 312176076, s: 172182411 },
    SmallPrimes { p: 2136248321, g: 381261841, s: 369032670 },
    SmallPrimes { p: 2136242177, g: 358688773, s: 1640007994 },
    SmallPrimes { p: 2136229889, g: 512677188, s: 75585225 },
    SmallPrimes { p: 2136219649, g: 2095003250, s: 1970086149 },
    SmallPrimes { p: 2136207361, g: 1909650722, s: 537760675 },
    SmallPrimes { p: 2136176641, g: 1334616195, s: 1533487619 },
    SmallPrimes { p: 2136158209, g: 2096285632, s: 1793285210 },
    SmallPrimes { p: 2136143873, g: 1897347517, s: 293843959 },
    SmallPrimes { p: 2136133633, g: 923586222, s: 1022655978 },
    SmallPrimes { p: 2136096769, g: 1464868191, s: 1515074410 },
    SmallPrimes { p: 2136094721, g: 2020679520, s: 2061636104 },
    SmallPrimes { p: 2136076289, g: 290798503, s: 1814726809 },
    SmallPrimes { p: 2136041473, g: 156415894, s: 1250757633 },
    SmallPrimes { p: 2135996417, g: 297459940, s: 1132158924 },
    SmallPrimes { p: 2135955457, g: 538755304, s: 1688831340 },
    SmallPrimes { p: 0, g: 0, s: 0 }
];