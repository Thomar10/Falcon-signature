#![allow(non_snake_case)]

#[cfg(test)]
mod tests {
    use falcon::{falcon_privatekey_size, falcon_publickey_size, falcon_tmpsize_expanded_key_size, falcon_tmpsize_expandprivate, falcon_tmpsize_keygen, falcon_tmpsize_signtree};
    use falcon::common::hash_to_point_vartime;
    use falcon::falcon::{falcon_expand_privatekey, falcon_keygen_make, fpr};
    use falcon::shake::{i_shake256_init, i_shake256_inject, InnerShake256Context};
    use falcon::sign::sign_tree as u_sign_tree;
    use falcon_masked::sign_masked::sign_tree;

    pub fn init_shake_with_random_context() -> InnerShake256Context {
        let mut sc_rust = InnerShake256Context { st: [0; 25], dptr: 0 };
        i_shake256_init(&mut sc_rust);
        let input_rust: [u8; 25] = rand::random();
        i_shake256_inject(&mut sc_rust, &input_rust);
        sc_rust
    }

    #[test]
    fn test_sign_tree() {
        const LOGN: usize = 2;
        const LENGTH: usize = 1 << LOGN;
        const ORDER: usize = 2;
        let mut rng = init_shake_with_random_context();

        let pk_len = falcon_publickey_size!(LOGN);
        let sk_len = falcon_privatekey_size!(LOGN);
        let tmp_len = falcon_tmpsize_keygen!(LOGN);
        let exp_key_len = falcon_tmpsize_expanded_key_size!(LOGN);
        let exp_tmp_len = falcon_tmpsize_expandprivate!(LOGN);
        const TMP_SIG_LEN: usize = falcon_tmpsize_signtree!(LOGN);
        let mut pk: Vec<u8> = vec![0; pk_len];
        let mut sk: Vec<u8> = vec![0; sk_len];
        let mut tmp: Vec<u8> = vec![0; tmp_len];
        let mut exp_key: Vec<u8> = vec![0; exp_key_len];
        let mut tmp_exp: Vec<u8> = vec![0; exp_tmp_len];
        let mut tmp_sigsig: Vec<u8> = vec![0; TMP_SIG_LEN];
        let mut tmp_sig: Vec<i16> = vec![0; TMP_SIG_LEN];
        let mut tmp_sig_mask: Vec<i16> = vec![0; TMP_SIG_LEN];
        falcon_keygen_make(&mut rng, LOGN as u32, sk.as_mut_slice(), sk_len,
                           pk.as_mut_slice(), pk_len, tmp.as_mut_slice(), tmp_len);
        falcon_expand_privatekey(exp_key.as_mut_slice(), exp_key_len, sk.as_mut_slice(), sk_len, tmp_exp.as_mut_slice(), exp_tmp_len);

        let masked_expand = mask_expanded_key::<ORDER>(exp_key.as_mut_slice());

        let mut hm: Vec<u16> = vec![0; LENGTH];
        hash_to_point_vartime(&mut rng, hm.as_mut_slice(), LOGN as u32);
        let masked_hashed_message = mask_hashed_message::<ORDER, LENGTH>(hm.as_mut_slice());

        for _ in 0..10 {
            let mut u_rng = InnerShake256Context { st: rng.st, dptr: rng.dptr };
            sign_tree::<ORDER, TMP_SIG_LEN, LENGTH>(tmp_sig_mask.as_mut_slice(), &mut rng, &masked_expand, &masked_hashed_message, LOGN as u32);
            let (_, expkey) = exp_key.split_at(8); // alignment
            let expkey = bytemuck::cast_slice(expkey);
            u_sign_tree(tmp_sig.as_mut_slice(), &mut u_rng, expkey, &hm, LOGN as u32, bytemuck::cast_slice_mut(bytemuck::pod_align_to_mut::<u8, fpr>(tmp_sigsig.as_mut_slice()).1));
            assert_eq!(tmp_sig_mask.as_slice(), tmp_sig.as_slice());

        }
    }



    fn mask_hashed_message<const ORDER: usize, const LENGTH: usize>(hm: &mut [u16]) -> [[u16; ORDER]; LENGTH] {
        let mut mhm: [[u16; ORDER]; LENGTH] = [[0; ORDER]; LENGTH];
        for i in 0..LENGTH {
            let mut mask: [u16; ORDER] = [0; ORDER];
            mask[0] = hm[i];
            mhm[i] = mask;
        }
        mhm
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