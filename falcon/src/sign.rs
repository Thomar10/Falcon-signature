use std::mem;
use crate::common::is_short_half;
use crate::fft::{fft, ifft, poly_add, poly_LDL_fft, poly_LDLmv_fft, poly_merge_fft, poly_mul_fft, poly_muladj_fft, poly_mulconst, poly_mulselfadj_fft, poly_neg, poly_split_fft, poly_sub};
use crate::fpr::{fpr_mul, fpr_sqrt, FPR_INV_SIGMA, fpr_of, fpr_floor, fpr_sub, fpr_half, fpr_sqr, FPR_INV_2SQRSIGMA0, fpr_trunc, FPR_INV_LOG2, FPR_LOG2, fpr_expm_p63, FPR_INVERSE_OF_Q, fpr_neg, fpr_rint, FPR_SIGMA_MIN, fpr_add, FPR_INVSQRT8, FPR_INVSQRT2};
use crate::rng::{Prng, prng_get_u64, prng_get_u8, prng_init, State};
use crate::shake::InnerShake256Context;

macro_rules! MKN {
    ($logn: expr) => {
        (1 << $logn) as usize
    }
}

#[allow(non_camel_case_types)]
type fpr = u64;

#[allow(non_snake_case)]
#[inline(always)]
pub fn ffLDL_treesize(logn: u32) -> u32 {
    return (logn + 1) << logn;
}

#[allow(non_snake_case)]
pub fn ffLDL_fft_inner(tree: &mut [fpr], g0: &mut [fpr], g1: &mut [fpr], logn: u32, tmp: &mut [fpr]) -> () {
    let n: usize;
    let hn: usize;

    n = MKN!(logn);
    if n == 1 {
        tree[0] = g0[0];
        return;
    }
    hn = n >> 1;

    poly_LDLmv_fft(tmp, tree, (*g0).as_ref(), g1, (*g0).as_ref(), logn);

    let (g10, g11) = g1.split_at_mut(hn);
    poly_split_fft(g10, g11, g0, logn);
    let (g00, g01) = g0.split_at_mut(hn);
    poly_split_fft(g00, g01, tmp, logn);

    ffLDL_fft_inner(&mut tree[n..], g10, g11, logn - 1, tmp);
    ffLDL_fft_inner(&mut tree[n + (ffLDL_treesize(logn - 1) as usize)..], g00, g01, logn - 1, tmp);
}

#[allow(non_snake_case)]
pub fn ffLDL_fft(tree: &mut [fpr], g00: &mut [fpr], g01: &mut [fpr],
                 g11: &mut [fpr], logn: u32, tmp: &mut [fpr]) -> () {
    let n: usize;
    let hn: usize;

    n = MKN!(logn);
    if n == 1 {
        tree[0] = g00[0];
        return;
    }
    hn = n >> 1;
    let (d00, futured11) = tmp.split_at_mut(n);
    let (d11, shiny_temp) = futured11.split_at_mut(n);

    d00.copy_from_slice(&g00[..n]);
    poly_LDLmv_fft(d11, tree, g00, g01, g11, logn);

    let (tmp0, tmp1) = shiny_temp.split_at_mut(hn);
    poly_split_fft(tmp0, tmp1, d00, logn);
    let (d000, d001) = d00.split_at_mut(hn);
    poly_split_fft(d000, d001, d11, logn);

    d11.copy_from_slice(&shiny_temp[..n]);

    let (d110, d111) = d11.split_at_mut(hn);
    ffLDL_fft_inner(&mut tree[n..], d110, d111, logn - 1, shiny_temp);

    ffLDL_fft_inner(&mut tree[n + (ffLDL_treesize(logn - 1) as usize)..], d000, d001, logn - 1, shiny_temp);
}

#[allow(non_snake_case)]
pub fn ffLDL_binary_normalize(tree: &mut [fpr], orig_logn: u32, logn: u32) {
    let n: usize = MKN!(logn);

    if n == 1 {
        tree[0] = fpr_mul(fpr_sqrt(tree[0]), FPR_INV_SIGMA[orig_logn as usize])
    } else {
        ffLDL_binary_normalize(&mut tree[n..], orig_logn, logn - 1);
        ffLDL_binary_normalize(&mut tree[n + (ffLDL_treesize(logn - 1) as usize)..], orig_logn, logn - 1);
    }
}

pub fn smallints_to_fpr(r: &mut [fpr], t: &[i8], logn: u32) {
    let n: usize = MKN!(logn);

    for u in 0..n {
        r[u] = fpr_of(t[u] as i64);
    }
}

//TODO check if skoffs are needed
#[inline(always)]
pub fn skoff_b00(_logn: u32) -> usize {
    return 0;
}

#[inline(always)]
pub fn skoff_b01(logn: u32) -> usize {
    return MKN!(logn);
}

#[inline(always)]
pub fn skoff_b10(logn: u32) -> usize {
    return 2 * MKN!(logn);
}

