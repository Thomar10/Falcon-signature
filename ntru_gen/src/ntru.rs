#![allow(non_snake_case)]

use crate::mp31::{mp_add, mp_div, mp_intt, mp_mkgm, mp_mkigm, mp_montymul, mp_norm, mp_ntt, mp_rx31, mp_set, PRIMES, tbmask};
use crate::poly::{poly_big_to_small, poly_mp_set_small};
use crate::zint31::{zint_mod_small_signed, zint_rebuild_crt};

pub struct NtruProfile {
    pub(crate) q: u32,
    pub(crate) min_logn: u32,
    pub(crate) max_logn: u32,
    pub(crate) max_bl_small: [u16; 11],
    pub(crate) max_bl_large: [u16; 10],
    pub(crate) word_win: [u16; 10],
    pub(crate) reduce_bits: u32,
    pub(crate) coeff_FG_limit: [u8; 11],
    pub(crate) min_save_fg: [u16; 11],
}

fn make_fg_zero(logn: usize, f: &[i8], g: &[i8], tmp: &mut [u32]) {
    let n = 1 << logn;
    let (ft, inter) = tmp.split_at_mut(n);
    let (gt, gm) = inter.split_at_mut(n);
    let p = PRIMES[0].p;
    let p0i = PRIMES[0].p0i;
    poly_mp_set_small(logn, ft, f, p);
    poly_mp_set_small(logn, gt, g, p);
    mp_mkgm(logn, gm, PRIMES[0].g, p, p0i);
    mp_ntt(logn, ft, gm, p, p0i);
    mp_ntt(logn, gt, gm, p, p0i);
}

pub fn make_fg_step(profile: &NtruProfile, logn_top: usize, depth: u32, tmp: &mut [u32]) {
    let logn = logn_top - depth as usize;
    let n = 1 << logn;
    let hn = n >> 1;
    let slen: usize = profile.max_bl_small[depth as usize] as usize;
    let tlen: usize = profile.max_bl_small[(depth + 1) as usize] as usize;

    tmp.copy_within(hn * tlen * 2..(hn * tlen * 2 + 2 * n * slen), 0);
    let (fd, inter) = tmp.split_at_mut(hn * tlen);
    let (gd, inter) = inter.split_at_mut(hn * tlen);
    let (fs, inter) = inter.split_at_mut(n * slen);
    let (gs, t1) = inter.split_at_mut(n * slen);

    let mut xf = fs;
    let mut xg = gs;
    let mut yf = fd;
    let mut yg = gd;
    for u in 0..slen {
        let p = PRIMES[u].p;
        let p0i = PRIMES[u].p0i;
        let r2 = PRIMES[u].r2;
        for v in 0..hn {
            yf[v] = mp_montymul(
                mp_montymul(xf[2 * v], xf[2 * v + 1], p, p0i),
                r2, p, p0i);
            yg[v] = mp_montymul(
                mp_montymul(xg[2 * v], xg[2 * v + 1], p, p0i),
                r2, p, p0i);
        }
        mp_mkigm(logn, t1, PRIMES[u].ig, p, p0i);
        mp_intt(logn, xf, t1, p, p0i);
        mp_intt(logn, xg, t1, p, p0i);
        xf = xf.split_at_mut(n).1;
        xg = xg.split_at_mut(n).1;
        yf = yf.split_at_mut(hn).1;
        yg = yg.split_at_mut(hn).1;
    }

    let (fd, inter) = tmp.split_at_mut(hn * tlen);
    let (_, mut yf) = fd.split_at_mut(hn * slen);
    let (gd, inter2) = inter.split_at_mut(hn * tlen);
    let (_, mut yg) = gd.split_at_mut(hn * slen);
    let (inter_low, inter_top) = inter2.split_at_mut(2 * n * slen);

    zint_rebuild_crt(inter_low, slen, n, 2, true, inter_top);
    let (gs, inter) = inter2.split_at_mut(n * slen);
    let (fs, inter) = inter.split_at_mut(n * slen);
    let (t1, t2) = inter.split_at_mut(n);

    for u in slen..tlen {
        let p = PRIMES[u].p;
        let p0i = PRIMES[u].p0i;
        let r2 = PRIMES[u].r2;
        let rx = mp_rx31(slen as u32, p, p0i, r2);
        mp_mkgm(logn, t1, PRIMES[u].g, p, p0i);
        for v in 0..n {
            t2[v] = zint_mod_small_signed(
                fs.split_at_mut(v).1, slen, n, p, p0i, r2, rx);
        }
        mp_ntt(logn, t2, t1, p, p0i);
        for v in 0..hn {
            yf[v] = mp_montymul(
                mp_montymul(t2[2 * v], t2[2 * v + 1], p, p0i),
                r2, p, p0i);
        }
        yf = yf.split_at_mut(hn).1;
        for v in 0..n {
            t2[v] = zint_mod_small_signed(
                gs.split_at_mut(v).1, slen, n, p, p0i, r2, rx);
        }
        mp_ntt(logn, t2, t1, p, p0i);
        for v in 0..hn {
            yg[v] = mp_montymul(
                mp_montymul(t2[2 * v], t2[2 * v + 1], p, p0i),
                r2, p, p0i);
        }
        yg = yg.split_at_mut(hn).1;
    }
}

