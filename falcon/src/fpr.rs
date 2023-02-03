static FPR_LOG2: u64 = 4604418534313441775;
static FPR_INV_LOG2: u64 = 4609176140021203710;
static FPR_BNORM_MAX: u64 = 4670353323383631276;
static FPR_ZERO: u64 = 0;
static FPR_ONE: u64 = 4607182418800017408;
static FPR_TWO: u64 = 4611686018427387904;
static FPR_ONEHALF: u64 = 4602678819172646912;
static FPR_INVSQRT2: u64 = 4604544271217802189;
static FPR_INVSQRT8: u64 = 4600040671590431693;
static FPR_PTWO31: u64 = 4746794007248502784;
static FPR_PTWO31M1: u64 = 4746794007244308480;
static FPR_MTWO31M1: u64 = 13970166044099084288;
static FPR_PTWO63M1: u64 = 4890909195324358656;
static FPR_MTWO63M1: u64 = 14114281232179134464;
static FPR_PTWO63: u64 = 4890909195324358656;

static C: [u64; 13] = [
    0x00000004741183A3,
    0x00000036548CFC06,
    0x0000024FDCBF140A,
    0x0000171D939DE045,
    0x0000D00CF58F6F84,
    0x000680681CF796E3,
    0x002D82D8305B0FEA,
    0x011111110E066FD0,
    0x0555555555070F00,
    0x155555555581FF00,
    0x400000000002B400,
    0x7FFFFFFFFFFF4800,
    0x8000000000000000
];

fn fpr_norm64(mut m: u64, mut e: i32) -> (u64, i32) {
    let mut nt: u32;
    e -= 63;

    nt = (m >> 32) as u32;
    nt = (nt | (!nt + 1)) >> 31;
    m ^= (m ^ (m << 32)) & ((nt as u64) - 1);
    e += (nt << 5) as i32;

    nt = (m >> 48) as u32;
    nt = (nt | (!nt + 1)) >> 31;
    m ^= (m ^ (m << 16)) & ((nt as u64) - 1);
    e += (nt << 4) as i32;

    nt = (m >> 56) as u32;
    nt = (nt | (!nt + 1)) >> 31;
    m ^= (m ^ (m << 8)) & ((nt as u64) - 1);
    e += (nt << 3) as i32;

    nt = (m >> 60) as u32;
    nt = (nt | (!nt + 1)) >> 31;
    m ^= (m ^ (m << 4)) & ((nt as u64) - 1);
    e += (nt << 2) as i32;

    nt = (m >> 62) as u32;
    nt = (nt | (!nt + 1)) >> 31;
    m ^= (m ^ (m << 2)) & ((nt as u64) - 1);
    e += (nt << 1) as i32;

    nt = (m >> 63) as u32;
    nt = (nt | (!nt + 1)) >> 31;
    m ^= (m ^ (m << 1)) & ((nt as u64) - 1);
    e += nt as i32;
    (m, e)
}

pub fn fpr_add(mut x: u64, mut y: u64) -> u64 {
    let (mut m, mut xu, mut yu, za): (u64, u64, u64, u64);
    let cs: u32;
    let (mut ex, mut ey, sx, sy, mut cc): (i32, i32, i32, i32, i32);

    m = (1 << 63) - 1;
    za = (x & m) - (y & m);
    cs = (za >> 63) as u32 | ((1u32 - ((!za + 1) >> 63) as u32) & (x >> 63) as u32);
    m = (x ^ y) & !(cs as u64) + 1;
    x ^= m;
    y ^= m;

    ex = (x >> 52) as i32;
    sx = ex >> 11;
    ex &= 0x7FF;
    m = (((ex + 0x7FF) >> 11) as u64) << 52;
    xu = ((x & ((1u64 << 52) - 1)) | m) << 3;
    ex -= 1078;
    ey = (y >> 52) as i32;
    sy = ey >> 11;
    ey &= 0x7FF;
    m = (((ey + 0x7FF) >> 11) as u64) << 52;
    yu = ((y & ((1u64 << 52) - 1)) | m) << 3;
    ey -= 1078;

    cc = ex - ey;
    yu &= !((((cc - 60) as u32) >> 31) as u64) + 1;
    cc &= 63;

    m = fpr_ulsh(1, cc) - 1;
    yu |= (yu & m) + m;
    yu = fpr_ursh(yu, cc);

    xu += yu - ((yu << 1) & !((sx ^ sy) as u64) + 1);

    (xu, ex) = fpr_norm64(xu, ex);

    xu |= (((xu as u32) & 0x1FF) + 0x1FF) as u64;
    xu >>= 9;
    ex += 9;
    fpr(sx, ex, xu)
}

