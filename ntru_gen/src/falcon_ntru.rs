use crate::fxp::{fxr, fxr_add, fxr_lt, fxr_of, fxr_sqr, vect_adj_fft, vect_fft, vect_ifft, vect_invnorm_fft, vect_mul_autoadj_fft, vect_mul_realconst, vect_set};
use crate::gauss::gauss_sample_poly;
use crate::ntru::{NtruProfile, solve_ntru};
use crate::poly::{poly_is_invertible, poly_sqnorm};
use crate::prng::{NtruPrngChacha8Context, Rng};

pub fn falcon_keygen(logn: usize, f: &mut [i8], g: &mut [i8], F: &mut [i8], G: &mut [i8],
                     rng: Rng, ctx: &mut NtruPrngChacha8Context, tmp: &mut [u32]) -> bool {
    if tmp.len() < 7 {
        return false;
    }
    if logn < 2 || logn > 10 {
        return false;
    }

    //?? WHAT is this casting shit
    let tmp_len: usize = 50000;
    if tmp_len < 24 << logn {
        return false;
    }
    let mut profile: &NtruProfile;
    loop {
        match logn {
            8 => {
                profile = &FALCON_256;
                gauss_sample_poly(logn, f, &GAUSS_FALCON_256, rng, ctx);
                gauss_sample_poly(logn, g, &GAUSS_FALCON_256, rng, ctx);
            }
            9 => {
                profile = &FALCON_512;
                gauss_sample_poly(logn, f, &GAUSS_FALCON_512, rng, ctx);
                gauss_sample_poly(logn, g, &GAUSS_FALCON_512, rng, ctx);
            }
            10 => {
                profile = &FALCON_1024;
                gauss_sample_poly(logn, f, &GAUSS_FALCON_1024, rng, ctx);
                gauss_sample_poly(logn, g, &GAUSS_FALCON_1024, rng, ctx);
                return true;
            }
            _ => {
                todo!("Add 2 - 7 logn")
            }
        }

        if poly_sqnorm(logn, f) + poly_sqnorm(logn, g) >= 16823 {
            println!("sq norm");
            continue;
        }
        if !poly_is_invertible(logn, f, 2147465883, 2763744365, 248710,
        12289, 2863078533, 45, tmp) {
            println!("not inv :/");
            continue;
        }
        let n = 1 << logn;
        let rt1 = bytemuck::pod_align_to_mut::<u32, fxr>(tmp).1;
        let (rt1, inter) = rt1.split_at_mut(n);
        let (rt2, rt3) = inter.split_at_mut(n);
        vect_set(logn, rt1, f);
        vect_set(logn, rt2, g);
        vect_fft(logn, rt1);
        vect_fft(logn, rt2);
        vect_invnorm_fft(logn, rt3, rt1, rt2, 0);
        vect_adj_fft(logn, rt1);
        vect_adj_fft(logn, rt2);
        vect_mul_realconst(logn, rt1, fxr_of(12289));
        vect_mul_realconst(logn, rt2, fxr_of(12289));
        vect_mul_autoadj_fft(logn, rt1, rt3);
        vect_mul_autoadj_fft(logn, rt2, rt3);
        vect_ifft(logn, rt1);
        vect_ifft(logn, rt2);
        let mut sn: fxr = 0;
        for u in 0..n {
            sn = fxr_add(sn, fxr_add(fxr_sqr(rt1[u]), fxr_sqr(rt2[u])));
        }
        println!("{}", sn);
        println!("{:?}", rt1);
        println!("{:?}", rt2);

        if !fxr_lt(sn, 72251709809335) {
            println!("sn not lt");
            continue;
        }
        if !solve_ntru(profile, logn, f, g, tmp) {
            println!("solve_ntru");
            continue;
        }

        let tF = bytemuck::cast_slice_mut::<u32, i8>(tmp);
        let (tF, tG) = tF.split_at_mut(n);
        if F.len() > 0  {
         F.copy_from_slice(tF);
        }
        if G.len() > 0  {
            G.copy_from_slice(tG);
        }
        return true;
    }
}


pub const FALCON_256: NtruProfile = NtruProfile {
    q: 12289,
    min_logn: 2,
    max_logn: 8,
    max_bl_small: [1, 1, 2, 3, 4, 8, 14, 27, 53, 104, 207],
    max_bl_large: [1, 2, 3, 6, 11, 21, 40, 78, 155, 308],
    word_win: [1, 1, 2, 2, 2, 3, 3, 4, 5, 7],
    reduce_bits: 16,
    coeff_FG_limit: [0, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127],
    min_save_fg: [0, 0, 1, 2, 2, 2, 2, 2, 2, 3, 3],
};

pub const FALCON_512: NtruProfile = NtruProfile {
    q: 12289,
    min_logn: 9,
    max_logn: 9,
    max_bl_small: [1, 1, 2, 3, 4, 8, 14, 27, 53, 104, 207],
    max_bl_large: [1, 2, 3, 6, 11, 21, 40, 78, 155, 308],
    word_win: [1, 1, 2, 2, 2, 3, 3, 4, 5, 7],
    reduce_bits: 13,
    coeff_FG_limit: [0, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127],
    min_save_fg: [0, 0, 1, 2, 2, 2, 2, 2, 2, 3, 3],
};

pub const FALCON_1024: NtruProfile = NtruProfile {
    q: 12289,
    min_logn: 10,
    max_logn: 10,
    max_bl_small: [1, 1, 2, 3, 4, 8, 14, 27, 53, 104, 207],
    max_bl_large: [1, 2, 3, 6, 11, 21, 40, 78, 155, 308],
    word_win: [1, 1, 2, 2, 2, 3, 3, 4, 5, 7],
    reduce_bits: 11,
    coeff_FG_limit: [0, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127],
    min_save_fg: [0, 0, 1, 2, 2, 2, 2, 2, 2, 3, 3],
};

pub const GAUSS_FALCON_256: [u16; 49] = [
    24,
    1, 3, 6, 11, 22, 40, 73, 129,
    222, 371, 602, 950, 1460, 2183, 3179, 4509,
    6231, 8395, 11032, 14150, 17726, 21703, 25995, 30487,
    35048, 39540, 43832, 47809, 51385, 54503, 57140, 59304,
    61026, 62356, 63352, 64075, 64585, 64933, 65164, 65313,
    65406, 65462, 65495, 65513, 65524, 65529, 65532, 65534
];

pub const GAUSS_FALCON_512: [u16; 35] = [
    17,
    1, 4, 11, 28, 65, 146, 308, 615,
    1164, 2083, 3535, 5692, 8706, 12669, 17574, 23285,
    29542, 35993, 42250, 47961, 52866, 56829, 59843, 62000,
    63452, 64371, 64920, 65227, 65389, 65470, 65507, 65524,
    65531, 65534
];

pub const GAUSS_FALCON_1024: [u16; 25] = [
    12,
    2, 8, 28, 94, 280, 742, 1761, 3753,
    7197, 12472, 19623, 28206, 37329, 45912, 53063, 58338,
    61782, 63774, 64793, 65255, 65441, 65507, 65527, 65533
];
