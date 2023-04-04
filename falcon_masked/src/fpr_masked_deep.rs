use rand::{random, Rng, thread_rng};

use falcon::falcon::fpr;

pub fn secure_and<const ORDER: usize>(x: &[fpr], y: &[fpr]) -> [fpr; ORDER] {
    let mut rng = thread_rng();
    let mut r: [[fpr; ORDER]; ORDER] = [[0; ORDER]; ORDER];

    for i in 0..ORDER {
        for j in (i + 1)..ORDER {
            r[i][j] = rng.gen_range(0..2);
            r[j][i] = (r[i][j] ^ (x[i] & y[j])) ^ (x[j] & y[i]);
        }
    }
    let mut z = [0; ORDER];
    for i in 0..ORDER {
        z[i] = x[i] & y[i];
        for j in 0..ORDER {
            if i != j {
                z[i] = z[i] ^ r[i][j];
            }
        }
    }
    z
}

pub fn secure_or<const ORDER: usize>(x: &[fpr], y: &[fpr]) -> [fpr; ORDER] {
    let mut z = [0; ORDER];
    let t: [fpr; ORDER] = secure_and(x, y);
    for i in 0..ORDER {
        z[i] = x[i] ^ t[i] ^ y[i];
    }
    z
}

pub fn secure_non_zero<const ORDER: usize>(x: &[fpr], bits: i32, boolean: bool) -> [fpr; ORDER] {
    let mut t = [0; ORDER];
    if !boolean {
        t[0] = a2b_e(x[0] as i16, x[1] as i16) as u64;
        t[1] = !x[1];
    } else {
        for i in 0..ORDER {
            t[i] = x[i];
        }
    }
    let mut length = bits / 2;
    let mut l: [fpr; ORDER] = [0; ORDER];
    let mut r: [fpr; ORDER] = [0; ORDER];
    while length >= 1 {
        for i in 0..ORDER {
            l[i] = t[i] >> length;
            r[i] = t[i] & ((1 << length) - 1);
        }
        length = length / 2;
        t = secure_or(&l, &r);
    }
    for i in 0..ORDER {
        t[i] = t[i] & 1;
    }
    t
}

fn a2b_e(a: i16, r: i16) -> i16 {
    let mut gamma: i16 = random();
    let mut t = 2i16.wrapping_mul(gamma);
    let mut x = gamma ^ r;
    let mut omega = gamma & x;
    x = t ^ a;
    gamma = gamma ^ x;
    gamma = gamma & r;
    omega = omega ^ gamma;
    gamma = t & a;
    omega = omega ^ gamma;
    for _ in 1..16 {
        gamma = t & r;
        gamma = gamma ^ omega;
        t = t & a;
        gamma = gamma ^ t;
        t = 2i16.wrapping_mul(gamma);
    }
    return x ^ t;
}

fn a2b_u64(a: u64, r: u64) -> u64 {
    let mut gamma: u64 = random();
    let mut t = 2u64.wrapping_mul(gamma);
    let mut x = gamma ^ r;
    let mut omega = gamma & x;
    x = t ^ a;
    gamma = gamma ^ x;
    gamma = gamma & r;
    omega = omega ^ gamma;
    gamma = t & a;
    omega = omega ^ gamma;
    for _ in 1..64 {
        gamma = t & r;
        gamma = gamma ^ omega;
        t = t & a;
        gamma = gamma ^ t;
        t = 2u64.wrapping_mul(gamma);
    }
    return x ^ t;
}

fn a2b_u128(a: i128, r: i128) -> i128 {
    let mut gamma: i128 = random();
    let mut t = 2i128.wrapping_mul(gamma);
    let mut x = gamma ^ r;
    let mut omega = gamma & x;
    x = t ^ a;
    gamma = gamma ^ x;
    gamma = gamma & r;
    omega = omega ^ gamma;
    gamma = t & a;
    omega = omega ^ gamma;
    for _ in 1..106 {
        gamma = t & r;
        gamma = gamma ^ omega;
        t = t & a;
        gamma = gamma ^ t;
        t = 2i128.wrapping_mul(gamma);
    }
    return x ^ t;
}

