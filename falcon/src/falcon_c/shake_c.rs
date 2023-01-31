#[link(name = "shake", kind = "static")]

extern "C" {
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
