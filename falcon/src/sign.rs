use crate::fft::{poly_LDLmv_fft, poly_split_fft};

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