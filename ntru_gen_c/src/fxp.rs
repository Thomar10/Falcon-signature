#[link(name = "ng_fxp", kind = "static")]
extern "C" {
    pub fn ntrugen_inner_fxr_div(x: u64, y: u64) -> u64;
    pub fn ntrugen_vect_FFT(logn: u32, f: *const u64);
    pub fn ntrugen_vect_iFFT(logn: u32, f: *const u64);
    pub fn ntrugen_vect_set(logn: u32, d: *const u64, f: *const i8);
    pub fn ntrugen_vect_add(logn: u32, a: *const u64, b: *const u64);
    pub fn ntrugen_vect_mul_realconst(logn: u32, a: *const u64, c: u64);
    pub fn ntrugen_vect_mul_fft(logn: u32, a: *const u64, b: *const u64);
    pub fn ntrugen_vect_adj_fft(logn: u32, a: *const u64);
    pub fn ntrugen_vect_mul_autoadj_fft(logn: u32, a: *const u64, b: *const u64);
    pub fn ntrugen_vect_div_autoadj_fft(logn: u32, a: *const u64, b: *const u64);
    pub fn ntrugen_vect_norm_fft(logn: u32, d: *const u64, a: *const u64, b: *const u64);
    pub fn ntrugen_vect_invnorm_fft(logn: u32, d: *const u64, a: *const u64, b: *const u64, e: u32);
}
