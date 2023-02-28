#![allow(dead_code)]

use crate::shake_c::InnerShake256Context;

#[link(name = "common", kind = "static")]

extern "C" {
    pub fn hash_to_point_vartime_func(sc: *const InnerShake256Context, x: *const u16, logn: u32);
    pub fn hash_to_point_ct_func(sc: *const InnerShake256Context, x: *const u16, logn: u32, tmp: *const u8);
    pub fn is_short_func(s1: *const i16, s2: *const i16, logn: u32) -> i32;
    pub fn is_short_half_func(sqn: u32, s2: *const i16, logn: u32) -> i32;
}