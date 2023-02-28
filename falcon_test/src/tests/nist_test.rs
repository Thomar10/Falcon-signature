#[cfg(test)]
mod tests {
    use core::ptr::null;

    use crate::falcon_c::nist_c::{crypto_sign_keypair_func, randombytes_init_func};
    use crate::katrng::randombytes_init;
    use crate::nist::crypto_sign_keypair;

    #[test]
    fn test_crypto_sign_keypair() {

        let mut pk: [u8; 897] = [0; 897];
        let pk_c = [0u8; 897];
        let mut sk: [u8; 1281] = [0; 1281];
        let sk_c = [0u8; 1281];
        let mut entropy: [u8; 48] = [0; 48];
        let entropy_c: [u8; 48] = [0; 48];
        unsafe { randombytes_init_func(entropy_c.as_ptr(), null(), 256); }
        randombytes_init(&mut entropy);
        let res = crypto_sign_keypair(&mut pk, &mut sk, 9);
        let res_c = unsafe { crypto_sign_keypair_func(pk_c.as_ptr(), sk_c.as_ptr()) };
        assert_eq!(res, res_c == 0);
        assert_eq!(sk, sk_c);
        assert_eq!(pk, pk_c);
    }
}