fn b2a_i128(x: i128, r: i128) -> i128 {
    let rng: bool = random();
    let mut gamma: i128 = rng as i128;
    let mut t = x ^ gamma;
    t = t.wrapping_sub(gamma);
    t = t ^ x;
    gamma = gamma ^ r;
    let mut a = x ^ gamma;
    a = a.wrapping_sub(gamma);
    a ^ t
}


fn b2ai6(x: i8, r: i8) -> i8 {
    let randomi6: i8 = random();
    let mut gamma: i8 = randomi6 >> 2;
    let mut t = x ^ gamma;
    t = t.wrapping_sub(gamma);
    t = t ^ x;
    gamma = gamma ^ r;
    let mut a = x ^ gamma;
    a = a.wrapping_sub(gamma);
    a ^ t
}

fn b2a(x: i16, r: i16) -> i16 {
    let mut gamma: i16 = random();
    let mut t = x ^ gamma;
    t = t.wrapping_sub(gamma);
    t = t ^ x;
    gamma = gamma ^ r;
    let mut a = x ^ gamma;
    a = a.wrapping_sub(gamma);
    a ^ t
}



pub fn secure_mul<const ORDER: usize>(xx: &[fpr], yy: &[fpr]) -> [fpr; ORDER] {
    let mut s = [0; ORDER];
    let mut ex: [i16; ORDER] = [0; ORDER];
    let mut ey: [i16; ORDER] = [0; ORDER];
    let mut mx: [i128; ORDER] = [0; ORDER];
    let mut my: [i128; ORDER] = [0; ORDER];
    for i in 0..ORDER {
        s[i] = (xx[i] >> 63) ^ (yy[i] >> 63);
        ex[i] = ((xx[i] >> 52) & 0x7FF) as i16;
        ey[i] = ((yy[i] >> 52) & 0x7FF) as i16;
        mx[i] = (xx[i] & 0xFFFFFFFFFFFFF) as i128;
        my[i] = (yy[i] & 0xFFFFFFFFFFFFF) as i128;
    }
    ex[0] = b2a(ex[0], ex[1]);
    ey[0] = b2a(ey[0], ey[1]);
    mx[0] = b2a_i128(mx[0], mx[1]);
    my[0] = b2a_i128(my[0], my[1]);
    let mut e = [0; ORDER];
    e[0] = ex[0].wrapping_add(ey[0]).wrapping_sub(2100);
    e[1] = ex[1].wrapping_add(ey[1]);
    let mut mxx: [i128; ORDER] = [0; ORDER];
    mxx[0] = mx[0] + 4503599627370496;
    mxx[1] = mx[1];
    let mut myy: [i128; ORDER] = [0; ORDER];
    myy[0] = my[0] + 4503599627370496;
    myy[1] = my[1];
    let mut p: [i128; ORDER] = [0; ORDER];
    p[0] = mxx[0] * myy[0];
    p[1] = mxx[1] * myy[1] + mxx[1] * myy[0] + mxx[0] * myy[1];
    p[0] = a2b_u128(p[0], p[1]);
    let bb = secure_non_zero::<2>(&[p[0] as u64, p[1] as u64], 51, true);
    let mut z: [fpr; ORDER] = [0; ORDER];

    z[0] = ((p[0] >> 51) & 0xFFFFFFFFFFFFFF) as u64;
    z[1] = ((p[1] >> 51) & 0xFFFFFFFFFFFFFF) as u64;
    let mut zd: [fpr; ORDER] = [0; ORDER];
    zd[0] = ((p[0] >> 50) & 0xFFFFFFFFFFFFFF) as u64;
    zd[1] = ((p[1] >> 50) & 0xFFFFFFFFFFFFFF) as u64;
    for i in 0..ORDER {
        zd[i] = z[i] ^ zd[i];
    }
    let mut w: [i16; ORDER] = [0; ORDER];
    w[0] = ((p[0] >> 105) & 1) as i16;
    w[1] = ((p[1] >> 105) & 1) as i16;
    let mut zz = xor::<ORDER>(&z, &zd);
    let zd: [u64; ORDER] = secure_and(&zd, &[w[0].wrapping_neg() as u64, w[1].wrapping_neg() as u64]);
    z = xor(&zz, &zd);

    let z = secure_or::<2>(&z, &bb);
    w[0] = b2a(w[0], w[1]);

    for i in 0..ORDER {
        e[i] = e[i] + w[i] as i16;
    }
    let bx: [fpr; ORDER] = secure_non_zero(&[ex[0] as u64, ex[1] as u64], 16, false);
    let by: [fpr; ORDER] = secure_non_zero(&[ex[0] as u64, ex[1] as u64], 16, false);
    let d = secure_and::<2>(&bx, &by);
    let z = secure_and::<2>(&z, &[d[0].wrapping_neg(), d[1].wrapping_neg()]);
    secure_fpr(&s, &mut e, &z)
}


