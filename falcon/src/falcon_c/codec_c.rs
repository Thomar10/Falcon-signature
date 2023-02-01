#[link(name = "codec", kind = "static")]

extern "C" {
    pub fn falcon_inner_modq_encode(out: *(), max_out_len: usize, x: *const u16, logn: u32);
    pub fn falcon_inner_modq_decode(x: *const u16, logn: u16, inn: *const (), max_in_len: usize);
    pub fn falcon_inner_trim_i16_encode(out: *(), max_out_len: usize, x: *const i16, logn: u32, bits: u32);
    pub fn falcon_inner_trim_i16_decode(x: *i16, logn: u32, bits: u32, inn: *const (), max_in_len: usize);
    pub fn falcon_inner_trim_i8_encode(out: *(), max_out_len: usize, x: *const i8, logn: u32, bits: u32);
    pub fn falcon_inner_trim_i8_decode(x: *i8, logn: u32, bits: u32, inn: *const (), max_in_len: usize);
    pub fn falcon_inner_comp_encode(out: *(), max_out_len: usize, x: *const i16, logn: u32);
    pub fn falcon_inner_comp_decode(x: *i16, logn: u32, inn: *const (), max_in_len: usize);
}