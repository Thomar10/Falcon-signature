const L2BOUND: [u32; 11] = [
    0,    /* unused */
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

pub fn is_short_half(mut sqn: u32, s2: &[i16], logn: u32) -> bool {
    let n: usize = 1 << logn;
    let mut ng: u32 = -((sqn >> 31) as i32) as u32;

    for u in 0..n {
        let z: i32 = s2[u] as i32;
        sqn += (z * z) as u32;
        ng |= sqn;
    }
    sqn |= -((ng >> 31) as i32) as u32;

    return sqn <= L2BOUND[logn as usize];
}