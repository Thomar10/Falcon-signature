#[link(name = "vrfy", kind = "static")]
extern "C" {
    pub fn mq_add_func(x: u32, y: u32) -> u32;
    pub fn mq_sub_func(x: u32, y: u32) -> u32;
    pub fn mq_rshift1_func(x: u32) -> u32;
    pub fn mq_montymul_func(x: u32, y: u32) -> u32;
    pub fn mq_montysqr_func(x: u32) -> u32;
    pub fn mq_div_12289_func(x: u32, y: u32) -> u32;
    pub fn mq_NTT_func(a: *const u16, logn: u32);
    pub fn mq_iNTT_func(a: *const u16, logn: u32);
    pub fn mq_poly_tomonty_func(f: *const u16, logn: u32);
    pub fn mq_poly_montymul_ntt_func(f: *const u16, g: *const u16, logn: u32);
    pub fn mq_poly_sub_func(f: *const u16, g: *const u16, logn: u32);
    pub fn falcon_inner_to_ntt_monty_func(a: *const u16, logn: u32);
    pub fn falcon_inner_verify_raw_func(c0: *const u16, s2: *const i16, h: *const u16, logn: u32, tmp: *const u8) -> i32;
    pub fn falcon_inner_compute_public_func(h: *const u16, f: *const i8, g: *const i8, logn: u32, tmp: *const u8) -> i32;
    pub fn falcon_inner_complete_private_func(f: *const i32, g: *const i32, F: *const i32, logn: u32, tmp: *const u8) -> i32;
    pub fn falcon_inner_is_invertible_func(s2: *const i16, logn: u32, tmp: *const u8) -> i32;
    pub fn falcon_inner_verify_recover_func(h: *const u16, c0: *const u16, s1: *const i16, s2: *const i16, logn: u32, tmp: *const u8) -> i32;
    pub fn falcon_inner_count_nttzero_func(sig: *const i16, logn: u32, tmp: *const u8) -> i32;
}