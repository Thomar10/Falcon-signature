use crate::mp31::{mp_add, mp_half, mp_montymul, tbmask};

pub fn zint_mul_small(m: &mut [u32], len: usize, x: u32) -> u32 {
    let mut cc: u32 = 0;
    for u in 0..len {
        let z: u64 = (m[u] as u64) * (x as u64) + cc as u64;
        m[u] = (z as u32) & 0x7FFFFFFF;
        cc = (z >> 31) as u32;
    }
    cc
}

pub fn zint_mod_small_unsigned(d: &[u32], len: usize, stride: usize, p: u32, p0i: u32, r2: u32) -> u32 {
    let mut x: u32 = 0;
    let z = mp_half(r2, p);
    let mut d_index = len * stride;
    for _ in (0..len).rev() {
        d_index -= stride;
        let mut w: u32 = d[d_index] - p;
        w += p & tbmask(w);
        x = mp_montymul(x, z, p, p0i);
        x = mp_add(x, w, p);
    }
    x
}

pub fn zint_add_mul_small(x: &mut [u32], len: usize, xstride: usize, y: &[u32], s: u32) {
    let mut cc: u32 = 0;
    let mut x_index = 0;
    for u in 0..len {
        let xw = x[x_index];
        let yw = y[u];
        let z: u64 = (yw as u64) * (s as u64) + (xw as u64) + (cc as u64);
        x[x_index] = (z as u32) & 0x7FFFFFFF;
        cc = (z >> 31) as u32;
        x_index += xstride;
    }
    x[xstride] = cc;
}
