use crate::fft::{fft, poly_add, poly_LDLmv_fft, poly_muladj_fft, poly_mulselfadj_fft, poly_neg, poly_split_fft};
use crate::fpr::{fpr_mul, fpr_sqrt, FPR_INV_SIGMA, fpr_of};

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
pub fn ffLDL_fft(tree: &mut [fpr], g00: &mut [fpr], g01: &mut [fpr], g11: &mut [fpr], logn: u32, tmp: &mut [fpr]) -> () {
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
pub fn expand_privkey(expanded_key: &mut [fpr], f: &[i8], g: &[i8], F: &[i8], G: &[i8], logn: u32, tmp: &mut [u64]) {
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