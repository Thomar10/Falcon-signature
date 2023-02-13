#[link(name = "katrng2", kind = "static")]
extern "C" {
    pub fn randombytes_init2(entropy_input: *const u8, personalization_string: *const u8, security_strength: i32);
    pub fn randombytes2(x: *const u8, xlen: u64) -> i32;
}