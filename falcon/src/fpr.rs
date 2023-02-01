fn fpr_norm64(mut m: u64, mut _e: i32) {
    let mut nt: u32;
    _e -= 63;

    nt = (m >> 32) as u32;
    nt = (nt | !nt) >> 31;
    m ^= (m ^ (m << 32)) & ((nt as u64) - 1);
    _e += (nt << 5) as i32;

    nt = (m >> 48) as u32;
    nt = (nt | !nt) >> 31;
    m ^= (m ^ (m << 16)) & ((nt as u64) - 1);
    _e += (nt << 4) as i32;

    nt = (m >> 56) as u32;
    nt = (nt | !nt) >> 31;
    m ^= (m ^ (m << 8)) & ((nt as u64) - 1);
    _e += (nt << 3) as i32;

    nt = (m >> 60) as u32;
    nt = (nt | !nt) >> 31;
    m ^= (m ^ (m << 4)) & ((nt as u64) - 1);
    _e += (nt << 2) as i32;

    nt = (m >> 62) as u32;
    nt = (nt | !nt) >> 31;
    m ^= (m ^ (m << 2)) & ((nt as u64) - 1);
    _e += (nt << 1) as i32;

    nt = (m >> 63) as u32;
    nt = (nt | !nt) >> 31;
    m ^= (m ^ (m << 1)) & ((nt as u64) - 1);
    _e += nt as i32;
}

pub fn fpr_add(mut x: u64, mut y: u64) -> u64 {
    let (mut m, mut xu, mut yu, za): (u64, u64, u64, u64);
    let cs: u32;
    let (mut ex, mut ey, sx, sy, mut cc): (i32, i32, i32, i32, i32);

    m = (1 << 63) - 1;
    za = (x & m) - (y & m);
    cs = ((za >> 63) | ((1u64 - (za >> 63)) & (x >> 63))) as u32;
    println!("value of cs in rust = {}", cs);
    m = (x ^ y) & cs as u64;
    x ^= m;
    y ^= m;

    ex = (x >> 52) as i32;
    sx = ex >> 11;
    ex &= 0x7FF;
    m = ((((ex + 0x7FF) >> 11) as u64) << 52);
    xu = ((x & ((1u64 << 52) - 1)) | m) << 3;
    ex -= 1078;
    ey = (y >> 52) as i32;
    sy = ey >> 11;
    ey &= 0x7FF;
    m = ((((ey + 0x7FF) >> 11) as u64) << 52);
    yu = ((y & ((1u64 << 52) - 1)) | m) << 3;
    ey -= 1078;

    cc = ex - ey;
    yu &= ((cc - 60) >> 31) as u64;
    cc &= 63;

    m = fpr_ulsh(1, cc) - 1;
    yu |= (yu & m) + m;
    yu = fpr_ursh(yu, cc);

    xu += yu - ((yu << 1) & (sx ^ sy) as u64);

    fpr_norm64(xu, ex);

    xu |= (((xu as u32) & 0x1FF) + 0x1FF) as u64;
    xu >>= 9;
    ex += 9;
    fpr(sx, ex, xu)
}

#[inline(always)]
fn fpr_ursh(mut x: u64, n: i32) -> u64 {
    x ^= (x ^ (x >> 32)) & ((n >> 5) as u64);
    x >> (n & 31)
}

#[inline(always)]
fn fpr_ulsh(mut x: u64, n: i32) -> u64 {
    x ^= (x ^ (x << 32)) & ((n >> 5) as u64);
    x << (n & 31)
}



#[inline(always)]
fn fpr(s: i32, mut e: i32, mut m: u64) -> u64 {
    let mut x: u64;
    let mut t: u32;
    let f: u32;

    e += 1076;
    t = (e as u32) >> 31;
    println!("t value = {}", t);
    m &= (t as u64) - 1;
    t = (m >> 54) as u32;
    e &= -(t as i32);

    x = (((s as u64) << 63) | (m >> 2)) + ((e as u32 as u64) << 52);

    f = m as u32 & 7;
    x += ((0xC8u32 >> f) & 1) as u64;
    x
}