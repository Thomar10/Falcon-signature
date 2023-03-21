#![allow(non_snake_case)]

use crate::fxp::{fxr, fxr_round, vect_div_autoadj_fft, vect_fft, vect_ifft};
use crate::mp31::{mp_add, mp_div, mp_intt, mp_mkgm, mp_mkigm, mp_montymul, mp_norm, mp_ntt, mp_rx31, mp_set, mp_sub, PRIMES, tbmask};
use crate::poly::{poly_big_to_small, poly_mp_norm, poly_mp_set, poly_mp_set_small};
use crate::zint31::{zint_bezout, zint_mod_small_signed, zint_mul_small, zint_rebuild_crt};

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
    let (gs, inter) = inter2.split_at_mut(n * slen);
    let (fs, inter) = inter.split_at_mut(n * slen);
    let (t1, t2) = inter.split_at_mut(n);
    zint_rebuild_crt(fs, slen, n, 1, true, t1);
    zint_rebuild_crt(gs, slen, n, 1, true, t1);

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

pub fn make_fg_intermediate(profile: &NtruProfile, logn_top: usize, f: &[i8], g: &[i8], depth: u32, tmp: &mut [u32]) {
    make_fg_zero(logn_top, f, g, tmp);
    for d in 0..depth {
        make_fg_step(profile, logn_top, d, tmp);
    }
}

pub fn make_fg_deepest(profile: &NtruProfile, logn_top: usize, f: &[i8], g: &[i8], tmp: &mut [u32], mut sav_off: usize) -> bool {
    make_fg_zero(logn_top, f, g, tmp);
    let mut r = 1;

    let n = 1 << logn_top;
    let mut b = 0;
    for u in 0..n {
        b |= tmp[u] - 1;
    }
    r = 1 - (b >> 31);
    for d in 0..logn_top {
        make_fg_step(profile, logn_top, d as u32, tmp);

        let d2 = d + 1;
        if d2 < logn_top && d2 >= profile.min_save_fg[logn_top] as usize {
            let slen: usize = profile.max_bl_small[d2] as usize;
            let fglen: usize = (slen << (logn_top + 1 - d2)) as usize;
            sav_off -= fglen;
            tmp.copy_within(0..fglen, sav_off);
        }
    }

    r != 0
}

pub fn solve_NTRU_deepest(profile: &NtruProfile, logn_top: usize, f: &[i8], g: &[i8], tmp: &mut [u32]) -> bool {
    if !make_fg_deepest(profile, logn_top, f, g, tmp, 6 << logn_top) {
        return false;
    }

    let len: usize = profile.max_bl_small[logn_top] as usize;
    tmp.copy_within(0..2 * len, 2 * len);
    let (Fp, inter) = tmp.split_at_mut(len);
    let (Gp, inter) = inter.split_at_mut(len);
    let (fp, inter) = inter.split_at_mut(len);
    let (gp, t1) = inter.split_at_mut(len);

    zint_rebuild_crt(fp, len, 1, 1, false, t1);
    zint_rebuild_crt(gp, len, 1, 1, false, t1);

    if !zint_bezout(Gp, Fp, fp, gp, len, t1) {
        return false;
    }
    if profile.q != 1 {
        if zint_mul_small(Fp, len, profile.q) != 0
            || zint_mul_small(Gp, len, profile.q) != 0
        {
            return false;
        }
    }

    true
}

