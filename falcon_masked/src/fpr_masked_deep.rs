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
    let mut length = bits / 2;
    let mut l = [0; ORDER];
    let mut r = [0; ORDER];
    while length > 1 {
        for i in 0..ORDER {
            l[i] = t[i] >> length;
            r[i] = t[i] & ((1 << length) - 1);
            t[i] = secure_or(l, r);
            length = length / 2;
        }
    }
    for i in 0..ORDER {
        t[i] = t[i] & 1;
    }
    t
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