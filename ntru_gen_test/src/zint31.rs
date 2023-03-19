#[cfg(test)]
mod tests {
    use ntru_gen::zint31::{zint_add_mul_small, zint_mod_small_unsigned, zint_mul_small};
    use ntru_gen_c::zint31::{ntrugen_zint_add_mul_small, ntrugen_zint_mod_small_unsigned, ntrugen_zint_mul_small};
    use rand::Rng;

    const P: u32 = 12289;

    #[test]
    fn zint_mul_small_test() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let x: u32 = rand::random();
            let mut m: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let mc = m.clone();
            zint_mul_small(&mut m, 1024, x);
            unsafe { ntrugen_zint_mul_small(mc.as_ptr(), 1024, x); }
            assert_eq!(m, mc);
        }
    }

    #[test]
    fn zint_mod_small_unsigned_test() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let p0i: u32 = rand::random();
            let r2: u32 = rand::random();
            let stride = 4;
            let mut m: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let x = zint_mod_small_unsigned(&mut m, 1024 / stride, stride, P, p0i, r2);
            let xc = unsafe { ntrugen_zint_mod_small_unsigned(m.as_ptr(), 1024 / stride, stride, P, p0i, r2) };
            assert_eq!(x, xc);
        }
    }

    #[test]
    fn zint_add_mul_small_test() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let s: u32 = rand::random();
            let stride: usize = 1;
            let mut x: [u32; 20] = core::array::from_fn(|_| rng.gen::<u32>());
            let xc: [u32; 20] = x.clone();
            let y: [u32; 20] = core::array::from_fn(|_| rng.gen::<u32>());
            zint_add_mul_small(&mut x, 19, stride, &y, s);
            unsafe { ntrugen_zint_add_mul_small(xc.as_ptr(), 19, stride, y.as_ptr(), s) };
            assert_eq!(x, xc);
        }
    }
}