#[inline(always)]
pub fn skoff_b11(logn: u32) -> usize {
    return 3 * MKN!(logn);
}

#[inline(always)]
pub fn skoff_tree(logn: u32) -> usize {
    return 4 * MKN!(logn);
}

#[allow(non_snake_case)]
pub fn expand_privkey(expanded_key: &mut [fpr], f: &[i8], g: &[i8], F: &[i8],
                      G: &[i8], logn: u32, tmp: &mut [u64]) {
    let n: usize = MKN!(logn);

    let (b00, inter) = expanded_key.split_at_mut(n);
    let (b01, inter) = inter.split_at_mut(n);
    let (b10, inter) = inter.split_at_mut(n);
    let (b11, tree) = inter.split_at_mut(n);

    let rf = b01;
    let rg = b00;
    let rF = b11;
    let rG = b10;

    smallints_to_fpr(rf, f, logn);
    smallints_to_fpr(rg, g, logn);
    smallints_to_fpr(rF, F, logn);
    smallints_to_fpr(rG, G, logn);

    fft(rf, logn);
    fft(rg, logn);
    fft(rF, logn);
    fft(rG, logn);
    poly_neg(rf, logn);
    poly_neg(rF, logn);

    //TODO check if this is needed. We might be able to just initialize 4 empty arrays
    //TODO and then avoid taking tmp as input
    let (g00, inter) = tmp.split_at_mut(n);
    let (g01, inter) = inter.split_at_mut(n);
    let (g11, gxx) = inter.split_at_mut(n);

    g00.copy_from_slice(&rg[0..n]);
    poly_mulselfadj_fft(g00, logn);
    gxx[0..n].copy_from_slice(&rf[0..n]);
    poly_mulselfadj_fft(gxx, logn);
    poly_add(g00, gxx, logn);

    g01.copy_from_slice(&rg[0..n]);
    poly_muladj_fft(g01, rG, logn);
    gxx[0..n].copy_from_slice(&rf[0..n]);
    poly_muladj_fft(gxx, rF, logn);
    poly_add(g01, gxx, logn);

    g11.copy_from_slice(&rG[0..n]);
    poly_mulselfadj_fft(g11, logn);
    gxx[0..n].copy_from_slice(&rF[0..n]);
    poly_mulselfadj_fft(gxx, logn);
    poly_add(g11, gxx, logn);

    ffLDL_fft(tree, g00, g01, g11, logn, gxx);

    ffLDL_binary_normalize(tree, logn, logn);
}

pub struct SamplerContext {
    pub(crate) p: Prng,
    pub(crate) sigma_min: fpr
}

type SamplerZ = fn(&mut SamplerContext, fpr, fpr) -> i32;

//TODO test
#[allow(non_snake_case)]
pub fn ffSampling_fft_dyntree(samp: SamplerZ, samp_ctx: &mut SamplerContext, t0: &mut [fpr],
                              t1: &mut [fpr], g00: &mut [fpr], g01: &mut [fpr], g11: &mut [fpr],
                              orig_logn: u32, logn: u32, tmp: &mut [fpr]) {

    let n: usize = 1 << logn as usize;
    let hn: usize = n >> 1;

    if logn == 0 {
        let mut leaf: fpr = g00[0];
        leaf = fpr_mul(fpr_sqrt(leaf), FPR_INV_SIGMA[orig_logn as usize]);
        t0[0] = fpr_of(samp(samp_ctx, t0[0], leaf) as i64);
        t1[0] = fpr_of(samp(samp_ctx, t1[0], leaf) as i64);
        return;
    }

    poly_LDL_fft(g00, g01, g11, logn);

    let (tmp0, tmp1) = tmp.split_at_mut(hn);
    poly_split_fft(tmp0, tmp1, g00, logn);
    g00[..hn].copy_from_slice(tmp0);
    g00[hn..].copy_from_slice(&tmp1[..hn]);
    poly_split_fft(tmp0, tmp1, g11, logn);
    g11.copy_from_slice(&tmp[..n]);
    tmp[..n].copy_from_slice(g01);
    g01[..hn].copy_from_slice(&g00[..hn]);
    g01[hn..].copy_from_slice(&g11[..hn]);

    let (z10, z11) = tmp.split_at_mut(hn);
    poly_split_fft(z10, &mut z11[..hn], t1, logn);

    let (g110, g111) = g11.split_at_mut(hn);
    let (z1hn, z1n) = z11.split_at_mut(hn);
    ffSampling_fft_dyntree(samp, samp_ctx, z10, z1hn, g110, g111,
                           &mut g01[hn..], orig_logn, logn - 1, z1n);
    poly_merge_fft(z1n, z10, z1hn, logn);

    let (tmp0, inter) = tmp.split_at_mut(n);
    let (z1, tmpn1) = inter.split_at_mut(n);
    z1.copy_from_slice(t1);
    poly_sub(z1, tmpn1, logn);
    t1.copy_from_slice(&tmpn1[..n]);
    poly_mul_fft(tmp0, z1, logn);
    poly_add(t0, tmp0, logn);

    let (z0, z0n) = tmp.split_at_mut(n);
    let (z00, z01) = z0.split_at_mut(hn);

    poly_split_fft(z00, z01, t0, logn);

    let (g000, g001) = g00.split_at_mut(hn);
    ffSampling_fft_dyntree(samp, samp_ctx, z00, z01, g000, g001,
                           g01, orig_logn, logn - 1, z0n);
    poly_merge_fft(t0, z00, z01, logn);
}

