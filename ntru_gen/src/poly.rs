use crate::fxp::fxr;
use crate::mp31::{lzcnt, mp_norm, mp_set, tbmask};

pub fn poly_mp_set_small(logn: usize, d: &mut [u32], f: &[i8], p: u32) {
    for u in 0..(1 << logn) {
        d[u] = mp_set(f[u] as i32, p);
    }
}

pub fn poly_mp_set(logn: usize, f: &mut [u32], p: u32) {
    for u in 0..(1 << logn) {
        let mut x = f[u];
        x |= (x & 0x40000000) << 1;
        f[u] = mp_set(x as i32, p);
    }
}

pub fn poly_mp_norm(logn: usize, f: &mut [u32], p: u32) {
    for u in 0..(1 << logn) {
        f[u] = (mp_norm(f[u], p) & 0x7FFFFFFF) as u32;
    }
}

pub fn poly_big_to_small(logn: usize, d: &mut [i8], s: &[u32], lim: i32) -> bool {
    let n: usize = 1 << logn;
    for u in 0..n {
        let mut x = s[u];
        x |= (x & 0x40000000) << 1;
        let z: i32 = x as i32;
        if z < -lim || z > lim {
            return false;
        }
        d[u] = z as i8;
    }
    true
}

pub fn poly_max_bitlength(logn: usize, f: &[u32], flen: usize) -> u32 {
    if flen == 0 {
        return 0;
    }
    let n = 1 << logn;
    let mut t: u32 = 0;
    let mut tk: u32 = 0;
    let mut f_index = 0;
    for _ in 0..n {
        let m = (!(f[((flen - 1) << logn) + f_index] >> 30)).wrapping_add(1) & 0x7FFFFFFF;
        let mut c = 0;
        let mut ck = 0;
        for v in 0..flen {
            let w = f[(v << logn) + f_index] ^ m;
            let nz = ((w.wrapping_sub(1)) >> 31).wrapping_sub(1);
            c ^= nz & (c ^ w);
            ck ^= nz & (ck ^ (v as u32));
        }

        let rr = tbmask((tk.wrapping_sub(ck)) |
            (((tk ^ ck).wrapping_sub(1)) & (t.wrapping_sub(c))));
        t ^= rr & (t ^ c);
        tk ^= rr & (tk ^ ck);
        f_index += 1;
    }

    31 * tk + 32 - lzcnt(t)
}

pub fn poly_big_to_fixed(logn: usize, d: &mut [fxr], f: &[u32], len: usize, sc: u32) {
    let n = 1 << logn;
    if len == 0 {
        d.fill(0);
        return;
    }

    let (mut sch, mut scl) = divrev31(sc);
    let z: u32 = (scl - 1) >> 31;
    sch -= z;
    scl |= 31 & (!z).wrapping_add(1);

    let t0 = ((sch - 1) as u32) & 0xFFFFFF;
    let t1 = sch & 0xFFFFFF;
    let t2 = ((sch + 1) as u32) & 0xFFFFFF;


    let mut f_index = 0;
    for u in 0..n {
        let mut w0: u32 = 0;
        let mut w1: u32 = 0;
        let mut w2: u32 = 0;
        for v in 0..len {
            let w = f[(v << logn) + f_index];
            let t = (v as u32) & 0xFFFFFF;
            w0 |= w & (!(((t ^ t0) - 1) >> 31) as u32).wrapping_add(1);
            w1 |= w & (!(((t ^ t1) - 1) >> 31) as u32).wrapping_add(1);
            w2 |= w & (!(((t ^ t2) - 1) >> 31) as u32).wrapping_add(1);
        }

        let ws = (!(f[((len - 1) << logn) + f_index] >> 30)).wrapping_add(1) >> 1;
        w0 |= ws & (!((((len as u32).wrapping_sub(sch)) >> 31) as u32)).wrapping_add(1);
        w1 |= ws & (!((((len as u32).wrapping_sub(sch - 1)) >> 31) as u32)).wrapping_add(1);
        w2 |= ws & (!((((len as u32).wrapping_sub(sch - 2)) >> 31) as u32)).wrapping_add(1);

        w2 |= ((w2 & 0x40000000) as u32) << 1;
        let xl: u32 = (w0 >> (scl - 1)) | (w1 << (32 - scl));
        let xh: u32 = (w1 >> scl) | (w2 << (31 - scl));
        d[u] = (xl as u64) | ((xh as u64) << 32);
        f_index += 1;
    }
}

#[inline(always)]
fn divrev31(x: u32) -> (u32, u32) {
    let qq = (x * 67651u32) >> 21;
    (qq, x - 31 * qq)
}