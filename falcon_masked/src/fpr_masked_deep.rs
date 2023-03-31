use rand::{random, Rng, thread_rng};

use falcon::falcon::fpr;

//
// pub fn fpr_add<const ORDER: usize>(x: &[fpr], y: &[fpr]) -> [fpr; ORDER] {
//     let mut z = [0; ORDER];
//     let mut xi = [0; ORDER];
//     let mut yi = [0; ORDER];
//
//     for i in 0..ORDER {
//         xi[i] = x[i] << 63;
//     }
//     z
// }
//
// pub fn fpr_ursh<const ORDER: usize>(x: &[fpr], n: &[u8]) -> [fpr; ORDER] {
//     let mut z: [fpr; ORDER] = [0; ORDER];
//     let mut y = x.clone();
//     let mut m: [fpr; ORDER] = [0; ORDER];
//     m[0] = 1 << 63;
//     // refresh
//     for i in 0..ORDER {
//         y[i] >>= n[i];
//         m[i] >>= n[i];
//     }
//     z
// }

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

pub fn secure_and_i128<const ORDER: usize>(x: &[i128], y: &[i128]) -> [i128; ORDER] {
    let mut rng = thread_rng();
    let mut r: [[i128; ORDER]; ORDER] = [[0; ORDER]; ORDER];

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

pub fn secure_and_i64<const ORDER: usize>(x: &[i64], y: &[i64]) -> [i64; ORDER] {
    let mut rng = thread_rng();
    let mut r: [[i64; ORDER]; ORDER] = [[0; ORDER]; ORDER];

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

pub fn secure_or_i128<const ORDER: usize>(x: &[i128], y: &[i128]) -> [i128; ORDER] {
    let mut z = [0; ORDER];
    let t: [i128; ORDER] = secure_and_i128(x, y);
    for i in 0..ORDER {
        z[i] = x[i] ^ t[i] ^ y[i];
    }
    z
}

pub fn secure_or_i64<const ORDER: usize>(x: &[i64], y: &[i64]) -> [i64; ORDER] {
    let mut z = [0; ORDER];
    let t: [i64; ORDER] = secure_and_i64(x, y);
    for i in 0..ORDER {
        z[i] = x[i] ^ t[i] ^ y[i];
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
    // if bits == 1 {
    //     let mut t = secure_or(&t, &t);
    //     for i in 0..ORDER {
    //         t[i] = t[i] & 1;
    //     }
    //     return t;
    // }
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
    for _ in 1..128 {
        gamma = t & r;
        gamma = gamma ^ omega;
        t = t & a;
        gamma = gamma ^ t;
        t = 2i128.wrapping_mul(gamma);
    }
    return x ^ t;
}

fn b2a_i64(x: i64, r: i64) -> i64 {
    let rng: bool = random();
    let mut gamma: i64 = rng as i64;
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

pub fn secure_mul_test<const ORDER: usize>(sx: &[u64], ex: &mut [i16], mx: &mut [i128], sy: &[u64], ey: &mut [i16], my: &mut [i128]) -> ([u64; ORDER], [i16; ORDER], [i64; ORDER]) {
    let mut s = [0; ORDER];
    for i in 0..ORDER {
        s[i] = sx[i] ^ sy[i];
    }
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
    let mut bb = secure_non_zero::<2>(&[p[0] as u64, p[1] as u64], 51, true);
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
    let mut zz: [fpr; ORDER] = [0; ORDER];
    for i in 0..ORDER {
        zz[i] = z[i] ^ (zd[i] as u64);
    }
    let zd: [i64; ORDER] = secure_and_i64(&[zd[0] as i64, zd[1] as i64], &[-(w[0] as i64), -(w[1] as i64)]);
    for i in 0..ORDER {
        z[i] = zz[i] ^ (zd[i] as u64);
    }
    let z = secure_or::<2>(&z, &bb);
    w[0] = b2a(w[0], w[1]);

    for i in 0..ORDER {
        e[i] = e[i] + w[i] as i16;
    }
    let bx: [fpr; ORDER] = secure_non_zero(&[ex[0] as u64, ex[1] as u64], 16, false);
    let by: [fpr; ORDER] = secure_non_zero(&[ex[0] as u64, ex[1] as u64], 16, false);
    let d = secure_and::<2>(&bx, &by);
    let z: [i64; ORDER] = secure_and_i64(&[z[0] as i64, z[1] as i64], &[-(d[0] as i64), -(d[1] as i64)]);
    (s, e, z)
}

pub fn secure_mul<const ORDER: usize>(sx: &[u64], ex: &mut [i16], mx: &mut [i128], sy: &[u64], ey: &mut [i16], my: &mut [i128]) -> [fpr; ORDER] {
    let mut s = [0; ORDER];
    for i in 0..ORDER {
        s[i] = sx[i] ^ sy[i];
    }
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
    let mut bb = secure_non_zero::<2>(&[p[0] as u64, p[1] as u64], 51, true);
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
    let mut zz: [fpr; ORDER] = [0; ORDER];
    for i in 0..ORDER {
        zz[i] = z[i] ^ (zd[i] as u64);
    }
    let zd: [i64; ORDER] = secure_and_i64(&[zd[0] as i64, zd[1] as i64], &[-(w[0] as i64), -(w[1] as i64)]);
    for i in 0..ORDER {
        z[i] = zz[i] ^ (zd[i] as u64);
    }
    let z = secure_or::<2>(&z, &bb);
    w[0] = b2a(w[0], w[1]);

    for i in 0..ORDER {
        e[i] = e[i] + w[i] as i16;
    }
    let bx: [fpr; ORDER] = secure_non_zero(&[ex[0] as u64, ex[1] as u64], 16, false);
    let by: [fpr; ORDER] = secure_non_zero(&[ex[0] as u64, ex[1] as u64], 16, false);
    let d = secure_and::<2>(&bx, &by);
    let z = secure_and_i64::<2>(&[z[0] as i64, z[1] as i64], &[-(d[0] as i64), -(d[1] as i64)]);
    secure_fpr(&s, &mut e, &mut [z[0], z[1]])
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

        b[0] = b2a_i64(b[0] as i64, b[1] as i64) as u64;
        for i in 0..ORDER {
            e[i] = e[i] + ((b[i] as i16) << j);
        }
        j = j.wrapping_sub(1);
    }
    (x, e)
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
    let ym: [i64; ORDER] = secure_add(ym[0] as i64, refs[0] as i64, ym[1] as i64, refs[1] as i64);
    let d: [i64; ORDER] = secure_add(ym[0], xm[0] as i64, ym[1], xm[1] as i64);
    let mut b: [u64; ORDER] = secure_non_zero(&[d[0] as u64, d[1] as u64], 64, true);
    b[0] = !b[0];
    let mut cs: [u64; ORDER] = secure_and(&b, &[x[0] >> 63, x[1] >> 63]);
    cs = secure_or(&cs, &[((d[0] as u64) >> 63) as u64, ((d[1] as u64) >> 63) as u64]);
    let xy: [i64; ORDER] = secure_and_i64(&[(x[0] ^ y[0]) as i64, (x[1] ^ y[1]) as i64], &[-(cs[0] as i64), -(cs[1] as i64)]);
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
        mx[i] = (xx[i] & 0xFFFFFFFFFFFFF);
        my[i] = (yy[i] & 0xFFFFFFFFFFFFF);
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
        n[i] = ex[i].wrapping_sub(ey[i]);
        nd[i] = n[i];
    }
    nd[0] = nd[0].wrapping_sub(60);
    nd[0] = a2b_e(nd[0], nd[1]);
    let my: [i64; ORDER] = secure_and_i64(&[my[0] as i64, my[1] as i64],
                                          &[(-((nd[0] >> 15) & 1) as i64), (-((nd[1] >> 15) & 1) as i64)]);

    let mut my: [fpr; ORDER] = secure_fpr_ursh(&[my[0] as u64, my[1] as u64], &[(n[0] & 0x3F) as i8, (n[1] & 0x3F) as i8]);
    println!("n {}", (n[0] & 0x3F) ^ (n[1] & 0x3F));
    println!("my {}", my[0] ^my[1]);


    let mut myd: [u64; ORDER] = [0; ORDER];
    myd[0] = !my[0];
    myd[1] = my[1];
    let mut refs: [u64; ORDER] = [0; ORDER];
    refs[0] = 1;
    refresh::<2>(&mut refs);
    let mut myd: [i64; ORDER] = secure_add(myd[0] as i64, refs[0] as i64, myd[1] as i64, refs[1] as i64);
    let mut s: [u64; ORDER] = [0; ORDER];
    for i in 0..ORDER {
        s[i] = (-((sx[i] ^ sy[i]) as i64)) as u64;
        myd[i] = (my[i] as i64) ^ myd[i];
    }
    myd = secure_and_i64(&myd, &[s[0] as i64, s[1] as i64]);
    for i in 0..ORDER {
        my[i] = my[i] ^ (myd[i] as u64);
    }
    let z: [i64; ORDER] = secure_add(mx[0] as i64, my[0] as i64, mx[1] as i64, my[1] as i64);
    let (z, mut ex) = secure_fpr_norm::<2>(&[z[0] as u64, z[1] as u64], &[ex[0] as i16, ex[1] as i16]);
    let b: [fpr; ORDER] = secure_non_zero(&z, 9, true);
    let mut zz: [i64; ORDER] = [0; ORDER];
    for i in 0..ORDER {
        zz[i] = (z[i] >> 9) as i64;
    }
    zz = secure_or_i64(&zz, &[b[0] as i64, b[1] as i64]);
    ex[0] += 9;
    secure_fpr(&sx, &mut ex, &zz)
}

pub fn secure_fpr_ursh<const ORDER: usize>(xx: &[u64], n: &[i8]) -> ([fpr; ORDER]) {
    let mut y: [u64; ORDER] = [0; ORDER];
    if n[0] < 0 || n[1] < 0 {
        y[0] = (xx[0] >> (n[0] + n[1]));
        y[1] = (xx[1] >> (n[0] + n[1]));
    } else {
        y[0] = (xx[0] >> n[0]) >> n[1];
        y[1] = (xx[1] >> n[0]) >> n[1];
    }
    y
}

pub fn refresh<const ORDER: usize>(z: &mut [u64]) {
    for i in 0..ORDER {
        let rng: u64 = random();
        z[0] ^= rng;
        z[i] ^= rng;
    }
}

pub fn refresh_i64<const ORDER: usize>(z: &mut [i64]) {
    for i in 0..ORDER {
        let rng: i64 = random();
        z[0] ^= rng;
        z[i] ^= rng;
    }
}

pub fn secure_fpr<const ORDER: usize>(s: &[u64], e: &mut [i16], z: &[i64]) -> [fpr; ORDER] {
    let mut b: [i64; ORDER] = [0; ORDER];
    let mut e1: [i64; ORDER] = [0; ORDER];
    e1[0] = e[0].wrapping_add(1076) as i64;
    e1[0] = a2b_e(e1[0] as i16, e[1]) as i64;
    e1[1] = e[1] as i64;
    b[0] = (e1[0] >> 15) as i64;
    b[1] = (e1[1] >> 15) as i64;
    let z: [i64; ORDER] = secure_and_i64(&z, &[!b[0], b[1]]);
    b[0] = (z[0] >> 54) & 1;
    b[1] = (z[1] >> 54) & 1;

    let mut e: [i64; ORDER] = secure_and_i64(&e1, &[-b[0], -b[1]]);
    e = secure_add(e[0], (z[0] >> 54) & 1, e[1], (z[1] >> 54) & 1);
    let mut xx: [u64; ORDER] = secure_or(&[s[0] << 63, s[1] << 63], &[((z[0] & 0x3FFFFFFFFFFFFF) >> 2) as u64, ((z[1] & 0x3FFFFFFFFFFFFF) >> 2) as u64]);
    let mut x: [u64; ORDER] = secure_or(&xx,
                                        &[((e[0] & 0x7FF) << 52) as u64, ((e[1] & 0x7FF) << 52) as u64]);

    let mut f: [i64; ORDER] = secure_or_i64(&[z[0] & 1, z[1] & 1], &[(z[0] >> 2) & 1, (z[1] >> 2) & 1]);
    f = secure_and_i64(&f, &[(z[0] >> 1) & 1, (z[1] >> 1) & 1]);
    let xx: [i64; ORDER] = secure_add(x[0] as i64, f[0], x[1] as i64, f[1]);
    for i in 0..ORDER {
        x[i] = xx[i] as u64;
    }
    x
}

pub fn secure_add<const ORDER: usize>(x: i64, y: i64, rx: i64, ry: i64) -> [i64; ORDER] {
    let mut z = [0; ORDER];
    let mut rng = thread_rng();
    let mut c = rng.gen_range(0..2);
    let (mut t, mut omega, mut b, mut a0, mut a1);
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

    for _ in 2..16 {
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


pub fn secure_neg<const ORDER: usize>(x: &[fpr]) -> [fpr; ORDER] {
    let mut z = [0; ORDER];
    for i in 0..ORDER - 1 {
        z[i] = x[i];
    }
    z[ORDER - 1] = x[ORDER - 1].wrapping_neg();
    z
}

pub fn secure_xor<const ORDER: usize>(x: &[fpr], y: &[fpr]) -> [fpr; ORDER] {
    let mut z = [0; ORDER];
    for i in 0..ORDER {
        z[i] = x[i] ^ y[i];
    }
    z
}

pub fn secure_ursh<const ORDER: usize>(x: &[fpr], n: &[i8]) -> [fpr; ORDER] {
    let mut n_shift = [0; ORDER];

    n_shift//z
}

pub fn secure_rhs_const<const ORDER: usize>(x: &[fpr], c: i32) -> [fpr; ORDER] {
    let mut z = [0; ORDER];
    for i in 0..ORDER {
        z[i] = x[i] >> c;
    }
    z
}