use crate::shake::{i_shake256_extract, InnerShake256Context};

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

const OVERTAB: [u16; 11] = [
    0,
    65,
    67,
    71,
    77,
    86,
    100,
    122,
    154,
    205,
    287
];

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

pub fn hash_to_point_ct(sc: &mut InnerShake256Context, x: &mut [u16], logn: u32, tmp: &mut [u8]) {
    let over: u32;
    let n = 1usize << logn;
    let tt1: &mut [u16] = bytemuck::pod_align_to_mut(tmp).1;

    let mut tt2: [u16; 63] = [0; 63];
    let n2 = n << 1;
    over = OVERTAB[logn as usize] as u32;
    let m = n + over as usize;

    for u in 0usize..m {
        let (w, mut wr): (u32, u32);

        let buf = i_shake256_extract(sc, 2);
        w = ((buf[0] as u32) << 8) | buf[1] as u32;
        wr = w - (24578u32 & (((w.wrapping_sub(24578)) >> 31).wrapping_sub(1)));
        wr = wr - (24578u32 & (((wr.wrapping_sub(24578)) >> 31).wrapping_sub(1)));
        wr = wr - (12289u32 & (((wr.wrapping_sub(12289)) >> 31).wrapping_sub(1)));
        wr |= ((w.wrapping_sub(61445)) >> 31).wrapping_sub(1);
        if u < n {
            x[u] = wr as u16;
        } else if u < n2 {
            tt1[u - n] = wr as u16;
        } else {
            tt2[u - n2] = wr as u16;
        }
    }

    let mut p = 1;
    while p <= over {
        let mut v: u32 = 0;
        for u in 0..m {
            let (s, d): (*mut u16, *mut u16);
            let (sv, dv, mut mk): (u32, u32, u32);

            if u < n {
                s = x.as_mut_ptr().wrapping_add(u);
            } else if u < n2 {
                s = tt1.as_mut_ptr().wrapping_add(u - n);
            } else {
                s = tt2.as_mut_ptr().wrapping_add(u - n2);
            }
            unsafe { sv = *s as u32; }

            let j: u32 = (u - v as usize) as u32;
            mk = (sv >> 15).wrapping_sub(1u32);
            v = v.wrapping_sub(mk);

            if u < p as usize {
                continue;
            }

            if (u - p as usize) < n {
                d = x.as_mut_ptr().wrapping_add(u - p as usize);
            } else if (u - p as usize) < n2 {
                d = tt1.as_mut_ptr().wrapping_add((u - p as usize) - n);
            } else {
                d = tt2.as_mut_ptr().wrapping_add((u - p as usize) - n2);
            }
            unsafe { dv = *d as u32; }

            mk &= (!(((j & p) + 0x1FF) >> 9)).wrapping_add(1);

            unsafe { *s = (sv ^ (mk & (sv ^ dv))) as u16; }
            unsafe { *d = (dv ^ (mk & (sv ^ dv))) as u16; }
        }
        p <<= 1;
    }
}

pub fn is_short(s1: &[i16], s2: &[i16], logn: u32) -> i32 {
    let n = 1usize << logn;
    let mut s: u32 = 0;
    let mut ng: u32 = 0;
    for u in 0..n {
        let mut z: i32;
        z = s1[u] as i32;
        s = s.wrapping_add((z * z) as u32);
        ng |= s;
        z = s2[u] as i32;
        s = s.wrapping_add((z * z) as u32);
        ng |= s;
    }
    s |= (!(ng >> 31)).wrapping_add(1);
    return (s <= L2BOUND[logn as usize]) as i32;
}

pub fn is_short_half(mut sqn: u32, s2: &[i16], logn: u32) -> i32 {
    let n = 1usize << logn;
    let mut ng = (!(sqn >> 31)).wrapping_add(1);
    for u in 0..n {
        let z: i32 = s2[u] as i32;
        sqn = sqn.wrapping_add((z * z) as u32);
        ng |= sqn;
    }
    sqn |= (!(ng >> 31)).wrapping_add(1);

    (sqn <= L2BOUND[logn as usize]) as i32
}
