#[link(name = "vrfy", kind = "static")]
extern "C" {

    pub fn mq_add_func(x: u32, y: u32) -> u32;
    pub fn mq_sub_func(x: u32, y: u32) -> u32;
    #[allow(dead_code)]
    pub fn mq_rshift1_func(x: u32) -> u32;
    pub fn mq_montymul_func(x: u32, y: u32) -> u32;
    #[allow(dead_code)]
    pub fn mq_montysqr(x: u32) -> u32;
    #[allow(dead_code)]
    pub fn mq_div_12289(x: u32, y: u32) -> u32;
    #[allow(dead_code)]
    pub fn mq_NTT(a: *const u16, logn: u32);
    #[allow(dead_code)]
    pub fn mq_iNTT(a: *const u16, logn: u32);
    #[allow(dead_code)]
    pub fn mq_poly_tomonty(f: *const u16, logn: u32);
    #[allow(dead_code)]
    pub fn mq_poly_montymul_ntt(f: *const u16, g: *const u16, logn: u32);
    #[allow(dead_code)]
    pub fn mq_poly_sub(f: *const u16, g: *const u16, logn: u32);
    #[allow(dead_code)]
    pub fn falcon_inner_to_ntt_monty(a: *const u16, logn: u32);
    #[allow(dead_code)]
    pub fn falcon_inner_verify_raw(c0: *const u16, s2: *const i16, h: *const u16, logn: u32, tmp: *const u8);
    #[allow(dead_code)]
    pub fn falcon_inner_compute_public(f: *const i32, g: *const i32, logn: u32, tmp: *const u8);
    #[allow(dead_code)]
    pub fn falcon_inner_complete_private(f: *const i32, g: *const i32, F: *const i32, logn: u32, tmp: *const u8);
    #[allow(dead_code)]
    pub fn falcon_inner_is_invertible(s2: *const i16, logn: u32, tmp: *const u8);
    #[allow(dead_code)]
    pub fn falcon_inner_verify_recover(h: *const u16, c0: *const u16, s1: *const i16, s2: *const i16, logn: u32, tmp: *const u8);
    #[allow(dead_code)]
    pub fn falcon_inner_count_nttzero(sig: *const i16, logn: u32, tmp: *const u8);
}