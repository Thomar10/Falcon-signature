#[allow(non_upper_case_globals)]
pub const max_fg_bits: [u8; 11] = [
    0,
    8,
    8,
    8,
    8,
    8,
    7,
    7,
    6,
    6,
    5
];

#[allow(non_upper_case_globals)]
pub const max_FG_bits: [u8; 11] = [
    0,
    8,
    8,
    8,
    8,
    8,
    8,
    8,
    8,
    8,
    8
];

pub fn modq_encode(out: &mut [u8], out_index: usize, max_out: usize, x: &mut [u16], logn: u32) -> usize {
    let mut acc: u32;
    let mut acc_len: i32;
    let n = 1usize << logn;
    for u in 0..n {
        if x[u] >= 12289 {
            return 0;
        }
    }
    let out_len = ((n * 14) + 7) >> 3;
    if out.len() <= 0 {
        return out_len;
    }
    if out_len > max_out {
        return 0;
    }
    acc = 0;
    acc_len = 0;
    let mut index = out_index;
    for u in 0..n {
        acc = (acc << 14) | x[u] as u32;
        acc_len += 14;
        while acc_len >= 8 {
            acc_len -= 8;
            out[index] = (acc >> acc_len) as u8;
            index += 1;
        }
    }
    if acc_len > 0 {
        out[index] = (acc << (8 - acc_len)) as u8;
    }

    out_len
}

pub fn trim_i8_encode(out: &mut [u8], out_index: usize, max_out: usize, x: &mut [i8], logn: u32, bits: u32) -> usize {
    let n = 1usize << logn;
    let maxv: i8 = ((1 << (bits - 1)) as i32).wrapping_sub(1) as i8;
    let minv: i8 = -maxv;
    for u in 0..n {
        if x[u] < minv || x[u] > maxv {
            return 0;
        }
    }
    let out_len = ((n * bits as usize) + 7) >> 3;
    if out.len() <= 0 {
        return out_len;
    }
    if out_len > max_out {
        return 0;
    }
    let mut index = out_index;
    let mut acc: u32 = 0;
    let mut acc_len = 0;
    let mask = (1u32 << bits) - 1;
    for u in 0..n {
        acc = (acc << bits) | (x[u] as u8 as u32 & mask);
        acc_len += bits;
        while acc_len >= 8 {
            acc_len -= 8;
            out[index] = (acc >> acc_len) as u8;
            index += 1;
        }
    }
    if acc_len > 0 {
        out[index] = (acc << (8 - acc_len)) as u8;
    }
    out_len
}