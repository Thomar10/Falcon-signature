#[link(name = "shake", kind = "static")]

extern "C" {
    pub fn process_block(a: *const u64);
    pub fn falcon_inner_i_shake256_init(sc: *const InnerShake256Context);
    pub fn falcon_inner_i_shake256_inject(sc: *const InnerShake256Context, inn: *const u8, len: u64);
    pub fn falcon_inner_i_shake256_inject2(sc: *const InnerShake256Context, inn: *const u16, len: u64);
    pub fn falcon_inner_i_shake256_flip(sc: *const InnerShake256Context);
    pub fn falcon_inner_i_shake256_extract(sc: *const InnerShake256Context, out: *const u8, len: u64);
}

#[repr(C)]
pub union St {
    pub a: [u64; 25],
    pub dbuf: [u8; 200],
}

#[repr(C)]
pub struct InnerShake256Context {
    pub st: St,
    pub dptr: u64,
}