//TODO test
#[allow(non_snake_case)]
pub fn ffSampling_fft(samp: SamplerZ, samp_ctx: &mut SamplerContext, z0: &mut [fpr],
                      z1: &mut [fpr], tree: &mut [fpr], t0: &mut [fpr], t1: &mut [fpr], logn: u32, tmp: &mut [fpr]) {

    if logn == 2 {
        let (tree0, treerest) = tree.split_at(4);
        let (tree1, treerest) = treerest.split_at(4);

        let mut a_re: fpr = t1[0];
        let mut a_im: fpr = t1[2];
        let mut b_re: fpr = t1[1];
        let mut b_im: fpr = t1[3];
        let mut c_re: fpr = fpr_add(a_re, b_re);
        let mut c_im: fpr = fpr_add(a_im, b_im);
        let mut w0: fpr = fpr_half(c_re);
        let mut w1: fpr = fpr_half(c_im);
        c_re = fpr_sub(a_re, b_re);
        c_im = fpr_sub(a_im, b_im);
        let mut w2: fpr = fpr_mul(fpr_add(c_re, c_im), FPR_INVSQRT8);
        let mut w3: fpr = fpr_mul(fpr_add(c_im, c_re), FPR_INVSQRT8);

        let mut x0: fpr = w2;
        let mut x1: fpr = w3;
        let mut sigma: fpr = tree1[3];
        w2 = fpr_of(samp(samp_ctx, x0, sigma) as i64);
        w3 = fpr_of(samp(samp_ctx, x1, sigma) as i64);
        a_re = fpr_sub(x0, w2);
        a_im = fpr_sub(x1, w3);
        b_re = tree1[0];
        b_im = tree1[1];
        c_re = fpr_sub(fpr_mul(a_re, b_re), fpr_mul(a_im, b_im));
        c_im = fpr_add(fpr_mul(a_re, b_im), fpr_mul(a_im, b_re));
        x0 = fpr_add(c_re, w0);
        x1 = fpr_add(c_im, w1);
        sigma = tree1[2];
        w0 = fpr_of(samp(samp_ctx, x0, sigma) as i64);
        w1 = fpr_of(samp(samp_ctx, x1, sigma) as i64);

        a_re = w0;
        a_im = w1;
        b_re = w2;
        b_im = w3;
        c_re = fpr_mul(fpr_sub(b_re, b_im), FPR_INVSQRT2);
        c_im = fpr_mul(fpr_add(b_re, b_im), FPR_INVSQRT2);
        w0 = fpr_add(a_re, c_re);
        w2 = fpr_add(a_im, c_im);
        w1 = fpr_sub(a_re, c_re);
        w3 = fpr_sub(a_im, c_im);
        z1[0] = w0;
        z1[2] = w2;
        z1[1] = w1;
        z1[3] = w3;

        w0 = fpr_sub(t1[0], w0);
        w1 = fpr_sub(t1[1], w1);
        w2 = fpr_sub(t1[2], w2);
        w3 = fpr_sub(t1[3], w3);

        a_re = w0;
        a_im = w2;
        b_re = tree[0];
        b_im = tree[2];
        w0 = fpr_sub(fpr_mul(a_re, b_re), fpr_mul(a_im, b_im));
        w2 = fpr_add(fpr_mul(a_re, b_im), fpr_mul(a_im, b_re));
        a_re = w1;
        a_im = w3;
        b_re = tree[1];
        b_im = tree[3];
        w1 = fpr_sub(fpr_mul(a_re, b_re), fpr_mul(a_im, b_im));
        w3 = fpr_add(fpr_mul(a_re, b_im), fpr_mul(a_im, b_re));

        w0 = fpr_add(w0, t0[0]);
        w1 = fpr_add(w1, t0[1]);
        w2 = fpr_add(w2, t0[2]);
        w3 = fpr_add(w3, t0[3]);

        a_re = w0;
        a_im = w2;
        b_re = w1;
        b_im = w3;
        c_re = fpr_add(a_re, b_re);
        c_im = fpr_add(a_im, b_im);
        w0 = fpr_half(c_re);
        w1 = fpr_half(c_im);
        c_re = fpr_sub(a_re, b_re);
        c_im = fpr_sub(a_im, b_im);
        w2 = fpr_mul(fpr_add(c_re, c_im), FPR_INVSQRT8);
        w3 = fpr_mul(fpr_sub(c_im, c_re), FPR_INVSQRT8);

        x0 = w2;
        x1 = w3;
        sigma = tree0[3];
        let mut y0: fpr = fpr_of(samp(samp_ctx, x0, sigma) as i64);
        let mut y1: fpr = fpr_of(samp(samp_ctx, x1, sigma) as i64);
        w2 = y0;
        w3 = y1;
        a_re = fpr_sub(x0, y0);
        a_im = fpr_sub(x1, y1);
        b_re = tree0[0];
        b_im = tree0[1];
        c_re = fpr_sub(fpr_mul(a_re, b_re), fpr_mul(a_im, b_im));
        c_im = fpr_add(fpr_mul(a_re, b_im), fpr_mul(a_im, b_re));
        x0 = fpr_add(c_re, w0);
        x1 = fpr_add(c_im, w1);
        sigma = tree0[2];
        w0 = fpr_of(samp(samp_ctx, x0, sigma) as i64);
        w1 = fpr_of(samp(samp_ctx, x1, sigma) as i64);

        a_re = w0;
        a_im = w1;
        b_re = w2;
        b_im = w3;
        c_re = fpr_mul(fpr_sub(b_re, b_im), FPR_INVSQRT2);
        c_im = fpr_mul(fpr_add(b_re, b_im), FPR_INVSQRT2);
        z0[0] = fpr_add(a_re, c_re);
        z0[2] = fpr_add(a_im, c_im);
        z0[1] = fpr_sub(a_re, c_re);
        z0[3] = fpr_sub(a_im, c_im);

        return;
    }

    if logn == 1 {
        let mut x0: fpr = t1[0];
        let mut x1: fpr = t1[1];
        let mut sigma: fpr = tree[3];
        let y0: fpr = fpr_of(samp(samp_ctx, x0, sigma) as i64);
        let y1: fpr = fpr_of(samp(samp_ctx, x1, sigma) as i64);
        z1[0] = y0;
        z1[1] = y1;
        let a_re: fpr = fpr_sub(x0, y0);
        let a_im: fpr = fpr_sub(x1, y1);
        let b_re: fpr = tree[0];
        let b_im: fpr = tree[1];
        let c_re: fpr = fpr_sub(fpr_mul(a_re, b_re), fpr_mul(a_im, b_im));
        let c_im: fpr = fpr_add(fpr_mul(a_re, b_im), fpr_mul(a_im, b_re));
        let x0: fpr = fpr_add(c_re, t0[0]);
        let x1: fpr = fpr_add(c_im, t0[1]);
        sigma = tree[2];
        z0[0] = fpr_of(samp(samp_ctx, x0, sigma) as i64);
        z0[1] = fpr_of(samp(samp_ctx, x1, sigma) as i64);

        return;
    }

    let n: usize = 1 << logn;
    let hn: usize = n >> 1;

    let (tree0, treerest) = tree.split_at_mut(n);
    let (tree1, treerest) = treerest.split_at_mut(ffLDL_treesize(logn - 1) as usize);

    let (z10, z11) = z1.split_at_mut(hn);
    poly_split_fft(z10, z11, t1, logn);
    let (tmp0, tmprest) = tmp.split_at_mut(hn);
    let (tmp1, tmp2) = tmprest.split_at_mut(hn);
    ffSampling_fft(samp, samp_ctx, tmp0, tmp1, tree1, z10, z11, logn - 1, tmp2);
    poly_merge_fft(z1, tmp0, tmp1, logn);

    tmp[..n].copy_from_slice(t1);
    poly_sub(tmp, z1, logn);
    poly_mul_fft(tmp, tree, logn);
    poly_add(tmp, t0, logn);


    let (tree0, treerest) = tree.split_at_mut(n);
    let (tree1, treerest) = treerest.split_at_mut(ffLDL_treesize(logn - 1) as usize);

    let (z00, z01) = z0.split_at_mut(hn);
    poly_split_fft(z00, z01, tmp, logn);
    let (tmp0, tmprest) = tmp.split_at_mut(hn);
    let (tmp1, tmp2) = tmprest.split_at_mut(hn);
    ffSampling_fft(samp, samp_ctx, tmp0, tmp1, tree1, z00, z01, logn - 1, tmp2);
    poly_merge_fft(z0, tmp0, tmp1, logn);
}

