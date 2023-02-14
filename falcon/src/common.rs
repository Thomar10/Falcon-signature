use crate::shake::{i_shake256_extract, InnerShake256Context};

pub fn hash_to_point_vartime(sc: &mut InnerShake256Context, x: &mut [u16], logn: u32) {
    let mut n = 1usize << logn;
    let mut index = 0;
    while n > 0 {
        // let mut buf: [u8; 2] = [0; 2];
        let mut w: u32;

        let buf = i_shake256_extract(sc, 2);
        w = ((buf[0] as u32) << 8) | buf[1] as u32;
        if w < 61445 {
            while w >= 12289 {
                w -= 12289;
            }
            x[index] = w as u16;
            index += 1;
            n -= 1;
        }
    }
}