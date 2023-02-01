#[link(name = "shake", kind = "static")]

extern "C" {

    pub fn process_block(a: *const u64);
    pub fn i_shake256_init(sc: *const inner_shake256_context);
    pub fn i_shake256_inject(sc: *const inner_shake256_context, inn: *const u8, len: u64);
    pub fn i_shake256_flip(sc: *const inner_shake256_context);
    pub fn i_shake256_extract(sc: *const inner_shake256_context, out: *const u8, len: u64);
}

#[repr(C)]
pub union MyUnion {
    pub a: [u64; 25],
    pub dbuf: [u8; 200],
}

#[repr(C)]
pub struct inner_shake256_context {
    pub st: MyUnion,
    pub dptr: u64,
}
