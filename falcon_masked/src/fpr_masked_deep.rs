use rand::{Rng, thread_rng};

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
        // convert to boolean
    } else {
        for i in 0..ORDER {
            t[i] = x[i];
        }
    }
    let length = bits / 2;
    let mut l: [fpr; ORDER] = [0; ORDER];
    let mut r: [fpr; ORDER] = [0; ORDER];
    for i in 0..ORDER {
        l[i] = t[i] >> length;
        r[i] = t[i] & ((1 << length) - 1);
    }
    t = secure_or(&l, &r);
    for i in 0..ORDER {
        t[i] = t[i] & 1;
    }
    t
}

fn a2b_e(a: i16, r: i16) -> i16 {
    let mut gamma: i16 = rand::random();
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

pub fn secure_fpr<const ORDER: usize>(s: &[u64], e: &mut [i16], z: &mut [i64]) -> [fpr; ORDER] {
    let mut x = [0; ORDER];
    let mut b: [i64; ORDER] = [0; ORDER];
    let mut e1: [i64; ORDER] = [0; ORDER];
    e1[0] = e[0].wrapping_add(1076) as i64;
    e1[0] = a2b_e(e1[0] as i16, e[1]) as i64;
    e1[1] = e[1] as i64;
    b[0] = !(e1[0] >> 15) as i64;
    b[1] = (e1[1] >> 15) as i64;
    let z: [i64; ORDER] = secure_and_i64(&z, &b);
    for i in 0..ORDER {
        b[i] = -(z[i] >> 54);
    }
    let mut e: [i64; ORDER] = secure_and_i64(&e1, &b);
    e = secure_add(e[0], (z[0] >> 54) & 1, e[1], (z[1] >> 54) & 1);
    for i in 0..ORDER {
        x[i] = (((s[i] << 63) as i64) | ((e[i] & 0x7FF) << 52) | ((z[i] & 0x3FFFFFFFFFFFFF) >> 2)) as u64
    }
    let mut f: [i64; ORDER] = secure_or_i64(&[z[0] & 1, z[1] & 1], &[(z[0] >> 2) & 1, (z[1] >> 2) & 1]);
    f = secure_and_i64(&f, &[(z[0] >> 1) & 1, (z[1] >> 1) & 1]);
    let xx: [i64; ORDER] = secure_add(x[0] as i64, f[0], x[1] as i64, f[1]);
    x[0] = xx[0] as u64;
    x[1] = xx[1] as u64;
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