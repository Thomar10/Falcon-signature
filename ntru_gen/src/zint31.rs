use crate::mp31::{lzcnt, mp_add, mp_half, mp_montymul, mp_ninv31, mp_sub, PRIMES, tbmask};

pub fn zint_mul_small(m: &mut [u32], len: usize, x: u32) -> u32 {
    let mut cc: u32 = 0;
    for u in 0..len {
        let z: u64 = (m[u] as u64) * (x as u64) + cc as u64;
        m[u] = (z as u32) & 0x7FFFFFFF;
        cc = (z >> 31) as u32;
    }
    cc
}

pub fn zint_mod_small_unsigned(d: &[u32], len: usize, stride: usize, p: u32, p0i: u32, r2: u32) -> u32 {
    let mut x: u32 = 0;
    let z = mp_half(r2, p);
    let mut d_index = len * stride;
    for _ in (0..len).rev() {
        d_index -= stride;
        let mut w: u32 = d[d_index].wrapping_sub(p);
        w = w.wrapping_add(p & tbmask(w));
        x = mp_montymul(x, z, p, p0i);
        x = mp_add(x, w, p);
    }
    x
}

pub fn zint_add_mul_small(x: &mut [u32], len: usize, xstride: usize, y: &[u32], s: u32) {
    let mut cc: u32 = 0;
    let mut x_index = 0;
    for u in 0..len {
        let xw = x[x_index];
        let yw = y[u];
        let z: u64 = (yw as u64) * (s as u64) + (xw as u64) + (cc as u64);
        x[x_index] = (z as u32) & 0x7FFFFFFF;
        cc = (z >> 31) as u32;
        x_index += xstride;
    }
    x[x_index] = cc;
}

pub fn zint_norm_zero(x: &mut [u32], len: usize, xstride: usize, p: &[u32]) {
    let mut r: u32 = 0;
    let mut bb: u32 = 0;
    let mut x_index = len * xstride;
    for u in (0..len).rev() {
        x_index -= xstride;
        let wx = x[x_index];
        let wp = (p[u] >> 1) | (bb << 30);
        bb = p[u] & 1;

        let mut cc: u32 = wp.wrapping_sub(wx);
        cc = ((!cc).wrapping_add(1) >> 31) | (!(cc >> 31)).wrapping_add(1);

        r |= cc & ((r & 1).wrapping_sub(1));
    }

    let mut cc: u32 = 0;
    let m = tbmask(r);
    for j in 0..len {
        let mut xw = x[x_index];
        let w = xw.wrapping_sub(p[j]).wrapping_sub(cc);
        cc = w >> 31;
        xw ^= ((w & 0x7FFFFFFF) ^ xw) & m;
        x[x_index] = xw;
        x_index += xstride;
    }
}