pub fn solve_ntru(profile: &NtruProfile, logn: usize, f: &[i8], g: &[i8], tmp: &mut [u32]) -> bool {
    // let n = 1 << logn;
    //
    // if !solve_ntru_deepest(profile, logn, f, g, tmp) {
    //     return false;
    // }
    // let mut depth = logn;
    // while depth > 0 {
    //     depth -= 1;
    //     if !solve_ntru_intermediate(profile, logn, f, g, depth, tmp) {
    //         return false;
    //     }
    // }
    // if !solve_ntru_depth0(profile, logn, f, g, tmp) {
    //     return false;
    // }
    // let (tmpp, inter) = tmp.split_at_mut(2 * n);
    // let inter = bytemuck::cast_slice_mut::<u32, i8>(inter);
    // let (F, G) = inter.split_at_mut(n);
    // let lim = profile.coeff_FG_limit[logn];
    // if !poly_big_to_small(logn, F, tmpp, lim as i32) {
    //     return false;
    // }
    // if !poly_big_to_small(logn, G, tmpp.split_at_mut(n).1, lim as i32) {
    //     return false;
    // }
    // tmp.copy_within(2 * n..4 * n, 0);
    //
    // true
    true
}

pub fn recover_G(logn: usize, q: i32, ulim: u32, f: &[i8], g: &[i8], F: &[i8], tmp: &mut [u32]) -> bool {
    let n = 1 << logn;
    let (gm, inter) = tmp.split_at_mut(n);
    let (t1, t2) = inter.split_at_mut(n);

    let p = PRIMES[0].p;
    let p0i = PRIMES[0].p0i;
    let r2 = PRIMES[0].r2;
    mp_mkgm(logn, gm, PRIMES[0].g, p, p0i);

    for u in 0..n {
        t1[u] = mp_set(g[u] as i32, p);
        t2[u] = mp_set(F[u] as i32, p);
    }
    mp_ntt(logn, t1, gm, p, p0i);
    mp_ntt(logn, t2, gm, p, p0i);
    let mq = mp_set(q, p);
    for u in 0..n {
        let x = mp_montymul(t1[u], t2[u], p, p0i);
        t2[u] = mp_add(mq, mp_montymul(x, r2, p, p0i), p);
    }
    for u in 0..n {
        t1[u] = mp_set(f[u] as i32, p);
    }
    mp_ntt(logn, t1, gm, p, p0i);
    let mut b = 0;
    for u in 0..n {
        b |= t1[u] - 1;
        t2[u] = mp_div(t2[u], t1[u], p);
    }
    // gm = igm
    mp_mkigm(logn, gm, PRIMES[0].ig, p, p0i);
    mp_intt(logn, t2, gm, p, p0i);

    let (G, t2) = tmp.split_at_mut(2 * n);
    let G = bytemuck::pod_align_to_mut::<u32, i8>(G).1;
    for u in 0..n {
        let x = t2[u];
        let y = tbmask((ulim << 1) - mp_add(x, ulim, p));
        b |= y;
        let z = mp_norm(x & !y, p);
        G[u] = z as i8;
    }
    (1 - (b >> 31)) != 0
}