//TODO test
pub fn do_sign_tree(samp: SamplerZ, samp_ctx: &mut SamplerContext, s2: &mut [i16],
                    expanded_key: &mut [fpr], hm: &[u16], logn: u32, tmp: &mut [fpr]) -> bool {
    let n: usize = MKN!(logn);
    let (t0, tmprest) = tmp.split_at_mut(n);
    let (t1, tmprest) = tmprest.split_at_mut(n);

    let (b00, inter) = expanded_key.split_at_mut(n);
    let (b01, inter) = inter.split_at_mut(n);
    let (b10, inter) = inter.split_at_mut(n);
    let (b11, tree) = inter.split_at_mut(n);

    for u in 0..n {
        t0[u] = fpr_of(hm[u] as i64);
    }

    fft(t0, logn);
    let ni: fpr = FPR_INVERSE_OF_Q;
    t1.copy_from_slice(t0);
    poly_mul_fft(t1, b01, logn);
    poly_mulconst(t1, fpr_neg(ni), logn);
    poly_mul_fft(t0, b11, logn);
    poly_mulconst(t0, ni, logn);

    let (tx, tmprest) = tmprest.split_at_mut(n);
    let (ty, tmprest) = tmprest.split_at_mut(n);

    ffSampling_fft(samp, samp_ctx, tx, ty, tree, t0, t1, logn, tmprest);

    t0.copy_from_slice(tx);
    t1.copy_from_slice(ty);
    poly_mul_fft(tx, b00, logn);
    poly_mul_fft(ty, b10, logn);
    poly_add(tx, ty, logn);
    ty.copy_from_slice(t0);
    poly_mul_fft(ty, b01, logn);

    t0.copy_from_slice(tx);
    poly_mul_fft(t1, b11, logn);
    poly_add(t1, ty, logn);

    ifft(t0, logn);
    ifft(t1, logn);

    let s1tmp: &mut [i16];
    unsafe {
        s1tmp = mem::transmute(tx);
    }
    let mut sqn: u32 = 0;
    let mut ng: u32 = 0;

    for u in 0..n {
        let z: i32 = hm[u] as i32 - fpr_rint(t0[u]) as i32;

        sqn += (z * z) as u32;
        ng |= sqn;
        s1tmp[u] = z as i16;
    }
    sqn |= -((ng >> 31) as i32) as u32;

    let s2tmp: &mut [i16];
    unsafe {
        s2tmp = mem::transmute(t0);
    }

    for u in 0..n {
        s2tmp[u] = -fpr_rint(t1[u]) as i16;
    }

    if is_short_half(sqn, s2tmp, logn) > 0 {
        s2.copy_from_slice(&s2tmp[..n]);
        let tmpi: &mut [i16];
        unsafe {
            tmpi = mem::transmute(tmp);
        }
        tmpi.copy_from_slice(&s1tmp[..n]);
        return true;
    }
    return false;
}

