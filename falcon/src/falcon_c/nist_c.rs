#[link(name = "nist", kind = "static")]

extern "C" {
    pub fn randombytes_init(entropy_input: *const u8, personalization_string: *const u8, security_strength: i32);
    pub fn randombytes(x: *const u8, xlen: u64);
    pub fn crypto_sign_keypair(pk: *const u8, sk: *const u8);
    pub fn crypto_sign(sm: *const u8, smlen: *const u64, m: *const u8, mlen: u64, sk: *const u8);
    pub fn crypto_sign_open(m: *const u8, mlen: *const u64, sm: *const u8, smlen: u64, pk: *const u8);
}