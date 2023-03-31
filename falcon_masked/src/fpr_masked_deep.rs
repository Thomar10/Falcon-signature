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
    let mut gamma: i64 = random();
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
    // println!("e {}", e[0].wrapping_add(e[1]));
    x[0] = xx[0];
    x[1] = xx[1];
    let mut j: i32 = 5;
    let mut n: [u64; ORDER] = [0; ORDER];
    let mut t: [u64; ORDER] = [0; ORDER];
    while j >= 0 {
        let rsh = 64u64 - 2u64.pow(j as u32);
        for i in 0..ORDER {
            n[i] = x[i] >> rsh;
            t[i] = x[i] << (64 - rsh);
            t[i] = t[i] ^ x[i];
        }
        let mut nt: u32 = ((n[0]) ^ (n[1])) as u32;
        nt = (nt | nt.wrapping_neg()) >> 31;
        // let mut b: [fpr; ORDER] = secure_non_zero(&n, (64 - rsh) as i32, true);
        // let mut bb: [i64; ORDER] = [0; ORDER];
        // bb[0] = -(b[0] as i64);
        // bb[1] = -(b[1] as i64);
        // bb[0] = !bb[0];
        println!("bbb {}", nt);
        // let t: [fpr; ORDER] = secure_and(&t, &[bb[0] as u64, bb[1] as u64]);
        // for i in 0..ORDER {
        //     x[i] = x[i] ^ t[i];
        // }

        // b[0] = b2a_i64(b[0] as i64, b[1] as i64) as u64;
        // println!("bb{} {}", j, bb[0] + bb[1]);
        e[0] = e[0] + ((nt as i16) << j);
        e[1] = e[1] + (0 << j);
        // for i in 0..ORDER {
        //     e[i] = e[i] + ((b[i] as i16) << j);
        // }
        // println!("e {}", e[0].wrapping_add(e[1]));
        j = j.wrapping_sub(1);
    }
    (x, e)
}

pub fn secure_fpr_ursh<const ORDER: usize>(xx: &[u64], n: &[i8]) -> ([fpr; ORDER]) {
    let mut y: [u64; ORDER] = [0; ORDER];
    let mut m: [u64; ORDER] = [0; ORDER];
    for i in 0..ORDER {
        y[i] = xx[i];
    }
    m[0] = 1 << 63;
    for i in 0..ORDER {
        y[i] = y[i].wrapping_shr(n[i] as u32);
        m[i] = m[i].wrapping_shr(n[i] as u32);
    }
    let md: [i64; ORDER] = secure_add(m[0] as i64, -1, m[1] as i64, 0);
    let mut m: [fpr; ORDER] = secure_or(&m, &[md[0] as u64, md[1] as u64]);
    let z: [fpr; ORDER] = secure_and(&y, &m);
    m[0] = !m[0];
    let b: [fpr; ORDER] = secure_non_zero(&m, 64, true);
    secure_or(&z, &b)
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