//TODO test
#[allow(non_snake_case)]
pub fn do_sign_dyn(samp: SamplerZ, samp_ctx: &mut SamplerContext, s2: &mut [i16],
                   f: &[i8], g: &[i8], F: &[i8], G: &[i8], hm: &[u16], logn: u32, tmp: &mut [fpr]) -> bool {

    let n: usize = MKN!(logn);

    let (b00, inter) = tmp.split_at_mut(n);
    let (b01, inter) = inter.split_at_mut(n);
    let (b10, b11) = inter.split_at_mut(n);

    smallints_to_fpr(b01, f, logn);
    smallints_to_fpr(b00, g, logn);
    smallints_to_fpr(b11, F, logn);
    smallints_to_fpr(b10, G, logn);
    fft(b01, logn);
    fft(b00, logn);
    fft(b11, logn);
    fft(b10, logn);
    poly_neg(b01, logn);
    poly_neg(b11, logn);

    let (b11, rest) = b11.split_at_mut(n);
    let (t0, rest) = rest.split_at_mut(n);
    let (t1, rest) = rest.split_at_mut(n);
    t0.copy_from_slice(b01);
    poly_mulselfadj_fft(t0, logn);

    t1.copy_from_slice(b00);
    poly_muladj_fft(t1, b10, logn);
    poly_mulselfadj_fft(b00, logn);
    poly_add(b00, t0, logn);

    t0.copy_from_slice(b01);
    poly_muladj_fft(b01, b11, logn);
    poly_add(b01, t1, logn);

    poly_mulselfadj_fft(b10, logn);
    t1.copy_from_slice(b11);
    poly_mulselfadj_fft(t1, logn);
    poly_add(b10, t1, logn);

    let (g00, inter) = tmp.split_at_mut(n);
    let (g01, inter) = inter.split_at_mut(n);
    let (g11, inter) = inter.split_at_mut(n);
    let (b11, inter) = inter.split_at_mut(n);
    let (b01, inter) = inter.split_at_mut(n);
    let (t0, inter) = inter.split_at_mut(n);
    let (t1, inter) = inter.split_at_mut(n);

    for u in 0..n {
        t0[u] = fpr_of(hm[u] as i64);
    }

    fft(t0, logn);
    let ni: u64 = FPR_INVERSE_OF_Q;
    t1.copy_from_slice(t0);
    poly_mul_fft(t1, b01, logn);
    poly_mulconst(t1, fpr_neg(ni), logn);
    poly_mul_fft(t0, b11, logn);
    poly_mulconst(t0, ni, logn);

    b11.copy_from_slice(t0);
    b01.copy_from_slice(t1);

    let t0 = b11;
    let t1 = b01;

    ffSampling_fft_dyntree(samp, samp_ctx, t0, t1, g00, g01, g11, logn, logn, inter);

    let (b00, inter) = tmp.split_at_mut(n);
    let (b01, inter) = inter.split_at_mut(n);
    let (b10, inter) = inter.split_at_mut(n);
    let (b11, inter) = inter.split_at_mut(n);
    let (t0, inter) = inter.split_at_mut(n);
    let (t1, inter) = inter.split_at_mut(n);

    t1.copy_from_slice(t0);
    t0.copy_from_slice(b11);

    smallints_to_fpr(b01, f, logn);
    smallints_to_fpr(b00, g, logn);
    smallints_to_fpr(b11, F, logn);
    smallints_to_fpr(b10, G, logn);
    fft(b01, logn);
    fft(b00, logn);
    fft(b11, logn);
    fft(b10, logn);
    poly_neg(b01, logn);
    poly_neg(b11, logn);
    let(tx, rest) = inter.split_at_mut(n);
    let(ty, rest) = rest.split_at_mut(n);

    tx.copy_from_slice(t0);
    ty.copy_from_slice(t1);
    poly_mul_fft(tx, b00, logn);
    poly_mul_fft(ty, b10, logn);
    poly_add(tx, ty, logn);
    ty.copy_from_slice(t0);
    poly_mul_fft(ty, b01, logn);

    t0.copy_from_slice(tx);
    poly_mul_fft(t1, b11, logn);
    poly_add(t1, ty, logn);
    ifft(t0, logn);
    ifft(t1, logn);

    let (s1tmpf, s2tmpf) = tmp.split_at_mut(6*n);
    let s1tmp: &mut [i16];
    unsafe {
        s1tmp =  mem::transmute(s1tmpf);
    }
    let (s1tmp, t0i) = s1tmp.split_at_mut(4*4*n); //s1tmp now has 4 times as many indices and t0 was previously located at 4*n
    let t0: &mut [fpr];
    unsafe {
        t0 = mem::transmute(t0i);
    }

    let mut sqn = 0;
    let mut ng = 0;

    for u in 0..n {
        let z: i32 = hm[u] as i32 - fpr_rint(t0[u]) as i32;
        sqn += (z * z) as u32;
        ng |= sqn;
        s1tmp[u] = z as i16;
    }
    sqn |= -((ng >> 31) as i32) as u32;

    let s2tmp: &mut [i16];
    unsafe {
        s2tmp = mem::transmute(s2tmpf);
    }

    let t1 = &t0[n..];
    for u in 0..n {
        s2tmp[u] = -fpr_rint(t1[u]) as i16;
    }

    if is_short_half(sqn, s2tmp, logn) > 0 {
        s2.copy_from_slice(&s2tmp[..n]);
        let tmpi: &mut [i16];
        unsafe {
            tmpi = mem::transmute(tmp);
        }
        tmpi.copy_from_slice(&s1tmp[..n]);
        return true;
    }
    return false;
}

