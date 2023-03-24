#![allow(non_snake_case)]

use crate::fxp::fxr;
use crate::mp31::{lzcnt, mp_add, mp_half, mp_intt, mp_mkgm, mp_mkgmigm, mp_mkigm, mp_montymul, mp_norm, mp_ntt, mp_set, mp_sub, PRIMES, tbmask};
use crate::zint31::{zint_add_scaled_mul_small, zint_rebuild_crt, zint_sub_scaled};

pub fn poly_mp_set_small(logn: usize, d: &mut [u32], f: &[i8], p: u32) {
    for u in 0..(1 << logn) {
        d[u] = mp_set(f[u] as i32, p);
    }
}

pub fn poly_mp_set(logn: usize, f: &mut [u32], p: u32) {
    for u in 0..(1 << logn) {
        let mut x = f[u];
        x |= (x & 0x40000000) << 1;
        f[u] = mp_set(x as i32, p);
    }
}

pub fn poly_mp_norm(logn: usize, f: &mut [u32], p: u32) {
    for u in 0..(1 << logn) {
        f[u] = (mp_norm(f[u], p) & 0x7FFFFFFF) as u32;
    }
}

pub fn poly_big_to_small(logn: usize, d: &mut [i8], s: &[u32], lim: i32) -> bool {
    let n: usize = 1 << logn;
    for u in 0..n {
        let mut x = s[u];
        x |= (x & 0x40000000) << 1;
        let z: i32 = x as i32;
        if z < -lim || z > lim {
            return false;
        }
        d[u] = z as i8;
    }
    true
}

pub fn poly_max_bitlength(logn: usize, f: &[u32], flen: usize) -> u32 {
    if flen == 0 {
        return 0;
    }
    let n = 1 << logn;
    let mut t: u32 = 0;
    let mut tk: u32 = 0;
    let mut f_index = 0;
    for _ in 0..n {
        let m = (!(f[((flen - 1) << logn) + f_index] >> 30)).wrapping_add(1) & 0x7FFFFFFF;
        let mut c = 0;
        let mut ck = 0;
        for v in 0..flen {
            let w = f[(v << logn) + f_index] ^ m;
            let nz = ((w.wrapping_sub(1)) >> 31).wrapping_sub(1);
            c ^= nz & (c ^ w);
            ck ^= nz & (ck ^ (v as u32));
        }

        let rr = tbmask((tk.wrapping_sub(ck)) |
            (((tk ^ ck).wrapping_sub(1)) & (t.wrapping_sub(c))));
        t ^= rr & (t ^ c);
        tk ^= rr & (tk ^ ck);
        f_index += 1;
    }

    31 * tk + 32 - lzcnt(t)
}

pub fn poly_big_to_fixed(logn: usize, d: &mut [fxr], f: &[u32], len: usize, sc: u32) {
    let n = 1 << logn;
    if len == 0 {
        d[..n].fill(0);
        return;
    }

    let (mut sch, mut scl) = divrev31(sc);
    let z: u32 = (scl.wrapping_sub(1)) >> 31;
    sch = sch.wrapping_sub(z);
    scl |= 31 & z.wrapping_neg();
    let t0 = (sch.wrapping_sub(1) & 0xFFFFFF) as u32;
    let t1 = sch & 0xFFFFFF;
    let t2 = (sch.wrapping_add(1) & 0xFFFFFF) as u32;
    for u in 0..n {
        let (_, f) = f.split_at(u);
        let mut w0: u32 = 0;
        let mut w1: u32 = 0;
        let mut w2: u32 = 0;
        for v in 0..len {
            let w = f[(v << logn)];
            let t = (v & 0xFFFFFF) as u32;
            w0 |= w & ((((t ^ t0).wrapping_sub(1)) >> 31) as u32).wrapping_neg();
            w1 |= w & ((((t ^ t1).wrapping_sub(1)) >> 31) as u32).wrapping_neg();
            w2 |= w & ((((t ^ t2).wrapping_sub(1)) >> 31) as u32).wrapping_neg();
        }
        let ws = (f[((len - 1) << logn)] >> 30).wrapping_neg() >> 1;
        w0 |= ws & ((((len as u32).wrapping_sub(sch)) >> 31) as u32).wrapping_neg();
        w1 |= ws & (((len as u32).wrapping_sub(sch).wrapping_sub(1) as u32) >> 31).wrapping_neg();
        w2 |= ws & (((len as u32).wrapping_sub(sch).wrapping_sub(2) as u32) >> 31).wrapping_neg();


        w2 |= ((w2 & 0x40000000) as u32) << 1;
        let xl: u32 = (w0 >> (scl - 1)) | (w1 << (32 - scl));
        let xh: u32 = (w1 >> scl) | (w2 << (31 - scl));
        d[u] = (xl as u64) | ((xh as u64) << 32);
    }
}

