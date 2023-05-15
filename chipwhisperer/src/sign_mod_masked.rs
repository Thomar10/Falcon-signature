use alloc::vec;
use stm32f4xx_hal::gpio;
use stm32f4xx_hal::gpio::{Output, PushPull};
use falcon::common::is_short_half;
use falcon::falcon::fpr;
use falcon::fpr::{FPR_INVERSE_OF_Q, FPR_SIGMA_MIN, fpr_rint as rint, fpr_add as add};
use falcon::MKN;
use falcon::rng::{Prng, prng_init, State};
use falcon::shake::InnerShake256Context;
use falcon::sign::{SamplerContext, SamplerZ, sampler as samp};
use falcon_masked::fft_masked::{fft, ifft, poly_add, poly_mul_fft, poly_mulconst};
use falcon_masked::fpr_masked::{fpr_neg_fpr, fpr_of_i};
use falcon_masked::sign_masked::ffSampling_fft;

type TriggerPin = gpio::PA12<Output<PushPull>>;

fn reconstruct_fpr<const ORDER: usize>(hm: &[[fpr; ORDER]], res: &mut [fpr]) {
    for i in 0..res.len() {
        res[i] = add(hm[i][0], hm[i][1]);
    }
}

pub fn do_sign_tree<const ORDER: usize, const LOGN: usize>(samp: SamplerZ, samp_ctx: &mut SamplerContext, s2: &mut [i16],
                                                           expanded_key: &[[fpr; ORDER]], hm: &[u16], logn: u32, tmp: &mut [[fpr; ORDER]], trigger: &mut TriggerPin) -> bool {
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

    cortex_m::interrupt::free(|_| {
        trigger.set_high();
        ffSampling_fft(samp, samp_ctx, tx, ty, tree, t0, t1, logn, tmprest);
        trigger.set_low();
    });

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

pub fn sign_tree_masked_ffSampling_test<const ORDER: usize, const LOGN: usize>(sig: &mut [i16], rng: &mut InnerShake256Context,
                                                                  expanded_key: &[[fpr; ORDER]], hm: &[u16], logn: u32, tmp: &mut [[fpr; ORDER]], trigger: &mut TriggerPin) {
    loop {
        let mut spc: SamplerContext = SamplerContext { p: Prng { buf: [0; 512], ptr: 0, state: State { d: [0; 256] }, typ: 0 }, sigma_min: FPR_SIGMA_MIN[logn as usize] };
        prng_init(&mut spc.p, rng);
        let samp: SamplerZ = samp;

        if do_sign_tree::<ORDER, LOGN>(samp, &mut spc, sig, expanded_key, hm, logn, tmp, trigger) {
            break;
        }
    }
}