#[cfg(test)]
mod tests {
    use ntru_gen::zint31::{zint_add_mul_small, zint_mod_small_unsigned, zint_mul_small, zint_norm_zero, zint_rebuild_crt};
    use ntru_gen_c::zint31::{ntrugen_rebuild_CRT, ntrugen_zint_add_mul_small, ntrugen_zint_mod_small_unsigned, ntrugen_zint_mul_small, ntrugen_zint_norm_zero};
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
            let stride = rng.gen_range(1..5);
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
            let stride: usize = rng.gen_range(0..10);
            let mut x: [u32; 1048] = core::array::from_fn(|_| rng.gen::<u32>());
            let xc: [u32; 1048] = x.clone();
            let y: [u32; 1048] = core::array::from_fn(|_| rng.gen::<u32>());
            zint_add_mul_small(&mut x, 100, stride, &y, s);
            unsafe { ntrugen_zint_add_mul_small(xc.as_ptr(), 100, stride, y.as_ptr(), s) };
            assert_eq!(x, xc);
        }
    }

    #[test]
    fn zint_norm_zero_test() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let s: u32 = rand::random();
            let stride: usize = 1;
            let mut x: [u32; 20] = core::array::from_fn(|_| rng.gen::<u32>());
            let xc: [u32; 20] = x.clone();
            let p: [u32; 20] = core::array::from_fn(|_| rng.gen::<u32>());
            zint_norm_zero(&mut x, 19, stride, &p);
            unsafe { ntrugen_zint_norm_zero(xc.as_ptr(), 19, stride, p.as_ptr()) };
            assert_eq!(x, xc);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_zint_rebuild_CRT() {
        for _ in 0..200 {
             for stride in 1usize..3 {
                let mut rng = rand::thread_rng();
                let mut xx: [u32; 2048] = [10; 2048];
                let xxc: [u32; 2048] = [10; 2048];
                let num: usize = rng.gen_range(0..10);
                let len: usize = 100;
                let mut tmp: [u32; 5120] = [0; 5120];
                let tmp_c: [u32; 5120] = [0; 5120];
                zint_rebuild_crt(&mut xx, len, stride, num, false, &mut tmp);
                unsafe { ntrugen_rebuild_CRT(xxc.as_ptr(), len, stride, num, 0, tmp_c.as_ptr()) };
                assert_eq!(tmp, tmp_c);
                assert_eq!(xx, xxc);
            }
        }
        for _ in 0..200 {
            for stride in 1usize..3 {
                let mut rng = rand::thread_rng();
                let mut xx: [u32; 2048] = core::array::from_fn(|_| rng.gen::<u32>());
                let xxc: [u32; 2048] = xx.clone();
                let num: usize = rng.gen_range(0..10);
                let len: usize = 100;
                let mut tmp: [u32; 5120] = core::array::from_fn(|_| rng.gen::<u32>());
                let tmp_c: [u32; 5120] = tmp.clone();
                zint_rebuild_crt(&mut xx, len, stride, num, true, &mut tmp);
                unsafe { ntrugen_rebuild_CRT(xxc.as_ptr(), len, stride, num, 1, tmp_c.as_ptr()) };
                assert_eq!(tmp, tmp_c);
                assert_eq!(xx, xxc);
            }
        }
    }
}