pub fn do_sign_dyn_same(samp: SamplerZ, samp_ctx: &mut SamplerContext, s2: &mut [i16],
                   f: &[i8], g: &[i8], F: &[i8], G: &[i8], logn: u32, tmp: &mut [fpr]) -> bool {

    let n: usize = MKN!(logn);

    let (b00, inter) = tmp.split_at_mut(n);
    let (b01, inter) = inter.split_at_mut(n);
    let (b10, b11) = inter.split_at_mut(n);

    smallints_to_fpr(b01, f, logn);
    smallints_to_fpr(b00, g, logn);
    smallints_to_fpr(b11, F, logn);
    smallints_to_fpr(b10, G, logn);
    fft(b01, logn);
    fft(b00, logn);
    fft(b11, logn);
    fft(b10, logn);
    poly_neg(b01, logn);
    poly_neg(b11, logn);

    let (b11, rest) = b11.split_at_mut(n);
    let (t0, rest) = rest.split_at_mut(n);
    let (t1, rest) = rest.split_at_mut(n);
    t0.copy_from_slice(b01);
    poly_mulselfadj_fft(t0, logn);

    t1.copy_from_slice(b00);
    poly_muladj_fft(t1, b10, logn);
    poly_mulselfadj_fft(b00, logn);
    poly_add(b00, t0, logn);

    t0.copy_from_slice(b01);
    poly_muladj_fft(b01, b11, logn);
    poly_add(b01, t1, logn);

    poly_mulselfadj_fft(b10, logn);
    t1.copy_from_slice(b11);
    poly_mulselfadj_fft(t1, logn);
    poly_add(b10, t1, logn);

    let (g00, inter) = tmp.split_at_mut(n);
    let (g01, inter) = inter.split_at_mut(n);
    let (g11, inter) = inter.split_at_mut(n);
    let (b11, inter) = inter.split_at_mut(n);
    let (b01, inter) = inter.split_at_mut(n);
    let (t0, inter) = inter.split_at_mut(n);
    let (t1, inter) = inter.split_at_mut(n);

    for u in 0..n {
        t0[u] = fpr_of((s2[u] as u16) as i64);
    }

    fft(t0, logn);
    let ni: u64 = FPR_INVERSE_OF_Q;
    t1.copy_from_slice(t0);
    poly_mul_fft(t1, b01, logn);
    poly_mulconst(t1, fpr_neg(ni), logn);
    poly_mul_fft(t0, b11, logn);
    poly_mulconst(t0, ni, logn);

    b11.copy_from_slice(t0);
    b01.copy_from_slice(t1);

    let t0 = b11;
    let t1 = b01;

    ffSampling_fft_dyntree(samp, samp_ctx, t0, t1, g00, g01, g11, logn, logn, inter);

    let (b00, inter) = tmp.split_at_mut(n);
    let (b01, inter) = inter.split_at_mut(n);
    let (b10, inter) = inter.split_at_mut(n);
    let (b11, inter) = inter.split_at_mut(n);
    let (t0, inter) = inter.split_at_mut(n);
    let (t1, inter) = inter.split_at_mut(n);

    t1.copy_from_slice(t0);
    t0.copy_from_slice(b11);

    smallints_to_fpr(b01, f, logn);
    smallints_to_fpr(b00, g, logn);
    smallints_to_fpr(b11, F, logn);
    smallints_to_fpr(b10, G, logn);
    fft(b01, logn);
    fft(b00, logn);
    fft(b11, logn);
    fft(b10, logn);
    poly_neg(b01, logn);
    poly_neg(b11, logn);
    let(tx, rest) = inter.split_at_mut(n);
    let(ty, rest) = rest.split_at_mut(n);

    tx.copy_from_slice(t0);
    ty.copy_from_slice(t1);
    poly_mul_fft(tx, b00, logn);
    poly_mul_fft(ty, b10, logn);
    poly_add(tx, ty, logn);
    ty.copy_from_slice(t0);
    poly_mul_fft(ty, b01, logn);

    t0.copy_from_slice(tx);
    poly_mul_fft(t1, b11, logn);
    poly_add(t1, ty, logn);
    ifft(t0, logn);
    ifft(t1, logn);

    let (s1tmpf, s2tmpf) = tmp.split_at_mut(6*n);
    let s1tmp: &mut [i16];
    unsafe {
        s1tmp =  mem::transmute(s1tmpf);
    }
    let (s1tmp, t0i) = s1tmp.split_at_mut(4*4*n); //s1tmp now has 4 times as many indices and t0 was previously located at 4*n
    let t0: &mut [fpr];
    unsafe {
        t0 = mem::transmute(t0i);
    }

    let mut sqn = 0;
    let mut ng = 0;

    for u in 0..n {
        let z: i32 = (s2[u] as u16) as i32 - fpr_rint(t0[u]) as i32;
        sqn += (z * z) as u32;
        ng |= sqn;
        s1tmp[u] = z as i16;
    }
    sqn |= -((ng >> 31) as i32) as u32;

    let s2tmp: &mut [i16];
    unsafe {
        s2tmp = mem::transmute(s2tmpf);
    }

    let t1 = &t0[n..];
    for u in 0..n {
        s2tmp[u] = -fpr_rint(t1[u]) as i16;
    }

    if is_short_half(sqn, s2tmp, logn) > 0 {
        s2.copy_from_slice(&s2tmp[..n]);
        let tmpi: &mut [i16];
        unsafe {
            tmpi = mem::transmute(tmp);
        }
        tmpi.copy_from_slice(&s1tmp[..n]);
        return true;
    }
    return false;
}

