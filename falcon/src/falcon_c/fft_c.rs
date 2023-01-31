#[link(name = "fft", kind = "static")]
extern "C" {
    pub fn FFT(f: *mut u64, logn: u32);
    pub fn iFFT(f: *mut u64, logn: u32);
    pub fn poly_add(a: *mut u64, b: *mut u64, logn: u32);
    pub fn poly_sub(a: *mut u64, b: *mut u64, logn: u32);
    pub fn poly_neg(a: *mut u64, logn: u32);
    pub fn poly_adj_fft(a: *mut u64, logn: u32);
    pub fn poly_mul_fft(a: *mut u64, b: *mut u64, logn: u32);
    pub fn poly_muladj_fft(a: *mut u64, b: *mut u64, logn: u32);
    pub fn poly_mulselfadj_fft(a: *mut u64, logn: u32);
    pub fn poly_mulconst(a: *mut u64, x: u64, logn: u32);
    pub fn poly_div_fft(a: *mut u64, b: *mut u64, logn: u32);
    pub fn poly_invnorm2_fft(d: *mut u64, a: *mut u64, b: *mut u64, logn: u32);
    pub fn poly_add_muladj_fft(d: *mut u64, F: *mut u64, G: *mut u64, f: *mut u64, g: *mut u64, logn: u32);
    pub fn poly_mul_autoadj_fft(a: *mut u64, b: *mut u64, logn: u32);
    pub fn poly_div_autoadj_fft(a: *mut u64, b: *mut u64, logn: u32);
    pub fn poly_LDL_fft(g00: *mut u64, g01: *mut u64, g11: *mut u64, logn: u32);
    pub fn poly_LDLmv_fft(d11: *mut u64, l10: *mut u64, g00: *mut u64, g01: *mut u64, g11: *mut u64, logn: u32);
    pub fn poly_split_fft(f0: *mut u64, f1: *mut u64, f: *mut u64, logn: u32);
    pub fn poly_merge_fft(f: *mut u64, f0: *mut u64, f1: *mut u64, logn: u32);
}