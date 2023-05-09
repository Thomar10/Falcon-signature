#[link(name = "ng_poly", kind = "static")]
extern "C" {
    pub fn ntrugen_poly_mp_set_small(logn: u32, d: *const u32, f: *const i8, p: u32);
    pub fn ntrugen_poly_mp_set(logn: u32, f: *const u32, p: u32);
    pub fn ntrugen_poly_mp_norm(logn: u32, f: *const u32, p: u32);
    pub fn ntrugen_poly_big_to_small(logn: u32, d: *const i8, s: *const u32, lim: i32) -> bool;
    pub fn ntrugen_poly_max_bitlength(logn: u32, f: *const u32, flen: usize) -> u32;
    pub fn ntrugen_poly_big_to_fixed(logn: u32, d: *const u64, f: *const u32, len: usize, sc: u32);
    pub fn ntrugen_poly_sub_scaled(logn: u32, F: *const u32, Flen: usize, f: *const u32, flen: usize, k: *const i32, sc: u32);
    pub fn ntrugen_poly_sub_scaled_ntt(logn: u32, F: *const u32, Flen: usize, f: *const u32, flen: usize, k: *const i32, sc: u32, tmp: *const u32);
    pub fn ntrugen_poly_sub_kfg_scaled_depth1(logn: u32, F: *const u32, G: *const u32, FGlen: usize, k: *const u32, sc: u32, f: *const i8, g: *const i8, tmp: *const u32);
    pub fn ntrugen_poly_sqnorm(logn: u32, f: *const i8) -> u32;
}
