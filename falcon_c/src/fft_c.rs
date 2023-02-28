#![allow(dead_code)]
#[allow(non_snake_case)]
#[link(name = "fft", kind = "static")]
extern "C" {
    pub fn falcon_inner_FFT(f: *const u64, logn: u32);
    pub fn falcon_inner_iFFT(f: *const u64, logn: u32);
    pub fn falcon_inner_poly_add(a: *const u64, b: *const u64, logn: u32);
    pub fn falcon_inner_poly_sub(a: *const u64, b: *const u64, logn: u32);
    pub fn falcon_inner_poly_neg(a: *const u64, logn: u32);
    pub fn falcon_inner_poly_adj_fft(a: *const u64, logn: u32);
    pub fn falcon_inner_poly_mul_fft(a: *const u64, b: *const u64, logn: u32);
    pub fn falcon_inner_poly_muladj_fft(a: *const u64, b: *const u64, logn: u32);
    pub fn falcon_inner_poly_mulselfadj_fft(a: *const u64, logn: u32);
    pub fn falcon_inner_poly_mulconst(a: *const u64, x: u64, logn: u32);
    pub fn falcon_inner_poly_div_fft(a: *const u64, b: *const u64, logn: u32);
    pub fn falcon_inner_poly_invnorm2_fft(d: *const u64, a: *const u64, b: *const u64, logn: u32);
    pub fn falcon_inner_poly_add_muladj_fft(d: *const u64, F: *const u64, G: *const u64, f: *const u64, g: *const u64, logn: u32);
    pub fn falcon_inner_poly_mul_autoadj_fft(a: *const u64, b: *const u64, logn: u32);
    pub fn falcon_inner_poly_div_autoadj_fft(a: *const u64, b: *const u64, logn: u32);
    pub fn falcon_inner_poly_LDL_fft(g00: *const u64, g01: *const u64, g11: *const u64, logn: u32);
    pub fn falcon_inner_poly_LDLmv_fft(d11: *const u64, l10: *const u64, g00: *const u64, g01: *const u64, g11: *const u64, logn: u32);
    pub fn falcon_inner_poly_split_fft(f0: *const u64, f1: *const u64, f: *const u64, logn: u32);
    pub fn falcon_inner_poly_merge_fft(f: *const u64, f0: *const u64, f1: *const u64, logn: u32);
}