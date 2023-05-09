#![allow(non_snake_case)]

#[cfg(test)]
mod tests {
    use falcon::{falcon_privatekey_size, falcon_publickey_size, falcon_sig_ct_size, falcon_tmpsize_expanded_key_size, falcon_tmpsize_expandprivate, falcon_tmpsize_keygen, falcon_tmpsize_signdyn, falcon_tmpsize_signtree, falcon_tmpsize_verify};
    use falcon::common::hash_to_point_vartime;
    use falcon::falcon::{falcon_expand_privatekey, falcon_keygen_make, FALCON_SIG_COMPRESS, falcon_verify, fpr};
    use falcon::shake::{i_shake256_init, i_shake256_inject, InnerShake256Context};
    use falcon::sign::sign_tree as u_sign_tree;
    use falcon_masked::falcon_masked::{falcon_sign_tree_masked, falcon_sign_tree_masked_sample};
    use falcon_masked::sign_masked::sign_tree;
    use falcon_masked::sign_masked_mask_sample::sign_tree_sample;

    pub fn init_shake_with_random_context() -> InnerShake256Context {
        let mut sc_rust = InnerShake256Context { st: [0; 25], dptr: 0 };
        i_shake256_init(&mut sc_rust);
        let input_rust: [u8; 25] = rand::random();
        i_shake256_inject(&mut sc_rust, &input_rust);
        sc_rust
    }

    #[test]
    fn signature_verifies() {
        let mut rng = InnerShake256Context { st: [0; 25], dptr: 0 };
        const LOGN: usize = 10;
        const ORDER: usize = 2;
        let pk_len = falcon_publickey_size!(LOGN);
        let sk_len = falcon_privatekey_size!(LOGN);
        let tmp_len = falcon_tmpsize_keygen!(LOGN);
        let sig_len = falcon_sig_ct_size!(LOGN);
        let tmp_sig_len = falcon_tmpsize_signtree!(LOGN);
        let exp_key_len = falcon_tmpsize_expanded_key_size!(LOGN);
        let exp_tmp_len = falcon_tmpsize_expandprivate!(LOGN);
        let tmp_vrfy_len = falcon_tmpsize_verify!(LOGN);
        let mut tmp_ver: Vec<u8> = vec![0; tmp_sig_len];
        let mut pk: Vec<u8> = vec![0; pk_len];
        let mut sk: Vec<u8> = vec![0; sk_len];
        let mut signature: Vec<u8> = vec![0; sig_len];
        let mut tmp: Vec<u8> = vec![0; tmp_len];
        let mut exp_key: Vec<u8> = vec![0; exp_key_len];
        let mut tmp_exp: Vec<u8> = vec![0; exp_tmp_len];
        falcon_keygen_make(&mut rng, LOGN as u32, sk.as_mut_slice(), sk_len,
                           pk.as_mut_slice(), pk_len, tmp.as_mut_slice(), tmp_len);
        falcon_expand_privatekey(exp_key.as_mut_slice(), exp_key_len, sk.as_mut_slice(), sk_len, tmp_exp.as_mut_slice(), exp_tmp_len);
        let (res, sig_len) = falcon_sign_tree_masked_sample::<ORDER, LOGN>(&mut rng, signature.as_mut_slice(), sig_len,
                                        FALCON_SIG_COMPRESS, exp_key.as_mut_slice(),
                                        "data".as_bytes());

        assert_eq!(falcon_verify(signature.as_mut_slice(), sig_len, FALCON_SIG_COMPRESS, pk.as_mut_slice(),
                                 pk_len, "data".as_bytes(), tmp_ver.as_mut_slice(), tmp_vrfy_len), 0);
    }

    fn mask_expanded_key<const ORDER: usize>(key: &mut [u8]) -> Vec<[fpr; ORDER]> {
        let (_, expkey) = key.split_at(8); // alignment
        let expkey = bytemuck::cast_slice(expkey);
        let mut mkey: Vec<[fpr; ORDER]> = vec!([0; ORDER]; expkey.len());
        for i in 0..expkey.len() {
            let mut mask: [fpr; ORDER] = [0; ORDER];
            mask[0] = expkey[i];
            mkey[i] = mask;
        }
        mkey
    }
}