pub fn zint_rebuild_crt(xx: &mut [u32], xlen: usize, n: usize, num_sets: usize, normalized_signed: bool, tmp: &mut [u32]) {
    let mut uu = 0;
    tmp[0] = PRIMES[0].p;
    for u in 1..xlen {
        let (p, p0i, s, r2): (u32, u32, u32, u32);

        p = PRIMES[u].p;
        p0i = PRIMES[u].p0i;
        r2 = PRIMES[u].r2;
        s = PRIMES[u].s;
        uu += n;
        let mut kk = 0;
        for _ in 0..num_sets {
            for v in 0..n {
                let xp = xx[kk + v + uu];
                let x_off = xx.split_at_mut(kk + v).1;
                let xq = zint_mod_small_unsigned(x_off, u, n, p, p0i, r2);
                let xr = mp_montymul(s, mp_sub(xp, xq, p), p, p0i);

                zint_add_mul_small(xx.split_at_mut(kk + v).1, u, n, tmp, xr);
            }
            kk += n * xlen;
        }
        tmp[u] = zint_mul_small(tmp, u, p);
    }

    if normalized_signed {
        let mut kk = 0;
        for _ in 0..num_sets {
            for v in 0..n {
                let (_, x) = xx.split_at_mut(kk + v);
                zint_norm_zero(x, xlen, n, tmp);
            }
            kk += n * xlen;
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


pub fn zint_bezout(u: &mut [u32], v: &mut [u32], x: &[u32], y: &[u32], len: usize, tmp: &mut [u32]) -> bool {
    let (x0i, y0i): (u32, u32);
    let mut num: u32;
    let mut j: usize;

    if len == 0 {
        return false;
    }

    x0i = mp_ninv31(x[0]);
    y0i = mp_ninv31(y[0]);

    let (u0, inter) = tmp.split_at_mut(len);
    let (v0, inter) = inter.split_at_mut(len);
    let (a, b) = inter.split_at_mut(len);

    a.copy_from_slice(&x);
    b.copy_from_slice(&y);
    u0[0] = 1;
    u0[1..len].fill(0);
    v0.fill(0);
    u.copy_from_slice(&y);
    v.copy_from_slice(&x);
    v[0] -= 1;


    num = 62 * (len as u32) + 31;
    while num >= 30 {
        let mut c0 = 0xFFFFFFFF;
        let mut c1 = 0xFFFFFFFF;
        let mut cp: u32 = 0xFFFFFFFF;
        let mut a0 = 0;
        let mut a1 = 0;
        let mut b0 = 0;
        let mut b1 = 0;
        j = len;
        while j > 0 {
            j -= 1;
            let aw = a[j];
            let bw = b[j];
            a0 ^= (a0 ^ aw) & c0;
            a1 ^= (a1 ^ aw) & c1;
            b0 ^= (b0 ^ bw) & c0;
            b1 ^= (b1 ^ bw) & c1;
            cp = c0;
            c0 = c1;
            c1 &= (((aw | bw).wrapping_add(0x7FFFFFFF)) >> 31).wrapping_sub(1u32);
        }
        let s = lzcnt(a1 | b1 | ((cp & c0) >> 1));
        let mut ha = (a1 << s) | (a0 >> (31 - s));
        let mut hb = (b1 << s) | (b0 >> (31 - s));

        ha ^= cp & (ha ^ a1);
        hb ^= cp & (hb ^ b1);

        ha &= !c0;
        hb &= !c0;

        let mut xa = ((ha as u64) << 31) | a[0] as u64;
        let mut xb = ((hb as u64) << 31) | b[0] as u64;

        let mut fg0 = 1;
        let mut fg1: u64 = 1 << 32;
        for _ in 0..31 {
            let a_odd = (!(xa & 1)).wrapping_add(1);
            let mut dx = xa.wrapping_sub(xb);
            dx = ((dx as i64) >> 63) as u64;
            let swap = a_odd & dx;
            let t1 = swap & (xa ^ xb);
            xa ^= t1;
            xb ^= t1;
            let t2 = swap & (fg0 ^ fg1);
            fg0 ^= t2;
            fg1 ^= t2;
            xa = xa.wrapping_sub(a_odd & xb);
            fg0 = fg0.wrapping_sub(a_odd & fg1);
            xa >>= 1;
            fg1 <<= 1;
        }

        fg0 = fg0.wrapping_add(0x7FFFFFFF7FFFFFFF as u64);
        fg1 = fg1.wrapping_add(0x7FFFFFFF7FFFFFFF as u64);
        let mut f0 = (fg0 & 0xFFFFFFFF) as i64 - 0x7FFFFFFF;
        let mut g0 = (fg0 >> 32) as i64 - 0x7FFFFFFF;
        let mut f1 = (fg1 & 0xFFFFFFFF) as i64 - 0x7FFFFFFF;
        let mut g1 = (fg1 >> 32) as i64 - 0x7FFFFFFF;


        let negab = zint_co_reduce(a, b, len, f0, g0, f1, g1);
        f0 -= (f0 + f0) & -((negab & 1) as i64);
        g0 -= (g0 + g0) & -((negab & 1) as i64);
        f1 -= (f1 + f1) & -((negab >> 1) as i64);
        g1 -= (g1 + g1) & -((negab >> 1) as i64);
        zint_co_reduce_mod(u0, u, y, len, y0i, f0, g0, f1, g1);
        zint_co_reduce_mod(v0, v, x, len, x0i, f0, g0, f1, g1);
        num -= 31;
    }
    let mut r = b[0] ^ 1;
    for j in 1..len {
        r |= b[j];
    }
    r |= (x[0] & y[0] & 1) ^ 1;
    (1 - ((r | (!r).wrapping_add(1)) >> 31)) != 0
}


pub fn zint_co_reduce_mod(a: &mut [u32], b: &mut [u32], m: &[u32], len: usize, m0i: u32, xa: i64, xb: i64, ya: i64, yb: i64) {
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

pub fn zint_finish_mod(a: &mut [u32], len: usize, m: &[u32], neg: u32) {
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

#[inline(always)]
pub fn zint_mod_small_signed(d: &[u32], len: usize, stride: usize, p: u32, p0i: u32, r2: u32, rx: u32) -> u32 {
    if len == 0 {
        return 0;
    }
    let z = zint_mod_small_unsigned(d, len, stride, p, p0i, r2);
    mp_sub(z, rx & (!(d[(len - 1) * stride] >> 30)).wrapping_add(1), p)
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


pub fn zint_add_scaled_mul_small(x: &mut [u32], xlen: usize, y: &[u32], mut ylen: usize, stride: usize, k: i32, sch: usize, scl: u32) {
    let mut cc: i32;
    let mut tw: u32;
    if ylen == 0 {
        return;
    }
    let ysign = (!(y[stride * (ylen - 1)] >> 30)).wrapping_add(1) >> 1;
    tw = 0;
    cc = 0;
    let mut x_index = sch * stride;
    let mut y_index = 0;
    for _ in sch..xlen {
        let wy;
        if ylen > 0 {
            wy = y[y_index];
            y_index += stride;
            ylen -= 1;
        } else {
            wy = ysign;
        }
        let wys: u32 = ((wy << scl) & 0x7FFFFFFF) | tw;
        tw = wy >> (31 - scl);

        let z = ((wys as i64) * (k as i64) + (x[x_index] as i64) + (cc as i64)) as u64;
        x[x_index] = (z as u32) & 0x7FFFFFFF;

        let ccu = (z >> 31) as u32;
        cc = ccu as i32;
        x_index += stride;
    }
}

pub fn zint_sub_scaled(x: &mut [u32], xlen: usize, y: &[u32], ylen: usize, stride: usize, sch: usize, scl: u32) {
    if ylen == 0 {
        return;
    }

    let ysign: u32 = (!(y[stride * (ylen - 1)] >> 30)).wrapping_add(1) >> 1;
    let mut tw = 0;
    let mut cc = 0;
    let mut x_index = sch * stride;
    let mut y_index = 0;
    for _ in sch..xlen {
        let (w, wy, wys): (u32, u32, u32);

        if ylen > 0 {
            wy = y[y_index];
            y_index += stride
        } else {
            wy = ysign;
        }

        wys = ((wy << scl) & 0x7FFFFFFF) | tw;
        tw = wy >> (31 - scl);

        w = x[x_index].wrapping_sub(wys).wrapping_sub(cc);
        x[x_index] = w & 0x7FFFFFFF;
        cc = w >> 31;
        x_index += stride;
    }
}