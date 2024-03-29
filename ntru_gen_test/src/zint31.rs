#[cfg(test)]
mod tests {
    use ntru_gen::zint31::{zint_add_mul_small, zint_add_scaled_mul_small, zint_bezout, zint_mod_small_unsigned, zint_mul_small, zint_norm_zero, zint_rebuild_crt, zint_sub_scaled};
    use ntru_gen_c::zint31::{ntrugen_rebuild_CRT, ntrugen_zint_add_mul_small, ntrugen_zint_add_scaled_mul_small, ntrugen_zint_bezout, ntrugen_zint_mod_small_unsigned, ntrugen_zint_mul_small, ntrugen_zint_norm_zero, ntrugen_zint_sub_scaled};
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

    #[test]
    fn test_zint_bezout() {
        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            let mut u: [u32; 300] = core::array::from_fn(|_| rng.gen::<u32>());
            let u_c: [u32; 300] = u.clone();
            let mut v: [u32; 300] = core::array::from_fn(|_| rng.gen::<u32>());
            let v_c: [u32; 300] = v.clone();
            let x: [u32; 300] = core::array::from_fn(|_| rng.gen::<u32>());
            let x_c: [u32; 300] = x.clone();
            let y: [u32; 300] = core::array::from_fn(|_| rng.gen::<u32>());
            let y_c: [u32; 300] = y.clone();
            let mut tmp: [u32; 300 * 4] = [0; 1200];
            let tmp_c: [u32; 300 * 4] = tmp.clone();
            let len: usize = u.len();
            let res = zint_bezout(&mut u, &mut v, &x, &y, len, &mut tmp);
            let c_res = unsafe { ntrugen_zint_bezout(u_c.as_ptr(), v_c.as_ptr(), x_c.as_ptr(), y_c.as_ptr(), len, tmp_c.as_ptr()) };
            assert_eq!(x, x_c, "x");
            assert_eq!(y, y_c, "y");
            assert_eq!(res, c_res, "res");
            assert_eq!(tmp, tmp_c, "tmp");
            assert_eq!(v, v_c, "v");
            assert_eq!(u, u_c, "u");
        }
    }

    #[test]
    fn test_zint_sub_scaled() {
        for _ in 0..200 {
            let mut rng = rand::thread_rng();
            let mut x: [u32; 2048] = core::array::from_fn(|_| rng.gen::<u32>());
            let x_c: [u32; 2048] = x.clone();
            let xlen: usize = x.len();
            let y: [u32; 2048] = core::array::from_fn(|_| rng.gen::<u32>());
            let y_c: [u32; 2048] = y.clone();
            let ylen: usize = 1024;
            let sch: u32 = rand::random();
            let scl: u32 = rand::random();
            let stride = 2;
            zint_sub_scaled(&mut x, xlen, &y, ylen, stride, sch as usize, scl);
            unsafe { ntrugen_zint_sub_scaled(x_c.as_ptr(), xlen, y_c.as_ptr(), ylen, stride, sch, scl) };
            assert_eq!(x, x_c);
            assert_eq!(y, y_c);
        }
    }

    #[test]
    fn test_zint_add_scaled_mul_small() {
        for _ in 0..200 {
            let mut rng = rand::thread_rng();
            let mut x: [u32; 2048] = core::array::from_fn(|_| rng.gen::<u32>());
            let x_c: [u32; 2048] = x.clone();
            let k: i32 = rand::random();
            let xlen: usize = x.len();
            let y: [u32; 2048] = core::array::from_fn(|_| rng.gen::<u32>());
            let y_c: [u32; 2048] = y.clone();
            let ylen: usize = 1024;
            let sch: u32 = rand::random();
            let scl: u32 = rand::random();
            let stride = 2;
            zint_add_scaled_mul_small(&mut x, xlen, &y, ylen, stride, k, sch as usize, scl);
            unsafe { ntrugen_zint_add_scaled_mul_small(x_c.as_ptr(), xlen, y_c.as_ptr(), ylen, stride, k, sch, scl) };
            assert_eq!(x, x_c);
            assert_eq!(y, y_c);
        }
    }
}