pub fn fpr_scaled(mut i: i64, sc: i32) -> u64 {
    let (s, mut e): (i32, i32);
    let t: u32;
    let mut m: u64;

    s = ((i as u64) >> 63) as i32;
    i ^= -(s as i64);
    i += s as i64;

    m = i as u64;
    e = 9 + sc;
    (m, e) = fpr_norm64(m, e);

    m |= (((m as u32) & 0x1FF) + 0x1FF) as u64;
    m >>= 9;

    t = (((i | -i) as u64) >> 63) as u32;
    m &= !(t as u64) + 1;
    e &= -(t as i32);

    fpr(s, e, m)
}

pub fn fpr_expm_p63(x: u64, ccs: u64) -> u64 {
    let (mut z, mut y): (u64, u64);
    let (mut z0, mut z1, mut y0, mut y1): (u32, u32, u32, u32);
    let (mut a, mut b): (u64, u64);

    y = C[0];
    z = (fpr_trunc(fpr_mul(x, FPR_PTWO63)) << 1) as u64;
    let mut u = 1;
    while u < 13 {
        let mut c: u64;

        z0 = z as u32;
        z1 = (z >> 32) as u32;
        y0 = y as u32;
        y1 = (y >> 32) as u32;
        a = ((z0 as u64) * (y1 as u64))
            + (((z0 as u64) * (y0 as u64)) >> 32);
        b = (z1 as u64) * (y0 as u64);
        c = (a >> 32) + (b >> 32);
        c += ((a as u32 as u64) + (b as u32 as u64)) >> 32;
        c += (z1 as u64) * (y1 as u64);
        y = C[u] - c;
        u += 1;
    }

    z = (fpr_trunc(fpr_mul(ccs, FPR_PTWO63)) << 1) as u64;
    z0 = z as u32;
    z1 = (z >> 32) as u32;
    y0 = y as u32;
    y1 = (y >> 32) as u32;
    a = ((z0 as u64) * (y1 as u64))
        + (((z0 as u64) * (y0 as u64)) >> 32);
    b = (z1 as u64) * (y0 as u64);
    y = (a >> 32) + (b >> 32);
    y += ((a as u32 as u64) + (b as u32 as u64)) >> 32;
    y += (z1 as u64) * (y1 as u64);

    return y;
}

