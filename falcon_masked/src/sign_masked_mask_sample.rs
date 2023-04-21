use falcon::{falcon_tmpsize_signtree, MKN};
use falcon::common::is_short_half;
use falcon::falcon::fpr;
use falcon::fpr::{fpr_add as add, fpr_expm_p63 as expm_p63, FPR_INV_2SQRSIGMA0, FPR_INV_LOG2, FPR_INVERSE_OF_Q, FPR_INVSQRT2, FPR_INVSQRT8, FPR_LOG2, fpr_mul as mul, fpr_of as of, fpr_rint as rint, FPR_SIGMA_MIN};
use falcon::rng::{Prng, prng_get_u8, prng_init, State};
use falcon::shake::InnerShake256Context;
use falcon::sign::{ffLDL_treesize, gaussian0_sampler, sampler as samp, SamplerContext};

use crate::fft_masked::{fft, ifft, poly_add, poly_merge_fft, poly_mul_fft, poly_mulconst, poly_split_fft, poly_sub};
use crate::fpr_masked::{fpr_add, fpr_floor, fpr_half, fpr_mul, fpr_mul_const, fpr_neg_fpr, fpr_of, fpr_of_i, fpr_sqr, fpr_sub, fpr_sub_const, fpr_sub_const_fpr, fpr_trunc};

pub type SamplerZZ<const ORDER: usize> = fn(&mut SamplerContext, &[fpr; ORDER], &[fpr; ORDER]) -> i32;

pub fn sampler<const ORDER: usize>(spc: &mut SamplerContext, mu: &[fpr; ORDER], isigma: &[fpr; ORDER]) -> i32 {
    let s: [i64; ORDER] = fpr_floor(mu);
    let r: [fpr; ORDER] = fpr_sub(mu, &fpr_of::<ORDER>(&s));

    let dss: [fpr; ORDER] = fpr_half(&fpr_sqr::<ORDER>(isigma));
    let ccs: [fpr; ORDER] = fpr_mul_const(isigma, spc.sigma_min);

    loop {
        let z0: i32 = gaussian0_sampler(&mut spc.p);
        let b: i32 = prng_get_u8(&mut spc.p) as i32 & 1;
        let z: i64 = (b + ((b << 1) - 1) * z0) as i64;


        let mut x: [fpr; ORDER] = fpr_mul(&fpr_sqr::<ORDER>(&fpr_sub_const_fpr::<ORDER>(of(z), &r)), &dss);
        x = fpr_sub_const(&x, mul(of((z0 * z0) as i64), FPR_INV_2SQRSIGMA0));

        if BerExp(&mut spc.p, &x, &ccs) > 0 {
            return (s[0] + s[1] + z) as i32;
        }
    }
}


