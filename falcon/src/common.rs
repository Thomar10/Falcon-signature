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

pub fn is_short(s1: *mut i16, s2: &mut [i16], logn: u32) -> bool {
    let n = 1usize << logn;
    let mut s:u32 = 0;
    let mut ng: u32 = 0;
    let mut s1: *mut i16 = s1;
    for u in 0..n {
        let mut z: i32;
        unsafe { z = *s1 as i32; }
        s = s.wrapping_add((z * z) as u32);
        ng |= s;
        z = s2[u] as i32;
        s = s.wrapping_add((z * z) as u32);
        ng |= s;
        s1 = s1.wrapping_add(1);
    }
    s |= (!(ng >> 31)).wrapping_add(1);
    return s <= L2BOUND[logn as usize];
}

const L2BOUND: [u32; 11] = [
    0,
    101498,
    208714,
    428865,
    892039,
    1852696,
    3842630,
    7959734,
    16468416,
    34034726,
    70265242
];