pub fn fpr_mul(x: u64, y: u64) -> u64 {
    let (xu, yu, mut w, mut zu, zv): (u64, u64, u64, u64, u64);
    let (x0, x1, y0, y1, z0, mut z1, mut z2): (u32, u32, u32, u32, u32, u32, u32);
    let (ex, ey, d, e, s): (i32, i32, i32, i32, i32);

    xu = (x & (((1 as u64) << 52) - 1)) | ((1 as u64) << 52);
    yu = (y & (((1 as u64) << 52) - 1)) | ((1 as u64) << 52);


    x0 = (xu as u32) & 0x01FFFFFF;
    x1 = (xu >> 25) as u32;
    y0 = (yu as u32) & 0x01FFFFFF;
    y1 = (yu >> 25) as u32;
    w = (x0 as u64) * (y0 as u64);
    z0 = (w as u32) & 0x01FFFFFF;
    z1 = (w >> 25) as u32;
    w = (x0 as u64) * (y1 as u64);
    z1 += (w as u32) & 0x01FFFFFF;
    z2 = (w >> 25) as u32;
    w = (x1 as u64) * (y0 as u64);
    z1 += (w as u32) & 0x01FFFFFF;
    z2 += (w >> 25) as u32;
    zu = (x1 as u64) * (y1 as u64);
    z2 += z1 >> 25;
    z1 &= 0x01FFFFFF;
    zu += z2 as u64;

    zu |= (((z0 | z1) + 0x01FFFFFF) >> 25) as u64;

    zv = (zu >> 1) | (zu & 1);
    w = zu >> 55;
    zu ^= (zu ^ zv) & (!w + 1);

    ex = ((x >> 52) & 0x7FF) as i32;
    ey = ((y >> 52) & 0x7FF) as i32;
    e = ex + ey - 2100 + w as i32;

    /*
     * Sign bit is the XOR of the operand sign bits.
     */
    s = ((x ^ y) >> 63) as i32;


    d = ((ex + 0x7FF) & (ey + 0x7FF)) >> 11;
    zu &= (!d as u64) + 1;

    fpr(s, e, zu)
}

pub fn fpr_sqrt(x: u64) -> u64 {
    let (mut xu, mut q, mut s, mut r): (u64, u64, u64, u64);
    let (ex, mut e): (i32, i32);


    xu = (x & (((1 as u64) << 52) - 1)) | ((1 as u64) << 52);
    ex = ((x >> 52) & 0x7FF) as i32;
    e = ex - 1023;


    xu += xu & !((e & 1) as u64) + 1;
    e >>= 1;

    xu <<= 1;

    q = 0;
    s = 0;
    r = (1 as u64) << 53;
    let mut i = 0;
    while i < 54 {
        let (t, b): (u64, u64);

        t = s + r;
        b = ((xu - t) >> 63) - 1;
        s += (r << 1) & b;
        xu -= t & b;
        q += r & b;
        xu <<= 1;
        r >>= 1;

        i += 1;
    }

    q <<= 1;
    q |= (xu | !(xu + 1)) >> 63;

    e -= 54;


    q &= !(((ex + 0x7FF) >> 11) as u64) + 1;

    fpr(0, e, q)
}

#[inline(always)]
pub fn fpr_trunc(x: u64) -> i64 {
    let (t, mut xu): (u64, u64);
    let (e, cc): (i32, i32);

    e = ((x >> 52) & 0x7FF) as i32;
    xu = ((x << 10) | ((1 as u64) << 62)) & (((1 as u64) << 63) - 1);
    cc = 1085 - e;
    xu = fpr_ursh(xu, cc & 63);

    xu &= !((((cc - 64) as u32) >> 31) as u64) + 1;

    t = x >> 63;
    xu = (xu ^ (!t + 1)) + t;
    xu as i64
}


#[inline(always)]
fn fpr_ursh(mut x: u64, n: i32) -> u64 {
    x ^= (x ^ (x >> 32)) & (!(n >> 5) as u64 + 1);
    x >> (n & 31)
}

#[inline(always)]
fn fpr_ulsh(mut x: u64, n: i32) -> u64 {
    x ^= (x ^ (x << 32)) & (!(n >> 5) as u64 + 1);
    x << (n & 31)
}


#[inline(always)]
fn fpr(s: i32, mut e: i32, mut m: u64) -> u64 {
    let mut x: u64;
    let mut t: u32;
    let f: u32;

    e += 1076;
    t = (e as u32) >> 31;
    m &= (t as u64) - 1;
    t = (m >> 54) as u32;
    e &= -(t as i32);

    x = (((s as u64) << 63) | (m >> 2)) + ((e as u32 as u64) << 52);

    f = m as u32 & 7;
    x += ((0xC8u32 >> f) & 1) as u64;
    x
}