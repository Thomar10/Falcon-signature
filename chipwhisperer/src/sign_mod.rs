use stm32f4xx_hal::gpio;
use stm32f4xx_hal::gpio::{Output, PushPull};
use falcon::common::is_short_half;
use falcon::falcon::fpr;
use falcon::fft::{fft, ifft, poly_add, poly_mul_fft, poly_mulconst};
use falcon::fpr::{FPR_INVERSE_OF_Q, fpr_neg, fpr_of, fpr_rint, FPR_SIGMA_MIN};
use falcon::MKN;
use falcon::rng::{Prng, prng_init, State};
use falcon::shake::InnerShake256Context;
use falcon::sign::{ffSampling_fft, sampler, SamplerContext, SamplerZ};
type TriggerPin = gpio::PA12<Output<PushPull>>;

pub fn do_sign_tree(samp: SamplerZ, samp_ctx: &mut SamplerContext, s2: &mut [i16],
                    expanded_key: &[fpr], hm: &[u16], logn: u32, tmp: &mut [fpr], trigger: &mut TriggerPin) -> bool {
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

    let s1tmp: &mut [i16] = bytemuck::cast_slice_mut(tx);
    let mut sqn: u32 = 0;
    let mut ng: u32 = 0;

    for u in 0..n {
        let z: i32 = hm[u] as i32 - fpr_rint(t0[u]) as i32;

        sqn = sqn.wrapping_add((z * z) as u32);
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

#[allow(non_snake_case)]
pub fn sign_tree_ffSampling_test(sig: &mut [i16], rng: &mut InnerShake256Context, expanded_key: &[fpr], hm: &[u16],
                 logn: u32, tmp: &mut [u8], trigger: &mut TriggerPin) {

    let ftmp: &mut [fpr] = bytemuck::cast_slice_mut(tmp);

    loop {
        let mut spc: SamplerContext = SamplerContext {p: Prng {buf: [0; 512], ptr: 0, state: State {d: [0; 256]}, typ: 0}, sigma_min: FPR_SIGMA_MIN[logn as usize]};
        prng_init(&mut spc.p, rng);
        let samp: SamplerZ = sampler;

        if do_sign_tree(samp, &mut spc, sig, expanded_key, hm, logn, ftmp, trigger) {
            break;
        }
    }
}