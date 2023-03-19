#[link(name = "ng_zint31", kind = "static")]
extern "C" {
    pub fn ntrugen_zint_mul_small(m: *const u32, len: usize, x: u32) -> u32;
    pub fn ntrugen_zint_mod_small_unsigned(d: *const u32, len: usize, stride: usize, p: u32, p0i: u32, r2: u32) -> u32;
    pub fn ntrugen_zint_add_mul_small(x: *const u32, len: usize, xstride: usize, y: *const u32, s: u32);
    pub fn ntrugen_zint_norm_zero(x: *const u32, len: usize, xstride: usize, p: *const u32);
    pub fn ntrugen_rebuild_CRT(xx: *const u32, xlen: usize, n: usize, num_sets: usize, normalize_signed: i32, tmp: *const u32);
    pub fn ntrugen_zint_negate(a: *const u32, len: usize, ctl: u32);
    pub fn ntrugen_zint_bezout(u: *const u32, v: *const u32, x: *const u32, y: *const u32, len: usize, tmp: *const u32) -> bool;
    pub fn ntrugen_zint_add_scaled_mul_small(x: *const u32, xlen: usize, y: *const u32, ylen: usize, stride: usize, k: i32, sch: u32, scl: u32);
    pub fn ntrugen_zint_sub_scaled(x: *const u32, xlen: usize, y: *const u32, ylen: usize, stride: usize, sch: u32, scl: u32);
}
