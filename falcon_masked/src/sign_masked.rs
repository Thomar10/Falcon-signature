use falcon::common::is_short_half;
use falcon::falcon::fpr;
use falcon::fpr::{fpr_add as add, FPR_INVERSE_OF_Q, FPR_INVSQRT2, FPR_INVSQRT8, fpr_rint as rint, FPR_SIGMA_MIN};
use falcon::MKN;
use falcon::rng::{Prng, prng_init, State};
use falcon::shake::InnerShake256Context;
use falcon::sign::{ffLDL_treesize, sampler, SamplerContext, SamplerZ};

use crate::fft_masked::{fft, ifft, poly_add, poly_merge_fft, poly_mul_fft, poly_mulconst, poly_split_fft, poly_sub};
use crate::fpr_masked::{fpr_add, fpr_half, fpr_mul, fpr_neg_fpr, fpr_of, fpr_of_i, fpr_sub};

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

*/

#[allow(non_snake_case)]
pub fn ffSampling_fft<const ORDER: usize>(samp: SamplerZ, samp_ctx: &mut SamplerContext, z0: &mut [[fpr; ORDER]],
                                          z1: &mut [[fpr; ORDER]], tree: &[[fpr; ORDER]], t0: &mut [[fpr; ORDER]], t1: &[[fpr; ORDER]], logn: u32, tmp: &mut [[fpr; ORDER]]) {
    let mut fpr_invsqrt8: [fpr; ORDER] = [0; ORDER];
    fpr_invsqrt8[0] = FPR_INVSQRT8;
    let mut fpr_invsqrt2: [fpr; ORDER] = [0; ORDER];
    fpr_invsqrt2[0] = FPR_INVSQRT2;
    if logn == 2 {
        let (_, treerest): (_, &[[fpr; ORDER]]) = tree.split_at(4);
        let (tree0, tree1): (&[[fpr; ORDER]], &[[fpr; ORDER]]) = treerest.split_at(4);

        let mut a_re: [fpr; ORDER] = t1[0];
        let mut a_im: [fpr; ORDER] = t1[2];
        let mut b_re: [fpr; ORDER] = t1[1];
        let mut b_im: [fpr; ORDER] = t1[3];
        let mut c_re: [fpr; ORDER] = fpr_add(&a_re, &b_re);
        let mut c_im: [fpr; ORDER] = fpr_add(&a_im, &b_im);
        let mut w0: [fpr; ORDER] = fpr_half(&c_re);
        let mut w1: [fpr; ORDER] = fpr_half(&c_im);
        c_re = fpr_sub(&a_re, &b_re);
        c_im = fpr_sub(&a_im, &b_im);
        let mut w2: [fpr; ORDER] = fpr_mul(&fpr_add::<ORDER>(&c_re, &c_im), &fpr_invsqrt8);
        let mut w3: [fpr; ORDER] = fpr_mul(&fpr_sub::<ORDER>(&c_im, &c_re), &fpr_invsqrt8);

        let mut x0: [fpr; ORDER] = w2;
        let mut x1: [fpr; ORDER] = w3;
        let mut sigma: [fpr; ORDER] = tree1[3];
        w2 = fpr_of_i(samp(samp_ctx, add(x0[0], x0[1]), add(sigma[0], sigma[1])) as i64);
        w3 = fpr_of_i(samp(samp_ctx, add(x1[0], x1[1]), add(sigma[0], sigma[1])) as i64);
        a_re = fpr_sub(&x0, &w2);
        a_im = fpr_sub(&x1, &w3);
        b_re = tree1[0];
        b_im = tree1[1];
        c_re = fpr_sub(&fpr_mul::<ORDER>(&a_re, &b_re), &fpr_mul::<ORDER>(&a_im, &b_im));
        c_im = fpr_add(&fpr_mul::<ORDER>(&a_re, &b_im), &fpr_mul::<ORDER>(&a_im, &b_re));
        x0 = fpr_add(&c_re, &w0);
        x1 = fpr_add(&c_im, &w1);
        sigma = tree1[2];
        w0 = fpr_of_i(samp(samp_ctx, add(x0[0], x0[1]), add(sigma[0], sigma[1])) as i64);
        w1 = fpr_of_i(samp(samp_ctx, add(x1[0], x1[1]), add(sigma[0], sigma[1])) as i64);

        a_re = w0;
        a_im = w1;
        b_re = w2;
        b_im = w3;
        c_re = fpr_mul(&fpr_sub::<ORDER>(&b_re, &b_im), &fpr_invsqrt2);
        c_im = fpr_mul(&fpr_add::<ORDER>(&b_re, &b_im), &fpr_invsqrt2);
        w0 = fpr_add(&a_re, &c_re);
        w2 = fpr_add(&a_im, &c_im);
        w1 = fpr_sub(&a_re, &c_re);
        w3 = fpr_sub(&a_im, &c_im);
        z1[0] = w0;
        z1[2] = w2;
        z1[1] = w1;
        z1[3] = w3;

        w0 = fpr_sub(&t1[0], &w0);
        w1 = fpr_sub(&t1[1], &w1);
        w2 = fpr_sub(&t1[2], &w2);
        w3 = fpr_sub(&t1[3], &w3);

        a_re = w0;
        a_im = w2;
        b_re = tree[0];
        b_im = tree[2];
        w0 = fpr_sub(&fpr_mul::<ORDER>(&a_re, &b_re), &fpr_mul::<ORDER>(&a_im, &b_im));
        w2 = fpr_add(&fpr_mul::<ORDER>(&a_re, &b_im), &fpr_mul::<ORDER>(&a_im, &b_re));
        a_re = w1;
        a_im = w3;
        b_re = tree[1];
        b_im = tree[3];
        w1 = fpr_sub(&fpr_mul::<ORDER>(&a_re, &b_re), &fpr_mul::<ORDER>(&a_im, &b_im));
        w3 = fpr_add(&fpr_mul::<ORDER>(&a_re, &b_im), &fpr_mul::<ORDER>(&a_im, &b_re));

        w0 = fpr_add(&w0, &t0[0]);
        w1 = fpr_add(&w1, &t0[1]);
        w2 = fpr_add(&w2, &t0[2]);
        w3 = fpr_add(&w3, &t0[3]);

        a_re = w0;
        a_im = w2;
        b_re = w1;
        b_im = w3;
        c_re = fpr_add(&a_re, &b_re);
        c_im = fpr_add(&a_im, &b_im);
        w0 = fpr_half(&c_re);
        w1 = fpr_half(&c_im);
        c_re = fpr_sub(&a_re, &b_re);
        c_im = fpr_sub(&a_im, &b_im);
        w2 = fpr_mul(&fpr_add::<ORDER>(&c_re, &c_im), &fpr_invsqrt8);
        w3 = fpr_mul(&fpr_sub::<ORDER>(&c_im, &c_re), &fpr_invsqrt8);

        x0 = w2;
        x1 = w3;
        sigma = tree0[3];
        let y0: [fpr; ORDER] = fpr_of_i(samp(samp_ctx, add(x0[0], x0[1]), add(sigma[0], sigma[1])) as i64);
        let y1: [fpr; ORDER] = fpr_of_i(samp(samp_ctx, add(x1[0], x1[1]), add(sigma[0], sigma[1])) as i64);
        w2 = y0;
        w3 = y1;
        a_re = fpr_sub(&x0, &y0);
        a_im = fpr_sub(&x1, &y1);
        b_re = tree0[0];
        b_im = tree0[1];
        c_re = fpr_sub(&fpr_mul::<ORDER>(&a_re, &b_re), &fpr_mul::<ORDER>(&a_im, &b_im));
        c_im = fpr_add(&fpr_mul::<ORDER>(&a_re, &b_im), &fpr_mul::<ORDER>(&a_im, &b_re));
        x0 = fpr_add(&c_re, &w0);
        x1 = fpr_add(&c_im, &w1);
        sigma = tree0[2];
        w0 = fpr_of_i(samp(samp_ctx, add(x0[0], x0[1]), add(sigma[0], sigma[1])) as i64);
        w1 = fpr_of_i(samp(samp_ctx, add(x1[0], x1[1]), add(sigma[0], sigma[1])) as i64);

        a_re = w0;
        a_im = w1;
        b_re = w2;
        b_im = w3;
        c_re = fpr_mul(&fpr_sub::<ORDER>(&b_re, &b_im), &fpr_invsqrt2);
        c_im = fpr_mul(&fpr_add::<ORDER>(&b_re, &b_im), &fpr_invsqrt2);
        z0[0] = fpr_add(&a_re, &c_re);
        z0[2] = fpr_add(&a_im, &c_im);
        z0[1] = fpr_sub(&a_re, &c_re);
        z0[3] = fpr_sub(&a_im, &c_im);

        return;
    }

    if logn == 1 {
        let x0: [fpr; ORDER] = t1[0];
        let x1: [fpr; ORDER] = t1[1];
        let mut sigma: [fpr; ORDER] = tree[3];
        let y0: [fpr; ORDER] = fpr_of_i(samp(samp_ctx, add(x0[0], x0[1]), add(sigma[0], sigma[1])) as i64);
        let y1: [fpr; ORDER] = fpr_of_i(samp(samp_ctx, add(x1[0], x1[1]), add(sigma[0], sigma[1])) as i64);
        z1[0] = y0;
        z1[1] = y1;
        let a_re: [fpr; ORDER] = fpr_sub(&x0, &y0);
        let a_im: [fpr; ORDER] = fpr_sub(&x1, &y1);
        let b_re: [fpr; ORDER] = tree[0];
        let b_im: [fpr; ORDER] = tree[1];
        let c_re: [fpr; ORDER] = fpr_sub(&fpr_mul::<ORDER>(&a_re, &b_re), &fpr_mul::<ORDER>(&a_im, &b_im));
        let c_im: [fpr; ORDER] = fpr_add(&fpr_mul::<ORDER>(&a_re, &b_im), &fpr_mul::<ORDER>(&a_im, &b_re));
        let x0: [fpr; ORDER] = fpr_add(&c_re, &t0[0]);
        let x1: [fpr; ORDER] = fpr_add(&c_im, &t0[1]);
        sigma = tree[2];
        z0[0] = fpr_of_i(samp(samp_ctx, add(x0[0], x0[1]), add(sigma[0], sigma[1])) as i64);
        z0[1] = fpr_of_i(samp(samp_ctx, add(x1[0], x1[1]), add(sigma[0], sigma[1])) as i64);

        return;
    }

    let n: usize = 1 << logn;
    let hn: usize = n >> 1;

    let tree1: &[[fpr; ORDER]] = &tree[n + ffLDL_treesize(logn - 1) as usize..];


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


    let tree0: &[[fpr; ORDER]] = &tree[n..];

    let (z00, z01) = z0.split_at_mut(hn);
    poly_split_fft(z00, z01, tmp, logn);
    let (tmp0, tmprest) = tmp.split_at_mut(hn);
    let (tmp1, tmp2) = tmprest.split_at_mut(hn);
    ffSampling_fft(samp, samp_ctx, tmp0, tmp1, tree0, z00, z01, logn - 1, tmp2);
    poly_merge_fft(z0, tmp0, tmp1, logn);
}

