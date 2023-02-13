#[cfg(test)]
mod tests {
    use crate::falcon_c::nist_c::crypto_sign_keypair_func;
    use crate::nist::crypto_sign_keypair;

    #[test]
    fn test_crypto_sign_keypair() {
        let mut pk: [u16; 897] = [0; 897];
        let pk_c = [0; 897];
        let mut sk: [u16; 1281] = [0; 1281];
        let sk_c = [0; 1281];
        let res = crypto_sign_keypair(&mut pk, &mut sk);
        let res_c = unsafe { crypto_sign_keypair_func(pk_c.as_ptr(), sk_c.as_ptr()) };
        //assert_eq!(res, res_c != 0);
        assert_eq!(sk, sk_c);
        //assert_eq!(pk, pk_c);
    }
}