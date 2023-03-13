use bytemuck;

use crate::common::is_short_half;
use crate::falcon::fpr;
use crate::fft::{fft, ifft, poly_add, poly_LDL_fft, poly_LDLmv_fft, poly_merge_fft, poly_mul_fft, poly_muladj_fft, poly_mulconst, poly_mulselfadj_fft, poly_neg, poly_split_fft, poly_sub};
use crate::fpr::{fpr_add, fpr_expm_p63, fpr_floor, fpr_half, FPR_INV_2SQRSIGMA0, FPR_INV_LOG2, FPR_INV_SIGMA, FPR_INVERSE_OF_Q, FPR_INVSQRT2, FPR_INVSQRT8, FPR_LOG2, fpr_mul, fpr_neg, fpr_of, fpr_rint, FPR_SIGMA_MIN, fpr_sqr, fpr_sqrt, fpr_sub, fpr_trunc};
use crate::MKN;
use crate::rng::{Prng, prng_get_u64, prng_get_u8, prng_init, State};
use crate::shake::InnerShake256Context;

/*#[allow(non_snake_case)]
#[inline(always)]
pub fn ffLDL_treesize(logn: u32) -> u32 {
    return (logn + 1) << logn;
}

pub fn smallints_to_fpr(r: &mut [fpr], t: &[i8], logn: u32) {
    let n: usize = MKN!(logn);

    for u in 0..n {
        r[u] = fpr_of(t[u] as i64);
    }
}

pub struct SamplerContext {
    pub p: Prng,
    pub sigma_min: fpr
}

type SamplerZ = fn(&mut SamplerContext, fpr, fpr) -> i32;

#[allow(non_snake_case)]
pub fn ffSampling_fft(samp: SamplerZ, samp_ctx: &mut SamplerContext, z0: &mut [fpr],
                      z1: &mut [fpr], tree: &[fpr], t0: &mut [fpr], t1: &[fpr], logn: u32, tmp: &mut [fpr]) {

    if logn == 2 {
        let (_, treerest) = tree.split_at(4);
        let (tree0, tree1) = treerest.split_at(4);

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
        let mut w3: fpr = fpr_mul(fpr_sub(c_im, c_re), FPR_INVSQRT8);

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
        let y0: fpr = fpr_of(samp(samp_ctx, x0, sigma) as i64);
        let y1: fpr = fpr_of(samp(samp_ctx, x1, sigma) as i64);
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
        let x0: fpr = t1[0];
        let x1: fpr = t1[1];
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

    let tree1: &[fpr] = &tree[n + ffLDL_treesize(logn - 1) as usize..];


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


    let tree0: &[fpr] = &tree[n..];

    let (z00, z01) = z0.split_at_mut(hn);
    poly_split_fft(z00, z01, tmp, logn);
    let (tmp0, tmprest) = tmp.split_at_mut(hn);
    let (tmp1, tmp2) = tmprest.split_at_mut(hn);
    ffSampling_fft(samp, samp_ctx, tmp0, tmp1, tree0, z00, z01, logn - 1, tmp2);
    poly_merge_fft(z0, tmp0, tmp1, logn);
}


pub fn do_sign_tree(samp: SamplerZ, samp_ctx: &mut SamplerContext, s2: &mut [i16],
                    expanded_key: &[fpr], hm: &[u16], logn: u32, tmp: &mut [fpr]) -> bool {
    let n: usize = MKN!(logn);
    let (t0, tmprest) = tmp.split_at_mut(n);
    let (t1, tmprest) = tmprest.split_at_mut(n);

    let (b00, inter) = expanded_key.split_at(n);
    let (b01, inter) = inter.split_at(n);
    let (b10, inter) = inter.split_at(n);
    let (b11, tree) = inter.split_at(n);

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

    let s1tmp: &mut [i16] = bytemuck::cast_slice_mut(tx);
    let mut sqn: u32 = 0;
    let mut ng: u32 = 0;

    for u in 0..n {
        let z: i32 = hm[u] as i32 - fpr_rint(t0[u]) as i32;

        sqn += (z * z) as u32;
        ng |= sqn;
        s1tmp[u] = z as i16;
    }
    sqn |= -((ng >> 31) as i32) as u32;

    let s2tmp: &mut [i16] = bytemuck::cast_slice_mut(t0);

    for u in 0..n {
        s2tmp[u] = -fpr_rint(t1[u]) as i16;
    }
    if is_short_half(sqn, s2tmp, logn) > 0 {
        s2[..n].copy_from_slice(&s2tmp[..n]);
        s2tmp[..n].copy_from_slice(&s1tmp[..n]);
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
        let z: i64 = (b + ((b << 1) - 1) * z0) as i64;

        let mut x = fpr_mul(fpr_sqr(fpr_sub(fpr_of(z), r)), dss);
        x = fpr_sub(x, fpr_mul(fpr_of((z0 * z0) as i64), FPR_INV_2SQRSIGMA0));

        if BerExp(&mut spc.p, x, ccs) > 0 {
            return (s + z) as i32;
        }
    }
}

pub fn gaussian0_sampler(p: &mut Prng) -> i32 {
    const DIST: [u32; 54] = [
        10745844, 3068844, 3741698,
        5559083, 1580863, 8248194,
        2260429, 13669192, 2736639,
        708981, 4421575, 10046180,
        169348, 7122675, 4136815,
        30538, 13063405, 7650655,
        4132, 14505003, 7826148,
        417, 16768101, 11363290,
        31, 8444042, 8086568,
        1, 12844466, 265321,
        0, 1232676, 13644283,
        0, 38047, 9111839,
        0, 870, 6138264,
        0, 14, 12545723,
        0, 0, 3104126,
        0, 0, 28824,
        0, 0, 198,
        0, 0, 1
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
}*/


pub fn sign_tree<const LENGTH: usize>(sig: &mut [[i16; LENGTH]], rng: &mut InnerShake256Context, expanded_key: &[fpr], hm: &[u16],
                 logn: u32, tmp: &mut [u8]) {

    let ftmp: &mut [fpr] = bytemuck::cast_slice_mut(tmp);

    loop {
        let mut spc: SamplerContext = SamplerContext {p: Prng {buf: [0; 512], ptr: 0, state: State {d: [0; 256]}, typ: 0}, sigma_min: FPR_SIGMA_MIN[logn as usize]};
        prng_init(&mut spc.p, rng);
        let samp: SamplerZ = sampler;

        if do_sign_tree(samp, &mut spc, sig, expanded_key, hm, logn, ftmp) {
            break;
        }
    }
}