pub fn BerExp<const ORDER: usize>(p: &mut Prng, x: &[fpr; ORDER], ccs: &[fpr]) -> i32 {
    let mut s: i32 = fpr_trunc(&fpr_mul_const::<ORDER>(x, FPR_INV_LOG2)) as i32;
    let r: [fpr; ORDER] = fpr_sub_const(x, mul(of(s as i64), FPR_LOG2));

    let mut sw: u32 = s as u32;
    sw ^= (sw ^ 63) & (-((63 as u32).wrapping_sub(sw).wrapping_shr(31) as i32) as u32);
    s = sw as i32;

    let z: u64 = ((expm_p63(add(r[0], r[1]) as u64, add(ccs[0], ccs[1])) << 1).wrapping_sub(1)).wrapping_shr(s as u32);

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

#[allow(non_snake_case)]
pub fn ffSampling_fft<const ORDER: usize>(samp: SamplerZZ<ORDER>, samp_ctx: &mut SamplerContext, z0: &mut [[fpr; ORDER]],
                                          z1: &mut [[fpr; ORDER]], tree: &[[fpr; ORDER]], t0: &mut [[fpr; ORDER]], t1: &[[fpr; ORDER]], logn: u32, tmp: &mut [[fpr; ORDER]]) {
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
        let mut w2: [fpr; ORDER] = fpr_mul_const(&fpr_add::<ORDER>(&c_re, &c_im), FPR_INVSQRT8);
        let mut w3: [fpr; ORDER] = fpr_mul_const(&fpr_sub::<ORDER>(&c_im, &c_re), FPR_INVSQRT8);

        let mut x0: [fpr; ORDER] = w2;
        let mut x1: [fpr; ORDER] = w3;
        let mut sigma: [fpr; ORDER] = tree1[3];

        // w2 = fpr_of_i(samp(samp_ctx, add(x0[0], x0[1]), add(sigma[0], sigma[1])) as i64);
        // w3 = fpr_of_i(samp(samp_ctx, add(x1[0], x1[1]), add(sigma[0], sigma[1])) as i64);
        w2 = fpr_of_i(samp(samp_ctx, &x0, &sigma) as i64);
        w3 = fpr_of_i(samp(samp_ctx, &x1, &sigma) as i64);
        a_re = fpr_sub(&x0, &w2);
        a_im = fpr_sub(&x1, &w3);
        b_re = tree1[0];
        b_im = tree1[1];
        c_re = fpr_sub(&fpr_mul::<ORDER>(&a_re, &b_re), &fpr_mul::<ORDER>(&a_im, &b_im));
        c_im = fpr_add(&fpr_mul::<ORDER>(&a_re, &b_im), &fpr_mul::<ORDER>(&a_im, &b_re));
        x0 = fpr_add(&c_re, &w0);
        x1 = fpr_add(&c_im, &w1);
        sigma = tree1[2];
        // w0 = fpr_of_i(samp(samp_ctx, add(x0[0], x0[1]), add(sigma[0], sigma[1])) as i64);
        // w1 = fpr_of_i(samp(samp_ctx, add(x1[0], x1[1]), add(sigma[0], sigma[1])) as i64);
        w0 = fpr_of_i(samp(samp_ctx, &x0, &sigma) as i64);
        w1 = fpr_of_i(samp(samp_ctx, &x1, &sigma) as i64);

        a_re = w0;
        a_im = w1;
        b_re = w2;
        b_im = w3;
        c_re = fpr_mul_const(&fpr_sub::<ORDER>(&b_re, &b_im), FPR_INVSQRT2);
        c_im = fpr_mul_const(&fpr_add::<ORDER>(&b_re, &b_im), FPR_INVSQRT2);
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
        w2 = fpr_mul_const(&fpr_add::<ORDER>(&c_re, &c_im), FPR_INVSQRT8);
        w3 = fpr_mul_const(&fpr_sub::<ORDER>(&c_im, &c_re), FPR_INVSQRT8);

        x0 = w2;
        x1 = w3;
        sigma = tree0[3];
        // let y0: [fpr; ORDER] = fpr_of_i(samp(samp_ctx, add(x0[0], x0[1]), add(sigma[0], sigma[1])) as i64);
        // let y1: [fpr; ORDER] = fpr_of_i(samp(samp_ctx, add(x1[0], x1[1]), add(sigma[0], sigma[1])) as i64);
        let y0: [fpr; ORDER] = fpr_of_i(samp(samp_ctx, &x0, &sigma) as i64);
        let y1: [fpr; ORDER] = fpr_of_i(samp(samp_ctx, &x1, &sigma) as i64);
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
        // w0 = fpr_of_i(samp(samp_ctx, add(x0[0], x0[1]), add(sigma[0], sigma[1])) as i64);
        // w1 = fpr_of_i(samp(samp_ctx, add(x1[0], x1[1]), add(sigma[0], sigma[1])) as i64);
        w0 = fpr_of_i(samp(samp_ctx, &x0, &sigma) as i64);
        w1 = fpr_of_i(samp(samp_ctx, &x1, &sigma) as i64);

        a_re = w0;
        a_im = w1;
        b_re = w2;
        b_im = w3;
        c_re = fpr_mul_const(&fpr_sub::<ORDER>(&b_re, &b_im), FPR_INVSQRT2);
        c_im = fpr_mul_const(&fpr_add::<ORDER>(&b_re, &b_im), FPR_INVSQRT2);
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
        // let y0: [fpr; ORDER] = fpr_of_i(samp(samp_ctx, add(x0[0], x0[1]), add(sigma[0], sigma[1])) as i64);
        // let y1: [fpr; ORDER] = fpr_of_i(samp(samp_ctx, add(x1[0], x1[1]), add(sigma[0], sigma[1])) as i64);
        let y0: [fpr; ORDER] = fpr_of_i(samp(samp_ctx, &x0, &sigma) as i64);
        let y1: [fpr; ORDER] = fpr_of_i(samp(samp_ctx, &x1, &sigma) as i64);
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
        // z0[0] = fpr_of_i(samp(samp_ctx, add(x0[0], x0[1]), add(sigma[0], sigma[1])) as i64);
        // z0[1] = fpr_of_i(samp(samp_ctx, add(x1[0], x1[1]), add(sigma[0], sigma[1])) as i64);
        z0[0] = fpr_of_i(samp(samp_ctx, &x0, &sigma) as i64);
        z0[1] = fpr_of_i(samp(samp_ctx, &x1, &sigma) as i64);

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

pub fn do_sign_tree<const ORDER: usize, const LOGN: usize>(samp: SamplerZZ<ORDER>, samp_ctx: &mut SamplerContext, s2: &mut [i16],
                                                           expanded_key: &[[fpr; ORDER]], hm: &[u16], logn: u32, tmp: &mut [[fpr; ORDER]]) -> bool {
    let n: usize = MKN!(logn);
    let (t0, tmprest) = tmp.split_at_mut(n);
    let (t1, tmprest) = tmprest.split_at_mut(n);

    let (b00, inter) = expanded_key.split_at(n);
    let (b01, inter) = inter.split_at(n);
    let (b10, inter) = inter.split_at(n);
    let (b11, tree) = inter.split_at(n);

    for u in 0..n {
        t0[u] = fpr_of_i::<ORDER>(hm[u] as i64);
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

    let length = 1 << LOGN;
    let mut tx_r = vec![0; length];
    reconstruct_fpr::<ORDER>(tx, &mut tx_r);
    let s1tmp: &mut [i16] = bytemuck::cast_slice_mut::<fpr, i16>(&mut tx_r);
    let mut sqn: u32 = 0;
    let mut ng: u32 = 0;
    let mut t0_r = vec![0; length];
    reconstruct_fpr::<ORDER>(t0, &mut t0_r);
    for u in 0..n {
        let z: i32 = hm[u] as i32 - rint(t0_r[u]) as i32;
        sqn = sqn.wrapping_add((z * z) as u32);
        ng |= sqn;
        s1tmp[u] = z as i16;
    }
    sqn |= -((ng >> 31) as i32) as u32;

    let s2tmp: &mut [i16] = bytemuck::cast_slice_mut(&mut t0_r);
    let mut t1_r = vec![0; length];
    reconstruct_fpr::<ORDER>(t1, &mut t1_r);

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

fn reconstruct_fpr<const ORDER: usize>(hm: &[[fpr; ORDER]], res: &mut [fpr]) {
    for i in 0..res.len() {
        res[i] = add(hm[i][0], hm[i][1]);
    }
}


pub fn sign_tree_sample<const ORDER: usize, const LOGN: usize>(sig: &mut [i16], rng: &mut InnerShake256Context,
                                                        expanded_key: &[[fpr; ORDER]], hm: &[u16], logn: u32) {
    let tmp_length: usize = falcon_tmpsize_signtree!(LOGN);
    let mut ftmp = vec![[0; ORDER]; tmp_length];
    for i in 0..tmp_length {
        ftmp[i] = [0; ORDER];
    }


    loop {
        let mut spc: SamplerContext = SamplerContext { p: Prng { buf: [0; 512], ptr: 0, state: State { d: [0; 256] }, typ: 0 }, sigma_min: FPR_SIGMA_MIN[logn as usize] };
        prng_init(&mut spc.p, rng);
        // let samp: SamplerZ = samp;
        let samp: SamplerZZ<ORDER> = sampler;

        if do_sign_tree::<ORDER, LOGN>(samp, &mut spc, sig, expanded_key, hm, logn, ftmp.as_mut_slice()) {
            break;
        }
    }
}