pub fn poly_sub_scaled(logn: usize, F: &mut [u32], mut Flen: usize, f: &[u32], flen: usize, k: &[i32], sc: u32) {
    if flen == 0 {
        return;
    }
    let (sch, scl) = divrev31(sc);
    if sch >= Flen as u32 {
        return;
    }
    let n = 1 << logn;
    let F = F.split_at_mut((sch as usize) << logn).1;
    Flen -= sch as usize;
    for u in 0..n {
        let mut kf = -k[u];
        let mut x = F.split_at_mut(u).1;
        for v in 0..n {
            zint_add_scaled_mul_small(x, Flen, f.split_at(v).1, flen, n, kf, 0, scl);
            if u + v == n - 1 {
                x = F;
                kf = -kf;
            } else {
                x = x.split_at_mut(1).1;
            }
        }
    }
}

pub fn poly_sub_scaled_ntt(logn: usize, F: &mut [u32], Flen: usize, f: &[u32], flen: usize, k: &[i32], sc: u32, tmp: &mut [u32]) {
    let n = 1 << logn;
    let tlen = flen + 1;
    let (gm, inter) = tmp.split_at_mut(n);
    let (igm, inter) = inter.split_at_mut(n);
    let (fk, t1) = inter.split_at_mut(tlen << logn);
    let (sch, scl) = divrev31(sc);
    for u in 0..tlen {
        let p = PRIMES[u].p;
        let p0i = PRIMES[u].p0i;
        let r2 = PRIMES[u].r2;
        mp_mkgmigm(logn, gm, igm, PRIMES[u].g, PRIMES[u].ig, p, p0i);
        for v in 0..n {
            t1[v] = mp_set(k[v], p);
        }
        mp_ntt(logn, t1, gm, p, p0i);
        let (_, fs) = f.split_at(u << logn);
        let (_, ff) = fk.split_at_mut(u << logn);
        for v in 0..n {
            ff[v] = mp_montymul(mp_montymul(t1[v],
                                            fs[v], p, p0i), r2, p, p0i);
        }
        mp_intt(logn, ff, igm, p, p0i);
    }
    zint_rebuild_crt(fk, tlen, n, 1, true, t1);
    for u in 0..n {
        zint_sub_scaled(F.split_at_mut(u).1, Flen, fk.split_at_mut(u).1, tlen, n, sch as usize, scl);
    }
}