pub fn do_sign_tree<const ORDER: usize, const LENGTH: usize>(samp: SamplerZ, samp_ctx: &mut SamplerContext, s2: &mut [i16],
                                                             expanded_key: &[[fpr; ORDER]], hm: &[[u16; ORDER]], logn: u32, tmp: &mut [[fpr; ORDER]]) -> bool {
    let n: usize = MKN!(logn);
    let (t0, tmprest) = tmp.split_at_mut(n);
    let (t1, tmprest) = tmprest.split_at_mut(n);

    let (b00, inter) = expanded_key.split_at(n);
    let (b01, inter) = inter.split_at(n);
    let (b10, inter) = inter.split_at(n);
    let (b11, tree) = inter.split_at(n);

    for u in 0..n {
        let mut hmm: [i64; ORDER] = [0; ORDER];
        for i in 0..ORDER {
            hmm[i] = hm[u][i] as i64;
        }
        t0[u] = fpr_of::<ORDER>(&hmm);
    }

    fft(t0, logn);
    let ni = FPR_INVERSE_OF_Q;
    t1.copy_from_slice(t0);
    poly_mul_fft(t1, b01, logn);
    poly_mulconst(t1, fpr_neg_fpr(ni), logn);
    poly_mul_fft(t0, b11, logn);
    poly_mulconst(t0, ni, logn);

    let (tx, tmprest): (&mut [[fpr; ORDER]], &mut [[fpr; ORDER]]) = tmprest.split_at_mut(n);
    let (ty, tmprest): (&mut [[fpr; ORDER]], &mut [[fpr; ORDER]]) = tmprest.split_at_mut(n);

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

    let mut tx_r: [fpr; LENGTH] = reconstruct_fpr::<ORDER, LENGTH>(tx);
    let s1tmp: &mut [i16] = bytemuck::cast_slice_mut::<fpr, i16>(&mut tx_r);
    let mut sqn: u32 = 0;
    let mut ng: u32 = 0;
    let hmm: [u16; LENGTH] = reconstruct_hash_message::<ORDER, LENGTH>(hm);
    let mut t0_r = reconstruct_fpr::<ORDER, LENGTH>(t0);
    for u in 0..n {
        let z: i32 = hmm[u] as i32 - rint(t0_r[u]) as i32;
        sqn = sqn.wrapping_add((z * z) as u32);
        ng |= sqn;
        s1tmp[u] = z as i16;
    }
    sqn |= -((ng >> 31) as i32) as u32;

    let s2tmp: &mut [i16] = bytemuck::cast_slice_mut(&mut t0_r);
    let t1_r = reconstruct_fpr::<ORDER, LENGTH>(t1);

    for u in 0..n {
        s2tmp[u] = -rint(t1_r[u]) as i16;
    }
    if is_short_half(sqn, s2tmp, logn) > 0 {
        s2[..n].copy_from_slice(&s2tmp[..n]);
        s2tmp[..n].copy_from_slice(&s1tmp[..n]);
        return true;
    }
    return false;
}

