#![allow(non_snake_case)]

#[link(name = "ng_ntru", kind = "static")]
extern "C" {
    pub fn make_fg_step_test(profile: &NtruProfileC, logn_top: u32, depth: u32, tmp: *const u32);
    pub fn make_fg_intermediate_test(profile: &NtruProfileC, logn_top: u32, f: *const i8, g: *const i8, depth: u32, tmp: *const u32);
    pub fn make_fg_deepest_test(profile: &NtruProfileC, logn_top: u32, f: *const i8, g: *const i8, tmp: *const u32, sav_off: usize);
    pub fn solve_NTRU_depth0_test(profile: &NtruProfileC, logn: u32, f: *const i8, g: *const i8, tmp: *const u32) -> i32;
    pub fn ntrugen_solve_ntru(profile: &NtruProfileC, logn: u32, f: *const i8, g: *const i8, tmp: *const u32) -> i32;
}

#[repr(C)]
pub struct NtruProfileC {
    pub q: u32,
    pub min_logn: u32,
    pub max_logn: u32,
    pub max_bl_small: [u16; 11],
    pub max_bl_large: [u16; 10],
    pub word_win: [u16; 10],
    pub reduce_bits: u32,
    pub coeff_FG_limit: [u8; 11],
    pub min_save_fg: [u16; 11],
}