pub fn poly_sub_kfg_scaled_depth1(logn_top: usize, F: &mut [u32], G: &mut [u32], FGlen: usize, k: &mut [u32], sc: u32, f: &[i8], g: &[i8], tmp: &mut [u32]) {
    let logn = logn_top - 1;
    let n = 1usize << logn as usize;
    let hn = n >> 1;
    let (gm, inter) = tmp.split_at_mut(n);
    let (t1, t2) = inter.split_at_mut(n);

    if FGlen == 1 {
        let p = PRIMES[0].p;
        for u in 0..n {
            let mut xf = F[u];
            let mut xg = G[u];
            xf |= (xf & 0x40000000) << 1;
            xg |= (xg & 0x40000000) << 1;
            F[u] = mp_set(xf as i32, p);
            G[u] = mp_set(xg as i32, p);
        }
    } else {
        let p0 = PRIMES[0].p;
        let p0_0i = PRIMES[0].p0i;
        let z0 = mp_half(PRIMES[0].r2, p0);
        let p1 = PRIMES[1].p;
        let p1_0i = PRIMES[1].p0i;
        let z1 = mp_half(PRIMES[1].r2, p1);
        for u in 0..n {
            let xl = F[u];
            let xh = F[u + n] | ((F[u + n] & 0x40000000) << 1);
            let yl0 = xl - (p0 & !tbmask(xl.wrapping_sub(p0)));
            let yh0 = mp_set(xh as i32, p0);
            let r0 = mp_add(yl0, mp_montymul(yh0, z0, p0, p0_0i), p0);
            let yl1 = xl - (p1 & !tbmask(xl.wrapping_sub(p1)));
            let yh1 = mp_set(xh as i32, p1);
            let r1 = mp_add(yl1, mp_montymul(yh1, z1, p1, p1_0i), p1);
            F[u] = r0;
            F[u + n] = r1;

            let xl = G[u];
            let xh = G[u + n] | ((G[u + n] & 0x40000000) << 1);
            let yl0 = xl - (p0 & !tbmask(xl.wrapping_sub(p0)));
            let yh0 = mp_set(xh as i32, p0);
            let r0 = mp_add(yl0, mp_montymul(yh0, z0, p0, p0_0i), p0);
            let yl1 = xl - (p1 & !tbmask(xl.wrapping_sub(p1)));
            let yh1 = mp_set(xh as i32, p1);
            let r1 = mp_add(yl1, mp_montymul(yh1, z1, p1, p1_0i), p1);
            G[u] = r0;
            G[u + n] = r1;
        }
    }

    for u in 0..FGlen {
        let p = PRIMES[u].p;
        let p0i = PRIMES[u].p0i;
        let r2 = PRIMES[u].r2;
        let r3 = mp_montymul(r2, r2, p, p0i);
        mp_mkgm(logn, gm, PRIMES[u].g, p, p0i);
        let mut scv = mp_montymul(
            1 << (sc & 31), r2, p, p0i);
        let mut m = sc >> 5;
        while m > 0 {
            scv = mp_montymul(scv, r2, p, p0i);
            m -= 1;
        }
        for v in 0..n {
            let x = mp_set(k[v] as i32, p);
            k[v] = mp_montymul(scv, x, p, p0i);
        }
        mp_ntt(logn, k, gm, p, p0i);
        let (_, Fu) = F.split_at_mut(u << logn);
        let (_, Gu) = G.split_at_mut(u << logn);
        mp_ntt(logn, Fu, gm, p, p0i);
        mp_ntt(logn, Gu, gm, p, p0i);
        for v in 0..n {
            t1[v] = mp_set(f[(v << 1) + 0] as i32, p);
            t2[v] = mp_set(f[(v << 1) + 1] as i32, p);
        }
        mp_ntt(logn, t1, gm, p, p0i);
        mp_ntt(logn, t2, gm, p, p0i);
        for v in 0..hn {
            let mut xe0 = t1[(v << 1) + 0];
            let mut xe1 = t1[(v << 1) + 1];
            let mut xo0 = t2[(v << 1) + 0];
            let mut xo1 = t2[(v << 1) + 1];
            let xv0 = gm[hn + v];
            let xv1 = p - xv0;
            xe0 = mp_montymul(xe0, xe0, p, p0i);
            xe1 = mp_montymul(xe1, xe1, p, p0i);
            xo0 = mp_montymul(xo0, xo0, p, p0i);
            xo1 = mp_montymul(xo1, xo1, p, p0i);
            let xf0 = mp_sub(xe0,
                             mp_montymul(xo0, xv0, p, p0i), p);
            let xf1 = mp_sub(xe1,
                             mp_montymul(xo1, xv1, p, p0i), p);

            let xkf0 = mp_montymul(
                mp_montymul(xf0, k[(v << 1) + 0], p, p0i),
                r3, p, p0i);
            let xkf1 = mp_montymul(
                mp_montymul(xf1, k[(v << 1) + 1], p, p0i),
                r3, p, p0i);
            Fu[(v << 1) + 0] = mp_sub(Fu[(v << 1) + 0], xkf0, p);
            Fu[(v << 1) + 1] = mp_sub(Fu[(v << 1) + 1], xkf1, p);
        }
        for v in 0..n {
            t1[v] = mp_set(g[(v << 1) + 0] as i32, p);
            t2[v] = mp_set(g[(v << 1) + 1] as i32, p);
        }
        mp_ntt(logn, t1, gm, p, p0i);
        mp_ntt(logn, t2, gm, p, p0i);
        for v in 0..hn {
            let mut xe0 = t1[(v << 1) + 0];
            let mut xe1 = t1[(v << 1) + 1];
            let mut xo0 = t2[(v << 1) + 0];
            let mut xo1 = t2[(v << 1) + 1];
            let xv0 = gm[hn + v];
            let xv1 = p - xv0;
            xe0 = mp_montymul(xe0, xe0, p, p0i);
            xe1 = mp_montymul(xe1, xe1, p, p0i);
            xo0 = mp_montymul(xo0, xo0, p, p0i);
            xo1 = mp_montymul(xo1, xo1, p, p0i);
            let xg0 = mp_sub(xe0,
                             mp_montymul(xo0, xv0, p, p0i), p);
            let xg1 = mp_sub(xe1,
                             mp_montymul(xo1, xv1, p, p0i), p);

            let xkg0 = mp_montymul(
                mp_montymul(xg0, k[(v << 1) + 0], p, p0i),
                r3, p, p0i);
            let xkg1 = mp_montymul(
                mp_montymul(xg1, k[(v << 1) + 1], p, p0i),
                r3, p, p0i);
            Gu[(v << 1) + 0] = mp_sub(Gu[(v << 1) + 0], xkg0, p);
            Gu[(v << 1) + 1] = mp_sub(Gu[(v << 1) + 1], xkg1, p);
        }

        mp_mkigm(logn, t1, PRIMES[u].ig, p, p0i);
        mp_intt(logn, Fu, t1, p, p0i);
        mp_intt(logn, Gu, t1, p, p0i);
        if (u + 1) < FGlen {
            mp_intt(logn, k, t1, p, p0i);
            scv = 1u32 << ((!sc).wrapping_add(1) & 31);
            let mut m = sc >> 5;
            while m > 0 {
                scv = mp_montymul(scv, 1, p, p0i);
                m -= 1;
            }
            for v in 0..n {
                k[v] = mp_norm(
                    mp_montymul(scv, k[v], p, p0i), p) as u32;
            }
        }
    }
    if FGlen == 1 {
        let p = PRIMES[0].p;
        for u in 0..n {
            F[u] =
                (mp_norm(F[u], p) & 0x7FFFFFFF) as u32;
            G[u] =
                (mp_norm(G[u], p) & 0x7FFFFFFF) as u32;
        }
    } else {
        let p0 = PRIMES[0].p;
        let p1 = PRIMES[1].p;
        let p1_0i = PRIMES[1].p0i;
        let s = PRIMES[1].s;
        let pp: u64 = (p0 as u64) * (p1 as u64);
        let hpp: u64 = pp >> 1;
        for u in 0..n {
            let x0 = F[u];
            let x1 = F[u + n];
            let x0m1 = x0 - (p1 & !tbmask(x0.wrapping_sub(p1)));
            let y = mp_montymul(
                mp_sub(x1, x0m1, p1), s, p1, p1_0i);
            let mut z: u64 = (x0 as u64) + (p0 as u64) * (y as u64);
            z = z.wrapping_sub(pp & (!((hpp.wrapping_sub(z)) >> 63)).wrapping_add(1));
            F[u] = (z & 0x7FFFFFFF) as u32;
            F[u + n] = ((z >> 31) & 0x7FFFFFFF) as u32;
        }
        for u in 0..n {
            let x0 = G[u];
            let x1 = G[u + n];
            let x0m1 = x0 - (p1 & !tbmask(x0.wrapping_sub(p1)));
            let y = mp_montymul(
                mp_sub(x1, x0m1, p1), s, p1, p1_0i);
            let mut z = (x0 as u64) + (p0 as u64) * (y as u64);
            z = z.wrapping_sub(pp & (!((hpp.wrapping_sub(z)) >> 63)).wrapping_add(1));
            G[u] = (z & 0x7FFFFFFF) as u32;
            G[u + n] = ((z >> 31) & 0x7FFFFFFF) as u32;
        }
    }
}

