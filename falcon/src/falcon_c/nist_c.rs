#[link(name = "nist", kind = "static")]
extern "C" {
    pub fn randombytes_init_func(entropy_input: *const u8, personalization_string: *const u8, security_strength: i32);
    pub fn randombytes_func(x: *const u8, xlen: u64);
    pub fn crypto_sign_keypair_func(pk: *const u8, sk: *const u8) -> i32;
    #[allow(dead_code)]
    pub fn crypto_sign(sm: *const u8, smlen: *const u64, m: *const u8, mlen: u64, sk: *const u8);
    #[allow(dead_code)]
    pub fn crypto_sign_open_func(m: *const u8, mlen: *const u64, sm: *const u8, smlen: u64, pk: *const u8);
}