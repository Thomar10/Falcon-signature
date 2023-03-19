use crate::mp31::{mp_add, mp_half, mp_montymul, mp_sub, PRIMES, tbmask};

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