pub fn poly_is_invertible(logn: usize, f: &[i8], p: u32, p0i: u32, s: u32, r: u32, rm: u32, rs: u32, tmp: &mut [u32]) -> bool {
    let n = 1 << logn;
    let (t1, t2) = tmp.split_at_mut(n);
    mp_mkgm(logn, t1, s, p, p0i);
    for u in 0..n {
        t2[u] = mp_set(f[u] as i32, p);
    }
    mp_ntt(logn, t2, t1, p, p0i);
    let mut b: u32 = 0;
    for u in 0..n {
        let mut x = t2[u];

        let y = (((x as u64) * (rm as u64)) >> rs) as u32;
        x -= r * y;
        b |= x.wrapping_sub(1);
    }
    (1 - (b >> 31)) != 0
}


pub fn poly_is_invertible_ext(logn: usize, f: &[i8], r1: u32, r2: u32, p: u32, p0i: u32, s: u32, r1m: u32, r1s: u32, r2m: u32, r2s: u32, tmp: &mut [u32]) -> bool {
    let n = 1 << logn;
    let (t1, t2) = tmp.split_at_mut(n);
    mp_mkgm(logn, t1, s, p, p0i);
    for u in 0..n {
        t2[u] = mp_set(f[u] as i32, p);
    }
    mp_ntt(logn, t2, t1, p, p0i);
    let mut b: u32 = 0;
    for u in 0..n {
        let x = t2[u];

        let y1 = (((x as u64) * (r1m as u64)) >> r1s) as u32;
        b |= (x - r1 * y1) - 1;

        let y2 = (((x as u64) * (r2m as u64)) >> r2s) as u32;
        b |= (x - r2 * y2) - 1;
    }
    (1 - (b >> 31)) != 0
}

pub fn poly_sqnorm(logn: usize, f: &[i8]) -> u32 {
    let n = 1 << logn;
    let mut s = 0;
    for u in 0..n {
        let x: i32 = f[u] as i32;
        s += x.wrapping_mul(x) as u32;
    }
    s
}

#[inline(always)]
pub fn divrev31(x: u32) -> (u32, u32) {
    let qq = (x.wrapping_mul(67651u32)) >> 21;
    (qq, x - 31 * qq)
}