pub fn sampler(spc: &mut SamplerContext, mu: fpr, isigma: fpr) -> i32 {
    let s: i64 = fpr_floor(mu);
    let r: fpr = fpr_sub(mu, fpr_of(s));

    let dss: fpr = fpr_half(fpr_sqr(isigma));
    let ccs: fpr = fpr_mul(isigma, spc.sigma_min);

    loop {
        let z0: i32 = gaussian0_sampler(&mut spc.p);
        let b: i32 = prng_get_u8(&mut spc.p) as i32 & 1;
        let z:i64 = (b + ((b << 1) - 1) * z0) as i64;

        let mut x = fpr_mul(fpr_sqr(fpr_sub(fpr_of(z), r)), dss);
        x = fpr_sub(x, fpr_mul(fpr_of((z0 * z0) as i64), FPR_INV_2SQRSIGMA0));

        //TODO fix infinite loop
        if BerExp(&mut spc.p, x, ccs) > 0 {
            return (s + z) as i32;
        }
    }
}

pub fn gaussian0_sampler(p: &mut Prng) -> i32 {
    const DIST: [u32; 54] = [
        10745844,  3068844,  3741698,
        5559083,  1580863,  8248194,
        2260429, 13669192,  2736639,
        708981,  4421575, 10046180,
        169348,  7122675,  4136815,
        30538, 13063405,  7650655,
        4132, 14505003,  7826148,
        417, 16768101, 11363290,
        31,  8444042,  8086568,
        1, 12844466,   265321,
        0,  1232676, 13644283,
        0,    38047,  9111839,
        0,      870,  6138264,
        0,       14, 12545723,
        0,        0,  3104126,
        0,        0,    28824,
        0,        0,      198,
        0,        0,        1
    ];

    let lo: u64 = prng_get_u64(p);
    let hi: u8 = prng_get_u8(p);
    let v0: u32 = (lo as u32) & 0xFFFFFF;
    let v1: u32 = (lo >> 24) as u32 & 0xFFFFFF;
    let v2: u32 = (lo >> 48) as u32 | ((hi as u32) << 16);

    let mut z: i32 = 0;
    for u in (0..DIST.len()).step_by(3) {
        let w0: u32 = DIST[u + 2];
        let w1: u32 = DIST[u + 1];
        let w2: u32 = DIST[u + 0];

        let mut cc = v0.wrapping_sub(w0) >> 31;
        cc = v1.wrapping_sub(w1).wrapping_sub(cc) >> 31;
        cc = v2.wrapping_sub(w2).wrapping_sub(cc) >> 31;

        z += cc as i32;
    }

    return z;
}