pub fn solve_ntru_depth0(profile: &NtruProfile, logn: usize, f: &[i8], g: &[i8], tmp: &mut [u32]) -> bool {
    let n = 1 << logn;
    let hn = n >> 1;

    let p = PRIMES[0].p;
    let p0i = PRIMES[0].p0i;
    let r2 = PRIMES[0].r2;

    let (Fd, inter) = tmp.split_at_mut(hn);
    let (Gd, inter) = inter.split_at_mut(hn);
    let (ft, inter) = inter.split_at_mut(n);
    let (gt, gm) = inter.split_at_mut(n);

    mp_mkgm(logn, gm, PRIMES[0].g, p, p0i);
    poly_mp_set_small(logn, ft, f, p);
    poly_mp_set_small(logn, gt, g, p);
    mp_ntt(logn, ft, gm, p, p0i);
    mp_ntt(logn, gt, gm, p, p0i);

    poly_mp_set(logn - 1, Fd, p);
    poly_mp_set(logn - 1, Gd, p);
    mp_ntt(logn - 1, Fd, gm, p, p0i);
    mp_ntt(logn - 1, Gd, gm, p, p0i);

    for u in 0..hn {
        let fa = ft[(u << 1) + 0];
        let fb = ft[(u << 1) + 1];
        let ga = gt[(u << 1) + 0];
        let gb = gt[(u << 1) + 1];
        let mFd = mp_montymul(Fd[u], r2, p, p0i);
        let mGd = mp_montymul(Gd[u], r2, p, p0i);
        ft[(u << 1) + 0] = mp_montymul(gb, mFd, p, p0i);
        ft[(u << 1) + 1] = mp_montymul(ga, mFd, p, p0i);
        gt[(u << 1) + 0] = mp_montymul(fb, mGd, p, p0i);
        gt[(u << 1) + 1] = mp_montymul(fa, mGd, p, p0i);
    }

    tmp.copy_within(2 * hn..2 * hn + 2 * n, 0);
    let (Fp, inter) = tmp.split_at_mut(n);
    let (Gp, inter) = inter.split_at_mut(n);
    let (t1, inter) = inter.split_at_mut(n);
    let (t2, inter) = inter.split_at_mut(n);
    let (t3, t4) = inter.split_at_mut(n);

    poly_mp_set_small(logn, t4, f, p);
    mp_ntt(logn, t4, t2, p, p0i);
    for u in 0..n {
        let w = mp_montymul(t4[(n - 1) - u], r2, p, p0i);
        t1[u] = mp_montymul(w, Fp[u], p, p0i);
        t3[u] = mp_montymul(w, t4[u], p, p0i);
    }

    poly_mp_set_small(logn, t4, g, p);
    mp_ntt(logn, t4, t2, p, p0i);
    for u in 0..n {
        let w = mp_montymul(t4[(n - 1) - u], r2, p, p0i);
        t1[u] = mp_add(t1[u], mp_montymul(w, Gp[u], p, p0i), p);
        t3[u] = mp_add(t3[u], mp_montymul(w, t4[u], p, p0i), p);
    }

    mp_mkigm(logn, t4, PRIMES[0].ig, p, p0i);
    mp_intt(logn, t1, t4, p, p0i);
    mp_intt(logn, t3, t4, p, p0i);
    for u in 0..n {
        t1[u] = mp_norm(t1[u], p) as u32;
        t2[u] = mp_norm(t3[u], p) as u32;
    }

    let (_, inter) = tmp.split_at_mut(2 * n);
    let (t1, inter2) = inter.split_at_mut(n);
    let (t2, t3) = inter2.split_at_mut(n);
    let rt3 = bytemuck::pod_align_to_mut::<u32, fxr>(t3).1;
    for u in 0..n {
        rt3[u] = (t2[u] << 22) as u64;
    }
    vect_fft(logn, rt3);
    let inter = bytemuck::pod_align_to_mut::<u32, fxr>(inter2).1;
    let (rt2, rt3) = inter.split_at_mut(hn);
    rt2.copy_from_slice(&rt3[0..hn]);
    for u in 0..n {
        rt3[u] = (t1[u] << 22) as u64
    }
    vect_fft(logn, rt3);
    vect_div_autoadj_fft(logn, rt3, rt2);
    vect_ifft(logn, rt3);
    for u in 0..n {
        t1[u] = mp_set(fxr_round(rt3[u]), p);
    }

    let (Fp, inter) = tmp.split_at_mut(n);
    let (Gp, inter) = inter.split_at_mut(n);
    let (t1, inter) = inter.split_at_mut(n);
    let (t2, inter) = inter.split_at_mut(n);
    let (t3, t4) = inter.split_at_mut(n);

    mp_mkgm(logn, t4, PRIMES[0].g, p, p0i);
    mp_ntt(logn, t1, t4, p, p0i);
    for u in 0..n {
        t1[u] = mp_montymul(t1[u], r2, p, p0i);
    }

    for u in 0..n {
        t2[u] = mp_set(f[u] as i32, p);
        t3[u] = mp_set(g[u] as i32, p);
    }
    mp_ntt(logn, t2, t4, p, p0i);
    mp_ntt(logn, t3, t4, p, p0i);
    let rv = mp_montymul(profile.q, 1, p, p0i);
    for u in 0..n {
        Fp[u] = mp_sub(Fp[u], mp_montymul(t1[u], t2[u], p, p0i), p);
        Gp[u] = mp_sub(Gp[u], mp_montymul(t1[u], t3[u], p, p0i), p);
        let x = mp_sub(
            mp_montymul(t2[u], Gp[u], p, p0i),
            mp_montymul(t3[u], Fp[u], p, p0i), p);
        if x != rv {
            return false;
        }
    }
    mp_mkigm(logn, t4, PRIMES[0].ig, p, p0i);
    mp_intt(logn, Fp, t4, p, p0i);
    mp_intt(logn, Gp, t4, p, p0i);
    poly_mp_norm(logn, Fp, p);
    poly_mp_norm(logn, Gp, p);

    true
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