fn reconstruct_fpr<const ORDER: usize, const LENGTH: usize>(hm: &[[fpr; ORDER]]) -> [fpr; LENGTH] {
    let mut res: [fpr; LENGTH] = [0; LENGTH];
    for i in 0..LENGTH {
        res[i] = add(hm[i][0], hm[i][1]);
    }
    res
}

fn reconstruct_hash_message<const ORDER: usize, const LENGTH: usize>(hm: &[[u16; ORDER]]) -> [u16; LENGTH] {
    let mut res: [u16; LENGTH] = [0; LENGTH];
    for i in 0..LENGTH {
        res[i] = hm[i][0] + hm[i][1];
    }
    res
}

pub fn sign_tree<const ORDER: usize, const TMP_LENGTH: usize, const LENGTH: usize>(sig: &mut [i16], rng: &mut InnerShake256Context, expanded_key: &[[fpr; ORDER]], hm: &[[u16; ORDER]],
                                                                                   logn: u32) {
    let ftmp: &mut [[fpr; ORDER]; TMP_LENGTH] = &mut [[0; ORDER]; TMP_LENGTH];
    for i in 0..TMP_LENGTH {
        ftmp[i] = [0; ORDER];
    }


    loop {
        let mut spc: SamplerContext = SamplerContext { p: Prng { buf: [0; 512], ptr: 0, state: State { d: [0; 256] }, typ: 0 }, sigma_min: FPR_SIGMA_MIN[logn as usize] };
        prng_init(&mut spc.p, rng);
        let samp: SamplerZ = sampler;

        if do_sign_tree::<ORDER, LENGTH>(samp, &mut spc, sig, expanded_key, hm, logn, ftmp) {
            break;
        }
    }
}