pub fn secure_fpr_norm<const ORDER: usize>(xx: &[u64], ee: &[i16]) -> ([fpr; ORDER], [i16; ORDER]) {
    let mut e: [i16; ORDER] = [0; ORDER];
    let mut x: [u64; ORDER] = [0; ORDER];
    e[0] = ee[0].wrapping_sub(63);
    e[1] = ee[1];
    x[0] = xx[0];
    x[1] = xx[1];
    let mut j: i32 = 5;
    let mut n: [u64; ORDER] = [0; ORDER];
    let mut t: [u64; ORDER] = [0; ORDER];
    while j >= 0 {
        let rsh = 64 - (1 << j);
        for i in 0..ORDER {
            n[i] = x[i] >> rsh;
            t[i] = x[i] << (64 - rsh);
            t[i] = t[i] ^ x[i];
        }

        let mut b: [fpr; ORDER] = secure_non_zero(&n, (64 - rsh) as i32, true);
        let mut bb: [i64; ORDER] = [0; ORDER];
        bb[0] = -(b[0] as i64);
        bb[1] = -(b[1] as i64);
        bb[0] = !bb[0];

        let t: [fpr; ORDER] = secure_and(&t, &[bb[0] as u64, bb[1] as u64]);
        for i in 0..ORDER {
            x[i] = x[i] ^ t[i];
        }

        b[0] = b2ai6(b[0] as i8, b[1] as i8) as u64;
        for i in 0..ORDER {
            e[i] = e[i] + ((b[i] as i16) << j);
        }
        j = j.wrapping_sub(1);
    }
    (x, e)
}

pub fn secure_fpr_sub<const ORDER: usize>(x: &[fpr], y: &[fpr]) -> ([fpr; ORDER]) {
    let mut yy: [u64; ORDER] = [0; ORDER];
    yy[0] = y[0] ^ (1u64 << 63);
    yy[1] = y[1];
    secure_fpr_add(x, &yy)
}


