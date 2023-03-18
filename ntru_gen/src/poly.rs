use crate::mp31::mp_set;

pub fn poly_mp_set_small(logn: usize, d: &mut [u32], f: &[i8], p: u32) {
    for u in 0..(1 << logn) {
        d[u] = mp_set(f[u] as i32, p);
    }
}