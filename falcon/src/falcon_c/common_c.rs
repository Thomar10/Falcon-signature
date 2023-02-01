use crate::falcon_c::shake_c::inner_shake256_context;

#[link(name = "common", kind = "static")]

extern "C" {
    #[allow(dead_code)]
    pub fn hash_to_point_vartime(sc: *const inner_shake256_context, x: *const u16, logn: u32);
    #[allow(dead_code)]
    pub fn hash_to_point_ct(sc: *const inner_shake256_context, x: *const u16, logn: u32, tmp: *mut u8);
    #[allow(dead_code)]
    pub fn is_short(s1: *const i16, s2: *const i16, logn: u32);
    #[allow(dead_code)]
    pub fn is_short_half(sqn: u32, s2: *const i16, logn: u32);
}