pub fn secure_fpr_add<const ORDER: usize>(x: &[fpr], y: &[fpr]) -> ([fpr; ORDER]) {
    let mut xm: [u64; ORDER] = [0; ORDER];
    let mut ym: [u64; ORDER] = [0; ORDER];
    for i in 0..ORDER {
        xm[i] = x[i] & 0x7FFFFFFFFFFFFFFF;
        ym[i] = y[i] & 0x7FFFFFFFFFFFFFFF;
    }
    ym[0] = !ym[0];
    let mut refs: [u64; ORDER] = [0; ORDER];
    refs[0] = 1;
    refresh::<2>(&mut refs);
    let ym: [u64; ORDER] = secure_add_u64(ym[0], refs[0], ym[1], refs[1]);
    let d: [u64; ORDER] = secure_add_u64(ym[0], xm[0], ym[1], xm[1]);
    let mut b: [u64; ORDER] = secure_non_zero(&d, 64, true);
    b[0] = !b[0];
    let mut cs: [u64; ORDER] = secure_and(&b, &[x[0] >> 63, x[1] >> 63]);
    cs = secure_or(&cs, &[d[0] >> 63, d[1] >> 63]);
    let xy: [u64; ORDER] = secure_and(&[x[0] ^ y[0], x[1] ^ y[1]],
                                      &[cs[0].wrapping_neg(), cs[1].wrapping_neg()]);
    let mut xx: [u64; ORDER] = [0; ORDER];
    let mut yy: [u64; ORDER] = [0; ORDER];
    for i in 0..ORDER {
        xx[i] = x[i] ^ (xy[i] as u64);
        yy[i] = y[i] ^ (xy[i] as u64);
    }
    let mut sx: [u64; ORDER] = [0; ORDER];
    let mut sy: [u64; ORDER] = [0; ORDER];
    let mut ex: [i16; ORDER] = [0; ORDER];
    let mut ey: [i16; ORDER] = [0; ORDER];
    let mut mx: [u64; ORDER] = [0; ORDER];
    let mut my: [u64; ORDER] = [0; ORDER];
    for i in 0..ORDER {
        sx[i] = xx[i] >> 63;
        sy[i] = yy[i] >> 63;
        ex[i] = ((xx[i] >> 52) & 0x7FF) as i16;
        ey[i] = ((yy[i] >> 52) & 0x7FF) as i16;
        mx[i] = xx[i] & 0xFFFFFFFFFFFFF;
        my[i] = yy[i] & 0xFFFFFFFFFFFFF;
    }
    my[0] = my[0] | 0x10000000000000;
    mx[0] = mx[0] | 0x10000000000000;
    for i in 0..ORDER {
        mx[i] <<= 3;
        my[i] <<= 3;
    }
    ex[0] = b2a(ex[0], ex[1]);
    ey[0] = b2a(ey[0], ey[1]);
    ex[0] = ex[0].wrapping_sub(1078);
    ey[0] = ey[0].wrapping_sub(1078);

    let mut n: [i16; ORDER] = [0; ORDER];
    let mut nd: [i16; ORDER] = [0; ORDER];
    for i in 0..ORDER {
        n[i] = ex[i].saturating_sub(ey[i]);
        nd[i] = n[i];
    }
    nd[0] = nd[0].wrapping_sub(60);
    nd[0] = a2b_e(nd[0], nd[1]);
    n[0] = a2b_e(n[0], n[1]);
    let my: [u64; ORDER] = secure_and(&my,
                                      &[((nd[0] >> 15) & 1).wrapping_neg() as u64,
                                          ((nd[1] >> 15) & 1).wrapping_neg() as u64]);

    let mut my: [fpr; ORDER] = secure_ursh(&[my[0] as u64, my[1] as u64], &[n[0] as i8, n[1] as i8]);

    let mut myd: [u64; ORDER] = [0; ORDER];
    myd[0] = !my[0];
    myd[1] = my[1];
    let mut myd: [u64; ORDER] = secure_add_u64(myd[0], refs[0], myd[1], refs[1]);
    let mut s: [u64; ORDER] = [0; ORDER];
    for i in 0..ORDER {
        s[i] = (-((sx[i] ^ sy[i]) as i64)) as u64;
        myd[i] = my[i] ^ myd[i];
    }
    myd = secure_and(&myd, &s);
    for i in 0..ORDER {
        my[i] = my[i] ^ (myd[i] as u64);
    }
    let z: [u64; ORDER] = secure_add_u64(mx[0], my[0], mx[1], my[1]);
    let (z, mut ex) = secure_fpr_norm::<2>(&z, &ex);
    let b: [fpr; ORDER] = secure_non_zero(&z, 9, true);
    let mut zz: [u64; ORDER] = [0; ORDER];
    for i in 0..ORDER {
        zz[i] = z[i] >> 9;
    }
    zz = secure_or(&zz, &b);
    ex[0] += 9;
    secure_fpr(&sx, &mut ex, &zz)
}


pub fn refresh<const ORDER: usize>(z: &mut [u64]) {
    for i in 0..ORDER {
        let rng: u64 = random();
        z[0] ^= rng;
        z[i] ^= rng;
    }
}

