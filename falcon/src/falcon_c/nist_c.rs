#[link(name = "nist", kind = "static")]

extern "C" {
    #[allow(dead_code)]
    pub fn randombytes_init_func(entropy_input: *const u8, personalization_string: *const u8, security_strength: i32);
    #[allow(dead_code)]
    pub fn randombytes_func(x: *const u16, xlen: u64);
    #[allow(dead_code)]
    pub fn crypto_sign_keypair(pk: *const u8, sk: *const u8);
    #[allow(dead_code)]
    pub fn crypto_sign(sm: *const u8, smlen: *const u64, m: *const u8, mlen: u64, sk: *const u8);
    #[allow(dead_code)]
    pub fn crypto_sign_open(m: *const u8, mlen: *const u64, sm: *const u8, smlen: u64, pk: *const u8);
}