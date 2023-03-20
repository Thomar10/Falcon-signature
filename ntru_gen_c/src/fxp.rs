#[link(name = "ng_fxp", kind = "static")]
extern "C" {
    pub fn ntrugen_inner_fxr_div(x: u64, y: u64) -> u64;
    pub fn ntrugen_vect_FFT(logn: u32, f: *const u64);
}