pub fn secure_fpr<const ORDER: usize>(s: &[u64], e: &mut [i16], z: &[u64]) -> [fpr; ORDER] {
    let mut b: [u64; ORDER] = [0; ORDER];
    let mut e1: [i16; ORDER] = [0; ORDER];
    e1[0] = e[0].wrapping_add(1076);
    e1[0] = a2b_e(e1[0], e[1]);
    e1[1] = e[1];
    b[0] = (e1[0] >> 15) as u64;
    b[1] = (e1[1] >> 15) as u64;
    let z: [u64; ORDER] = secure_and(&z, &[!b[0], b[1]]);
    b[0] = (z[0] >> 54) & 1;
    b[1] = (z[1] >> 54) & 1;

    let mut e: [u64; ORDER] = secure_and(&[e1[0] as u64, e1[1] as u64], &[b[0].wrapping_neg(), b[1].wrapping_neg()]);
    e = secure_add_u64(e[0], (z[0] >> 54) & 1, e[1], (z[1] >> 54) & 1);
    let xx: [u64; ORDER] = secure_or(&[s[0] << 63, s[1] << 63],
                                     &[(z[0] & 0x3FFFFFFFFFFFFF) >> 2, (z[1] & 0x3FFFFFFFFFFFFF) >> 2]);
    let mut x: [u64; ORDER] = secure_or(&xx,
                                        &[(e[0] & 0x7FF) << 52, (e[1] & 0x7FF) << 52]);

    let mut f: [u64; ORDER] = secure_or(&[z[0] & 1, z[1] & 1], &[(z[0] >> 2) & 1, (z[1] >> 2) & 1]);
    f = secure_and(&f, &[(z[0] >> 1) & 1, (z[1] >> 1) & 1]);
    let xx: [u64; ORDER] = secure_add_u64(x[0], f[0], x[1], f[1]);
    xx
}


pub fn secure_add_u64<const ORDER: usize>(x: u64, y: u64, rx: u64, ry: u64) -> [u64; ORDER] {
    let mut z = [0; ORDER];
    let mut rng = thread_rng();
    let mut c = rng.gen_range(0..2);
    let (mut t, mut omega, mut b, mut a0, a1);
    t = x & y;
    omega = c ^ t;
    t = x & ry;
    omega = omega ^ t;
    t = y & rx;
    omega = omega ^ t;
    t = rx & ry;
    omega = omega ^ t;

    b = omega << 1;
    c = c << 1;
    a0 = x ^ y;
    a1 = rx ^ ry;
    t = c & a0;
    omega = omega ^ t;
    t = c & a1;
    omega = omega ^ t;

    for _ in 2..64 {
        t = b & a0;
        b = b & a1;
        b = b ^ omega;
        b = b ^ t;
        b = b << 1;
    }

    a0 = a0 ^ b;
    a0 = a0 ^ c;
    z[0] = a0;
    z[1] = a1;
    z
}

#[inline(always)]
pub fn xor<const ORDER: usize>(x: &[fpr], y: &[fpr]) -> [fpr; ORDER] {
    let mut z = [0; ORDER];
    for i in 0..ORDER {
        z[i] = x[i] ^ y[i];
    }
    z
}

pub fn secure_ursh<const ORDER: usize>(x: &[fpr], n: &[i8]) -> [fpr; ORDER] {
    let mut y = [0; ORDER];
    let mut nn = [0; ORDER];
    for i in 0..ORDER {
        y[i] = (x[i] >> 32) ^ x[i];
        nn[i] = (n[i] >> 5) as u64;
    }
    let mut mask: [u64; ORDER] = [0; ORDER];
    nn[0] = !nn[0];
    mask[0] = 1;
    refresh::<ORDER>(&mut mask);
    nn = secure_add_u64(nn[0], mask[0], nn[1], mask[1]);

    let mut z: [fpr; ORDER] = secure_and(&y, &nn);
    for i in 0..ORDER {
        z[i] = z[i] ^ x[i];
    }
    let mut nn = [0; ORDER];
    nn[0] = n[0] & 31;
    nn[1] = n[1] & 31;
    nn[0] = b2ai6(nn[0], nn[1]);
    if nn[0] < 0 {
        z[0] = (((z[0] as u128) << nn[0].abs()) >> nn[1]) as u64;
        z[1] = (((z[1] as u128) << nn[0].abs()) >> nn[1]) as u64;
    } else if nn[1] < 0 {
        z[0] = (((z[0] as u128) << nn[1].abs()) >> nn[0]) as u64;
        z[1] = (((z[1] as u128) << nn[1].abs()) >> nn[0]) as u64;
    } else {
        z[0] = (z[0] >> nn[0]) >> nn[1];
        z[1] = (z[1] >> nn[0]) >> nn[1];
    }
    z
}