#[allow(non_snake_case)]
pub fn BerExp(p: &mut Prng, x: fpr, ccs: fpr) -> i32 {
    let mut s: i32 = fpr_trunc(fpr_mul(x, FPR_INV_LOG2)) as i32;
    let r: fpr = fpr_sub(x, fpr_mul(fpr_of(s as i64), FPR_LOG2));

    let mut sw: u32 = s as u32;
    sw ^= (sw ^ 63) & (-((63 as u32).wrapping_sub(sw).wrapping_shr(31) as i32) as u32);
    s = sw as i32;

    let z: u64 = ((fpr_expm_p63(r as u64, ccs) << 1).wrapping_sub(1)).wrapping_shr(s as u32);

    let mut w: u32;
    let mut i: i32 = 64;
    loop {
        i -= 8;
        w = ((prng_get_u8(p) as u32).wrapping_sub((z >> i) as u32 & 0xFF)) as u32;
        if w != 0 || i <= 0 {
            break;
        }
    }
    return (w >> 31) as i32;
}

//TODO test
pub fn sign_tree(sig: &mut [i16], rng: &mut InnerShake256Context, expanded_key: &mut [fpr], hm: &[u16],
                 logn: u32, tmp: &mut [u8]) {

    let mut ftmp: &mut [fpr];
    unsafe {
        ftmp = mem::transmute(tmp);
    }

    loop {
        let mut spc: SamplerContext = SamplerContext {p: Prng {buf: [0; 512], ptr: 0, state: State {d: [0; 256]}, typ: 0}, sigma_min: FPR_SIGMA_MIN[logn as usize]};
        prng_init(&mut spc.p, rng);
        let samp: SamplerZ = sampler;

        if do_sign_tree(samp, &mut spc, sig, expanded_key, hm, logn, ftmp) {
            break;
        }
    }
}

//TODO test
#[allow(non_snake_case)]
pub fn sign_dyn(sig: &mut [i16], rng: &mut InnerShake256Context, f: &[i8], g: &[i8],
                F: &[i8], G: &[i8], hm: &[u16], logn: u32, tmp: &mut [u8]) {

    let mut ftmp: &mut [fpr];
    unsafe {
        ftmp = mem::transmute(tmp);
    }

    loop {
        let mut spc: SamplerContext = SamplerContext {p: Prng {buf: [0; 512], ptr: 0, state: State {d: [0; 256]}, typ: 0}, sigma_min: FPR_SIGMA_MIN[logn as usize]};
        prng_init(&mut spc.p, rng);
        let samp: SamplerZ = sampler;
        //let mut samp_ctx: &SamplerContext = &spc;

        if do_sign_dyn(samp, &mut spc, sig, f, g, F, G, hm, logn, &mut ftmp) {
            break;
        }
    }
}

//TODO test
#[allow(non_snake_case)]
pub fn sign_dyn_same(sig: &mut [i16], rng: &mut InnerShake256Context, f: &[i8], g: &[i8],
                F: &[i8], G: &[i8], logn: u32, tmp: &mut [u8]) {

    let mut ftmp: &mut [fpr];
    unsafe {
        ftmp = mem::transmute(tmp);
    }

    loop {
        let mut spc: SamplerContext = SamplerContext {p: Prng {buf: [0; 512], ptr: 0, state: State {d: [0; 256]}, typ: 0}, sigma_min: FPR_SIGMA_MIN[logn as usize]};
        prng_init(&mut spc.p, rng);
        let samp: SamplerZ = sampler;
        //let mut samp_ctx: &SamplerContext = &spc;

        if do_sign_dyn_same(samp, &mut spc, sig, f, g, F, G, logn, &mut ftmp) {
            break;
        }
    }
}