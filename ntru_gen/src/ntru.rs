#![allow(non_snake_case)]

use ntru_gen_c::poly::{ntrugen_poly_big_to_fixed, ntrugen_poly_sub_scaled};

use crate::fxp::{fxr, fxr_div, fxr_neg, fxr_round, vect_add, vect_div_autoadj_fft, vect_fft, vect_ifft, vect_mul2e, vect_mul_fft, vect_norm_fft};
use crate::mp31::{mp_add, mp_div, mp_intt, mp_mkgm, mp_mkgmigm, mp_mkigm, mp_montymul, mp_norm, mp_ntt, mp_rx31, mp_set, mp_sub, PRIMES, tbmask};
use crate::poly::{divrev31, poly_big_to_fixed, poly_big_to_small, poly_max_bitlength, poly_mp_norm, poly_mp_set, poly_mp_set_small, poly_sub_kfg_scaled_depth1, poly_sub_scaled, poly_sub_scaled_ntt};
use crate::zint31::{zint_bezout, zint_mod_small_signed, zint_mul_small, zint_rebuild_crt};

const MIN_LOGN_FGNTT: usize = 4;

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

    tmp.copy_within(0..(2 * n * slen), hn * tlen * 2);
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
    let (fs, inter) = inter2.split_at_mut(n * slen);
    let (gs, inter) = inter.split_at_mut(n * slen);
    zint_rebuild_crt(fs, slen, n, 1, true, inter);
    zint_rebuild_crt(gs, slen, n, 1, true, inter);
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
        b |= tmp[u].wrapping_sub(1);
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
    // tmp.copy_within(0..2 * len, 3 * len);
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

