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

#[allow(non_upper_case_globals)]
pub const max_sig_bits: [u8; 11] = [
    0, /* unused */
    10,
    11,
    11,
    12,
    12,
    12,
    12,
    12,
    12,
    12
];

pub fn modq_encode(out: &mut [u8], out_index: usize, max_out: usize, x: &[u16], logn: u32) -> usize {
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

pub fn modq_decode(x: &mut [u16], logn: u32, inn: &[u8], start_in: usize, max_in: usize) -> usize {
    let n = 1usize << logn;
    let in_len: usize = ((n * 14) + 7) >> 3;
    if in_len > max_in {
        return 0;
    }
    let mut u = 0;
    let mut acc: u32 = 0;
    let mut acc_len: i32 = 0;
    let mut in_index = start_in;
    while u < n {
        acc = (acc << 8) | inn[in_index] as u32;
        in_index += 1;
        acc_len += 8;
        if acc_len >= 14 {
            let w: u32;
            acc_len -= 14;
            w = (acc >> acc_len) & 0x3FFF;
            if w >= 12289 {
                return 0;
            }
            x[u] = w as u16;
            u += 1;
        }
    }
    if acc & ((1u32 << acc_len) - 1) != 0 {
        return 0;
    }
    in_len
}

pub fn comp_encode(out: &mut [u8], out_index: usize, max_out: usize, x: &[i16], logn: usize) -> usize {
    let n = 1usize << logn;

    for u in 0..n {
        if x[u] < -2047 || x[u] > 2047 {
            return 0;
        }
    }

    let mut acc: u32 = 0;
    let mut acc_len: u32 = 0;
    let mut v: usize = 0;
    for u in 0..n {
        acc <<= 1;
        let mut t: i32 = x[u] as i32;
        if t < 0 {
            t = -t;
            acc |= 1;
        }
        let mut w: u32 = t as u32;

        acc <<= 7;
        acc |= w & 127;
        w >>= 7;

        acc_len += 8;
        acc <<= w + 1;
        acc |= 1;
        acc_len += w + 1;

        while acc_len >= 8 {
            acc_len -= 8;
            if out.len() > 0 {
                if v >= max_out {
                    return 0;
                }
                out[v + out_index] = (acc >> acc_len) as u8;
            }
            v += 1;
        }
    }
    if acc_len > 0 {
        if out.len() > 0 {
            if v >= max_out {
                return 0;
            }
            out[v + out_index] = (acc << (8 - acc_len)) as u8;
        }
        v += 1;
    }

    v
}

pub fn comp_decode(x: &mut [i16], logn: u32, inn: &[u8], inn_index: usize, max_in: usize) -> usize {
    let n = 1usize << logn;
    let mut acc: u32 = 0;
    let mut acc_len: i32 = 0;
    let mut v: usize = 0;
    for u in 0..n {
        let (b, s, mut m): (u32, u32, u32);

        if v >= max_in {
            return 0;
        }

        acc = (acc << 8) | inn[v + inn_index] as u32;
        v += 1;
        b = acc >> acc_len;
        s = b & 128;
        m = b & 127;
        loop {
            if acc_len == 0 {
                if v >= max_in {
                    return 0;
                }
                acc = (acc << 8) | inn[v + inn_index] as u32;
                v += 1;
                acc_len = 8;
            }
            acc_len -= 1;
            if ((acc >> acc_len) & 1) != 0 {
                break;
            }
            m += 128;
            if m > 2047 {
                return 0;
            }
        }

        if s != 0 && m == 0 {
            return 0;
        }

        x[u] = (if s != 0 { -(m as i32) } else { m as i32 }) as i16;
    }

    if (acc & ((1 << acc_len) - 1)) != 0 {
        return 0;
    }
    v
}

pub fn trim_i8_encode(out: &mut [u8], out_index: usize, max_out: usize, x: &[i8], logn: u32, bits: u32) -> usize {
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

pub fn trim_i8_decode(x: &mut [i8], logn: u32, bits: u32, inn: &[u8], mut inn_index: usize, max_in: usize) -> usize {
    let n = 1usize << logn;
    let in_len: usize = ((n * bits as usize) + 7) >> 3;
    if in_len > max_in {
        return 0;
    }
    let mut u: usize = 0;
    let mut acc: u32 = 0;
    let mut acc_len: u32 = 0;
    let mask1 = (1u32 << bits) - 1;
    let mask2 = 1u32 << (bits - 1);
    let neg_mask2 = (!mask2).wrapping_add(1);
    while u < n {
        acc = (acc << 8) | inn[inn_index] as u32;
        inn_index += 1;
        acc_len += 8;
        while acc_len >= bits && u < n {
            acc_len -= bits;
            let mut w = (acc >> acc_len) & mask1;
            w |= (!(w & mask2)).wrapping_add(1);
            if w == neg_mask2 {
                return 0;
            }
            x[u] = w as i8;
            u += 1;
        }
    }
    if (acc & ((1u32 << acc_len) - 1)) != 0 {
        return 0;
    }
    in_len
}

pub fn trim_i16_decode(x: &mut [i16], logn: u32, bits: u32, inn: &[u8], mut inn_index: usize, max_in: usize) -> usize {
    let n = 1usize << logn;
    let in_len: usize = ((n * bits as usize) + 7) >> 3;
    if in_len > max_in {
        return 0;
    }
    let mut u: usize = 0;
    let mut acc: u32 = 0;
    let mut acc_len: u32 = 0;
    let mask1 = (1u32 << bits) - 1;
    let mask2 = 1u32 << (bits - 1);
    let neg_mask2 = (!mask2).wrapping_add(1);
    while u < n {
        acc = (acc << 8) | inn[inn_index] as u32;
        inn_index += 1;
        acc_len += 8;
        while acc_len >= bits && u < n {
            acc_len -= bits;
            let mut w = (acc >> acc_len) & mask1;
            w |= (!(w & mask2)).wrapping_add(1);
            if w == neg_mask2 {
                return 0;
            }
            w |= (!(w & mask2)).wrapping_add(1);
            x[u] = w as i32 as i16;
            u += 1;
        }
    }
    if (acc & ((1u32 << acc_len) - 1)) != 0 {
        return 0;
    }
    in_len
}

pub fn trim_i16_encode(out: &mut [u8], out_index: usize, max_out: usize, x: &[i16], logn: u32, bits: u32) -> usize {
    let n = 1usize << logn;
    let maxv: i32 = ((1 << (bits - 1)) as i32).wrapping_sub(1);
    let minv: i32 = -maxv;
    for u in 0..n {
        if x[u] < minv as i16 || x[u] > maxv as i16 {
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
        acc = (acc << bits) | (x[u] as u16 as u32 & mask);
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