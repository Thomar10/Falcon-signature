#[cfg(test)]
mod tests {
    use rand::Rng;
    use crate::codec::{max_fg_bits, trim_i8_encode};
    use crate::falcon_c::codec_c::falcon_inner_trim_i8_encode;

    #[test]
    fn test_trim_i8_encode() {
        /*
        let mut sk: [u16; 1281] = [0; 1281];
        let max_out = sk.len();
        let sk_c: [u16; 1281] = [0; 1281];
        let mut rng = rand::thread_rng();
        let mut f: [i8; 512] = core::array::from_fn(|_| rng.gen::<i8>());
        let f_c  = f.clone();
        let size_r = trim_i8_encode(&mut sk, 0, max_out - 1, &mut f, 9, max_fg_bits[9] as u32);
        let size_c = unsafe { falcon_inner_trim_i8_encode(sk_c.as_ptr(), max_out - 1, f_c.as_ptr(), 9, max_fg_bits[9] as u32) };
        assert_eq!(size_c, size_r);
        assert_eq!(sk, sk_c); */
    }
}