pub fn solve_ntru_deepest(profile: &NtruProfile, logn_top: usize, f: &[i8], g: &[i8], tmp: &mut [u32]) -> bool {
    if !make_fg_deepest(profile, logn_top, f, g, tmp, 6 << logn_top) {
        return false;
    }

    let len = profile.max_bl_small[logn_top] as usize;
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

pub fn solve_ntru_intermediate(profile: &NtruProfile, logn_top: usize, f: &[i8], g: &[i8], depth: usize, tmp: &mut [u32]) -> bool {
    let logn = logn_top - depth;
    let n = 1 << logn;
    let hn = n >> 1;

    let slen: usize = profile.max_bl_small[depth] as usize;
    let llen: usize = profile.max_bl_large[depth] as usize;
    let dlen: usize = profile.max_bl_small[depth + 1] as usize;


    if depth < profile.min_save_fg[logn_top] as usize {
        make_fg_intermediate(profile, logn_top, f, g, depth as u32, tmp.split_at_mut(dlen * hn * 2).1);
    } else {
        let mut sav_fg_index = (6 << logn_top) as usize;
        for d in profile.min_save_fg[logn_top]..=(depth as u16) {
            sav_fg_index -= (profile.max_bl_small[d as usize] as usize) << (logn_top + 1 - d as usize);
        }
        tmp.copy_within(sav_fg_index..sav_fg_index + 2 * slen * n, 2 * dlen * hn);
    }
    tmp.copy_within(2 * dlen * hn..2 * dlen * hn + 2 * n * slen, 2 * llen * n);
    tmp.copy_within(0..2 * dlen * hn, 2 * llen * n + 2 * slen * n);
    let (mut Ft, inter) = tmp.split_at_mut(llen * n);
    let (mut Gt, mut inter) = inter.split_at_mut(llen * n);
    let (mut ft, intergt) = inter.split_at_mut(slen * n);
    let (mut gt, mut t1) = intergt.split_at_mut(slen * n);
    let (Fd, Gd) = t1.split_at_mut(dlen * hn);

    for u in 0..llen {
        let p = PRIMES[u].p;
        let p0i = PRIMES[u].p0i;
        let r2 = PRIMES[u].r2;
        let rx = mp_rx31(dlen as u32, p, p0i, r2);
        let xt = Ft.split_at_mut(u * n + hn).1;
        let yt = Gt.split_at_mut(u * n + hn).1;
        for v in 0..hn {
            xt[v] = zint_mod_small_signed(Fd.split_at_mut(v).1, dlen, hn,
                                          p, p0i, r2, rx);
            yt[v] = zint_mod_small_signed(Gd.split_at_mut(v).1, dlen, hn,
                                          p, p0i, r2, rx);
        }
    }

    for u in 0..llen {
        if u == slen {
            zint_rebuild_crt(ft, slen, n, 1, true, t1);
            zint_rebuild_crt(gt, slen, n, 1, true, t1);
        }

        let p = PRIMES[u].p;
        let p0i = PRIMES[u].p0i;
        let r2 = PRIMES[u].r2;

        let (gm, inter) = t1.split_at_mut(n);
        let (igm, inter) = inter.split_at_mut(n);
        let (fx, gx) = inter.split_at_mut(n);
        mp_mkgmigm(logn, gm, igm, PRIMES[u].g, PRIMES[u].ig, p, p0i);
        if u < slen {
            fx[..n].copy_from_slice(&ft[u * n..u * n + n]);
            gx[..n].copy_from_slice(&gt[u * n..u * n + n]);
            mp_intt(logn, ft.split_at_mut(u * n).1, igm, p, p0i);
            mp_intt(logn, gt.split_at_mut(u * n).1, igm, p, p0i);
        } else {
            let rx = mp_rx31(slen as u32, p, p0i, r2);
            for v in 0..n {
                fx[v] = zint_mod_small_signed(ft.split_at_mut(v).1, slen, n,
                                              p, p0i, r2, rx);
                gx[v] = zint_mod_small_signed(gt.split_at_mut(v).1, slen, n,
                                              p, p0i, r2, rx);
            }
            mp_ntt(logn, fx, gm, p, p0i);
            mp_ntt(logn, gx, gm, p, p0i);
        }

        let Fe = Ft.split_at_mut(u * n).1;
        let Ge = Gt.split_at_mut(u * n).1;
        mp_ntt(logn - 1, Fe.split_at_mut(hn).1, gm, p, p0i);
        mp_ntt(logn - 1, Ge.split_at_mut(hn).1, gm, p, p0i);
        for v in 0..hn {
            let fa = fx[(v << 1) + 0];
            let fb = fx[(v << 1) + 1];
            let ga = gx[(v << 1) + 0];
            let gb = gx[(v << 1) + 1];
            let mFp = mp_montymul(Fe[v + hn], r2, p, p0i);
            let mGp = mp_montymul(Ge[v + hn], r2, p, p0i);
            Fe[(v << 1) + 0] = mp_montymul(gb, mFp, p, p0i);
            Fe[(v << 1) + 1] = mp_montymul(ga, mFp, p, p0i);
            Ge[(v << 1) + 0] = mp_montymul(fb, mGp, p, p0i);
            Ge[(v << 1) + 1] = mp_montymul(fa, mGp, p, p0i);
        }
        mp_intt(logn, Fe, igm, p, p0i);
        mp_intt(logn, Ge, igm, p, p0i);
    }

    if slen == llen {
        zint_rebuild_crt(ft, slen, n, 1, true, t1);
        zint_rebuild_crt(gt, slen, n, 1, true, t1);
    }

    zint_rebuild_crt(Ft, llen, n, 1, true, t1);
    zint_rebuild_crt(Gt, llen, n, 1, true, t1);
    let use_sub_ntt = depth > 1 && logn >= MIN_LOGN_FGNTT;
    if use_sub_ntt {
        tmp.copy_within(slen * n + 2 * llen * n..slen * n + 2 * llen * n + n * slen, slen * n + 2 * llen * n + n);
        (Ft, inter) = tmp.split_at_mut(llen * n);
        (Gt, inter) = inter.split_at_mut(llen * n);
        (ft, inter) = inter.split_at_mut(slen * n + n);
        (gt, t1) = inter.split_at_mut(slen * n + n);
    }

    let rt3 = bytemuck::cast_slice_mut::<u32, fxr>(t1);
    let (mut rt3, mut inter_fxr) = rt3.split_at_mut(n);
    let (mut rt4, mut rt1) = inter_fxr.split_at_mut(n);
    let mut rlen = profile.word_win[depth] as usize;
    if rlen > slen {
        rlen = slen;
    }
    let blen = slen - rlen;

    let ftb = ft.split_at_mut(blen * n).1;
    let gtb = gt.split_at_mut(blen * n).1;
    let scale_fg = 31 * (blen as u32);
    let mut scale_FG = 31 * (llen as u32);

    let scale_xf: u32 = poly_max_bitlength(logn, ftb, rlen);
    let scale_xg: u32 = poly_max_bitlength(logn, gtb, rlen);
    let mut scale_x = scale_xf;
    scale_x ^= (scale_xf ^ scale_xg) & tbmask(scale_xf.wrapping_sub(scale_xg));
    let mut scale_t: u32 = (15 - logn) as u32;
    scale_t ^= (scale_t ^ scale_x) & tbmask(scale_x.wrapping_sub(scale_t));
    let scdiff = scale_x.wrapping_sub(scale_t);

    poly_big_to_fixed(logn, rt3, ftb, rlen, scdiff);
    poly_big_to_fixed(logn, rt4, gtb, rlen, scdiff);


    vect_fft(logn, rt3);
    vect_fft(logn, rt4);
    vect_norm_fft(logn, rt1, rt3, rt4);
    vect_mul2e(logn, rt3, scale_t as u32);
    vect_mul2e(logn, rt4, scale_t as u32);
    for u in 0..hn {
        rt3[u] = fxr_div(rt3[u], rt1[u]);
        rt3[u + hn] = fxr_div(fxr_neg(rt3[u + hn]), rt1[u]);
        rt4[u] = fxr_div(rt4[u], rt1[u]);
        rt4[u + hn] = fxr_div(fxr_neg(rt4[u + hn]), rt1[u]);
    }

    //    let (mut Ft, inter) = tmp.split_at_mut(llen * n);
    //     let (mut Gt, mut inter) = inter.split_at_mut(llen * n);
    //     let (mut ft, intergt) = inter.split_at_mut(slen * n);
    //     let (mut gt, mut t1) = intergt.split_at_mut(slen * n);
    //     let (Fd, Gd) = t1.split_at_mut(dlen * hn);

    if depth == 1 {
        tmp.copy_within(2 * llen * n + 2 * slen * n..2 * llen * n + 2 * slen * n + 4 * n, 2 * llen * n);
    }


    if use_sub_ntt {
        (Ft, inter) = tmp.split_at_mut(llen * n);
        (Gt, inter) = inter.split_at_mut(llen * n);
        (ft, inter) = inter.split_at_mut(slen * n + n);
        (gt, inter) = inter.split_at_mut(slen * n + n);
        let (_, t2) = inter.split_at_mut(5 * n);

        let (mut gm, mut tn) = t2.split_at_mut(n);
        for u in 0..=slen {
            let p = PRIMES[u].p;
            let p0i = PRIMES[u].p0i;
            let r2 = PRIMES[u].r2;
            let rx = mp_rx31(slen as u32, p, p0i, r2);
            mp_mkgm(logn, gm, PRIMES[u].g, p, p0i);
            for v in 0..n {
                tn[v] = zint_mod_small_signed(
                    ft.split_at_mut(v).1, slen, n, p, p0i, r2, rx);
            }
            mp_ntt(logn, tn, gm, p, p0i);
            tn = tn.split_at_mut(n).1;
        }
        let (mut gm, mut tn) = t2.split_at_mut(n);
        ft[..(slen + 1) * n].copy_from_slice(&tn[..(slen + 1) * n]);
        for u in 0..=slen {
            let p = PRIMES[u].p;
            let p0i = PRIMES[u].p0i;
            let r2 = PRIMES[u].r2;
            let rx = mp_rx31(slen as u32, p, p0i, r2);
            mp_mkgm(logn, gm, PRIMES[u].g, p, p0i);
            for v in 0..n {
                tn[v] = zint_mod_small_signed(
                    gt.split_at_mut(v).1, slen, n, p, p0i, r2, rx);
            }
            mp_ntt(logn, tn, gm, p, p0i);
            tn = tn.split_at_mut(n).1;
        }
        let (_, mut tn) = t2.split_at_mut(n);
        gt[..(slen + 1) * n].copy_from_slice(&tn[..(slen + 1) * n]);
    }

    let mut FGlen = llen;

    if depth == 1 {
        loop {
            (Ft, inter) = tmp.split_at_mut(llen * n);
            (Gt, inter) = inter.split_at_mut(llen * n);
            let nrt3 = bytemuck::cast_slice_mut::<u32, fxr>(inter);
            (rt3, inter_fxr) = nrt3.split_at_mut(n);
            (rt4, inter_fxr) = inter_fxr.split_at_mut(n);
            let (rt1, rt2) = inter_fxr.split_at_mut(n);
            let (tlen, toff) = divrev31(scale_FG as u32);
            unsafe {
                ntrugen_poly_big_to_fixed(logn as u32, rt1.as_ptr(),
                                          Ft.split_at_mut(((tlen as usize) * n) as usize).1.as_ptr(), FGlen - tlen as usize, scale_x + toff)
            }
            unsafe {
                ntrugen_poly_big_to_fixed(logn as u32, rt2.as_ptr(),
                                          Gt.split_at_mut(((tlen as usize) * n) as usize).1.as_ptr(), FGlen - tlen as usize, scale_x + toff)
            }
            // poly_big_to_fixed(logn, rt1,
            //                   Ft.split_at_mut(((tlen as usize) * n) as usize).1, FGlen - tlen as usize, scale_x + toff);
            // poly_big_to_fixed(logn, rt2,
            //                   Gt.split_at_mut(((tlen as usize) * n) as usize).1, FGlen - tlen as usize, scale_x + toff);
            vect_fft(logn, rt1);
            vect_fft(logn, rt2);
            vect_mul_fft(logn, rt1, rt3);
            vect_mul_fft(logn, rt2, rt4);
            vect_add(logn, rt2, rt1);
            vect_ifft(logn, rt2);


            let mut k = bytemuck::cast_slice_mut::<fxr, i32>(rt1);
            let mut t2: &mut [i32] = &mut [];
            (k, t2) = k.split_at_mut(n);
            for u in 0..n {
                k[u] = fxr_round(rt2[u]);
            }
            // Sadly we have to reborrow such that t2 is big enough
            (Ft, inter) = tmp.split_at_mut(llen * n);
            (Gt, inter) = inter.split_at_mut(llen * n);
            let nrt3 = bytemuck::cast_slice_mut::<u32, fxr>(inter);
            (rt3, inter_fxr) = nrt3.split_at_mut(n);
            (rt4, inter_fxr) = inter_fxr.split_at_mut(n);
            let mut k = bytemuck::cast_slice_mut::<fxr, i32>(inter_fxr);
            let mut t2: &mut [i32] = &mut [];
            (k, t2) = k.split_at_mut(n);
            let scale_k = scale_FG - scale_fg;
            poly_sub_kfg_scaled_depth1(logn_top, Ft, Gt, FGlen, bytemuck::cast_slice_mut(k), scale_k as u32, f, g, bytemuck::cast_slice_mut(t2));

            if scale_FG <= scale_fg {
                break;
            }
            if scale_FG <= (scale_fg + profile.reduce_bits as u32) {
                scale_FG = scale_fg;
            } else {
                scale_FG -= profile.reduce_bits as u32;
            }
            while FGlen > slen && 31 * (FGlen - slen) > (scale_FG - scale_fg + 30) as usize {
                FGlen -= 1;
            }

        }
    } else {
        loop {
            if use_sub_ntt {
                (Ft, inter) = tmp.split_at_mut(llen * n);
                (Gt, inter) = inter.split_at_mut(llen * n);
                (ft, inter) = inter.split_at_mut(slen * n + n);
                (gt, t1) = inter.split_at_mut(slen * n + n);
            } else {
                (Ft, inter) = tmp.split_at_mut(llen * n);
                (Gt, inter) = inter.split_at_mut(llen * n);
                (ft, inter) = inter.split_at_mut(slen * n);
                (gt, t1) = inter.split_at_mut(slen * n);
            }
            let rt3 = bytemuck::cast_slice_mut::<u32, fxr>(t1);
            let (mut rt3, mut inter_fxr) = rt3.split_at_mut(n);
            let (mut rt4, mut inter_fxr_rt1) = inter_fxr.split_at_mut(n);
            let (mut rt1, mut inter_fxr) = inter_fxr_rt1.split_at_mut(n);
            let (mut rt2, _) = inter_fxr.split_at_mut(n);
            let (tlen, toff) = divrev31(scale_FG as u32);
            // TODO FIX
            unsafe {
                ntrugen_poly_big_to_fixed(logn as u32, rt1.as_ptr(),
                                          Ft.split_at_mut(((tlen as usize) * n) as usize).1.as_ptr(), FGlen - tlen as usize, scale_x + toff)
            }
            unsafe {
                ntrugen_poly_big_to_fixed(logn as u32, rt2.as_ptr(),
                                          Gt.split_at_mut(((tlen as usize) * n) as usize).1.as_ptr(), FGlen - tlen as usize, scale_x + toff)
            }
            // poly_big_to_fixed(logn, rt1,
            //                   Ft.split_at_mut(((tlen as usize) * n) as usize).1, FGlen - tlen as usize, scale_x + toff);
            // poly_big_to_fixed(logn, rt2,
            //                   Gt.split_at_mut(((tlen as usize) * n) as usize).1, FGlen - tlen as usize, scale_x + toff);
            vect_fft(logn, rt1);
            vect_fft(logn, rt2);
            vect_mul_fft(logn, rt1, rt3);
            vect_mul_fft(logn, rt2, rt4);
            vect_add(logn, rt2, rt1);
            vect_ifft(logn, rt2);
            let k = bytemuck::cast_slice_mut::<fxr, i32>(rt1);


            for u in 0..n {
                k[u] = fxr_round(rt2[u]);
            }

            let scale_k = scale_FG - scale_fg;
            if use_sub_ntt {
                let k = bytemuck::cast_slice_mut::<fxr, i32>(inter_fxr_rt1);
                let (k, t2) = k.split_at_mut(n);
                poly_sub_scaled_ntt(logn, Ft, FGlen, ft, slen,
                                    k, scale_k as u32, bytemuck::cast_slice_mut(t2));
                poly_sub_scaled_ntt(logn, Gt, FGlen, gt, slen,
                                    k, scale_k as u32, bytemuck::cast_slice_mut(t2));
            } else {
                poly_sub_scaled(logn, Ft, FGlen, ft, slen, k, scale_k as u32);
                poly_sub_scaled(logn, Gt, FGlen, gt, slen, k, scale_k as u32);
            }

            if scale_FG <= scale_fg {
                break;
            }
            if scale_FG <= (scale_fg + profile.reduce_bits as u32) {
                scale_FG = scale_fg;
            } else {
                scale_FG -= profile.reduce_bits as u32;
            }
            while FGlen > slen && 31 * (FGlen - slen) > (scale_FG - scale_fg + 30) as usize {
                FGlen -= 1;
            }
        }
    }


    tmp.copy_within(llen * n..llen * n + slen * n, slen * n);
    if depth == 1 {
        return true;
    }


    let (Ft, inter) = tmp.split_at_mut(slen * n);
    let (Gt, inter) = inter.split_at_mut(llen * n);
    let p = PRIMES[0].p;
    let p0i = PRIMES[0].p0i;
    let r2 = PRIMES[0].r2;
    let rx = mp_rx31(slen as u32, p, p0i, r2);
    if use_sub_ntt {
        let (_, inter) = inter.split_at_mut((llen - slen) * n);
        let (ft, inter) = inter.split_at_mut(slen * n + n);
        let (gt, inter) = inter.split_at_mut(slen * n + n);
        let (_, inter) = inter.split_at_mut(n);
        let (t2, inter) = inter.split_at_mut(n);
        let (t3, t4) = inter.split_at_mut(n);
        mp_mkgm(logn, t4, PRIMES[0].g, p, p0i);
        for u in 0..n {
            t2[u] = zint_mod_small_signed(
                Gt.split_at_mut(u).1, slen, n, p, p0i, r2, rx);
        }
        mp_ntt(logn, t2, t4, p, p0i);
        let t1 = ft;
        for u in 0..n {
            t3[u] = mp_montymul(t1[u], t2[u], p, p0i);
        }

        let t1 = gt;
        for u in 0..n {
            t2[u] = zint_mod_small_signed(
                Ft.split_at_mut(u).1, slen, n, p, p0i, r2, rx);
        }
        mp_ntt(logn, t2, t4, p, p0i);

        let rv = mp_montymul(profile.q, 1, p, p0i);
        for u in 0..n {
            let x = mp_montymul(t1[u], t2[u], p, p0i);
            if mp_sub(t3[u], x, p) != rv {
                return false;
            }
        }
    } else {
        let (_, inter) = inter.split_at_mut((llen - slen) * n);
        let (ft, inter) = inter.split_at_mut(slen * n);
        let (gt, inter) = inter.split_at_mut(slen *n );
        let (t1, inter) = inter.split_at_mut(n);
        let (t2, inter) = inter.split_at_mut(n);
        let (t3, t4) = inter.split_at_mut(n);
        mp_mkgm(logn, t4, PRIMES[0].g, p, p0i);
        for u in 0..n {
            t1[u] = zint_mod_small_signed(
                ft.split_at_mut(u).1, slen, n, p, p0i, r2, rx);
            t2[u] = zint_mod_small_signed(
                Gt.split_at_mut(u).1, slen, n, p, p0i, r2, rx);
        }
        mp_ntt(logn, t1, t4, p, p0i);
        mp_ntt(logn, t2, t4, p, p0i);
        for u in 0..n {
            t3[u] = mp_montymul(t1[u], t2[u], p, p0i);
        }

        for u in 0..n {
            t1[u] = zint_mod_small_signed(
                gt.split_at_mut(u).1, slen, n, p, p0i, r2, rx);
            t2[u] = zint_mod_small_signed(
                Ft.split_at_mut(u).1, slen, n, p, p0i, r2, rx);
        }
        mp_ntt(logn, t1, t4, p, p0i);
        mp_ntt(logn, t2, t4, p, p0i);

        let rv = mp_montymul(profile.q, 1, p, p0i);
        for u in 0..n {
            let x = mp_montymul(t1[u], t2[u], p, p0i);
            if mp_sub(t3[u], x, p) != rv {
                return false;
            }
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
    let n = 1 << logn;

    if !solve_ntru_deepest(profile, logn, f, g, tmp) {
        return false;
    }
    let mut depth = logn;
    while depth > 0 {
        depth -= 1;
        if !solve_ntru_intermediate(profile, logn, f, g, depth, tmp) {
            return false;
        }
    }
    if !solve_ntru_depth0(profile, logn, f, g, tmp) {
        return false;
    }
    let (tmpp, inter) = tmp.split_at_mut(2 * n);
    let inter = bytemuck::cast_slice_mut::<u32, i8>(inter);
    let (F, G) = inter.split_at_mut(n);
    let lim = profile.coeff_FG_limit[logn];
    if !poly_big_to_small(logn, F, tmpp, lim as i32) {
        return false;
    }
    if !poly_big_to_small(logn, G, tmpp.split_at_mut(n).1, lim as i32) {
        return false;
    }
    tmp.copy_within(2 * n..4 * n, 0);

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