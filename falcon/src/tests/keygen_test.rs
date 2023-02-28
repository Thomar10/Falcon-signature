#[cfg(test)]
pub(crate) mod tests {
    use rand::Rng;

    use crate::falcon_c::keygen_c::{falcon_inner_keygen, get_rng_u64_func, make_fg_func, make_fg_step_func, mkgauss_func, modp_add_func, modp_div_func, modp_iNTT2_ext_func, modp_mkgm2_func, modp_montymul_func, modp_ninv31_func, modp_norm_func, modp_NTT2_ext_func, modp_poly_rec_res_func, modp_R2_func, modp_R_func, modp_Rx_func, modp_set_func, modp_sub_func, poly_big_to_fp_func, poly_big_to_small_func, poly_small_mkgauss_func, poly_small_sqnorm_func, poly_small_to_fp_func, poly_sub_scaled_func, poly_sub_scaled_ntt_func, small_prime, solve_NTRU_binary_depth0_func, solve_NTRU_binary_depth1_func, solve_NTRU_deepest_func, solve_NTRU_func, solve_NTRU_intermediate_func, zint_add_mul_small_func, zint_add_scaled_mul_small_func, zint_bezout_func, zint_co_reduce_func, zint_co_reduce_mod_func, zint_finish_mod_func, zint_mod_small_signed_func, zint_mod_small_unsigned_func, zint_mul_small_func, zint_negate_func, zint_norm_zero_func, zint_one_to_plain_func, zint_rebuild_CRT_func, zint_sub_func, zint_sub_scaled_func};
    use crate::falcon_c::shake_c::{falcon_inner_i_shake256_init, falcon_inner_i_shake256_inject, InnerShake256Context as InnerShake256ContextC, St as StC};
    use crate::falcon_tmpsize_keygen;
    use crate::keygen::{get_rng_u64, keygen, make_fg, make_fg_index, make_fg_pointer, make_fg_step, make_fg_step_pointer, mkgauss, modp_add, modp_div, modp_iNTT2_ext, modp_mkgm2, modp_montymul, modp_ninv31, modp_norm, modp_NTT2_ext, modp_poly_rec_res, modp_R, modp_R2, modp_Rx, modp_set, modp_sub, poly_big_to_fp, poly_big_to_small, poly_small_mkgauss, poly_small_sqnorm, poly_small_to_fp, poly_sub_scaled, poly_sub_scaled_ntt, PRIMES, solve_ntru, solve_ntru_binary_depth0, solve_ntru_binary_depth1, solve_ntru_deepest, solve_ntru_intermediate_point, zint_add_mul_small, zint_add_scaled_mul_small, zint_bezout, zint_co_reduce, zint_co_reduce_mod, zint_finish_mod, zint_mod_small_signed, zint_mod_small_unsigned, zint_mul_small, zint_negate, zint_norm_zero, zint_one_to_plain, zint_rebuild_CRT, zint_sub, zint_sub_scaled};
    use crate::shake::{i_shake256_init, i_shake256_inject, InnerShake256Context};

    #[test]
    fn test_modp_set() {
        for _ in 0..2000 {
            let x: i32 = rand::random();
            let p: u32 = rand::random();
            let res = modp_set(x, p);
            let res_c = unsafe { modp_set_func(x, p) };
            assert_eq!(res, res_c);
        }
    }

    #[test]
    fn test_modp_norm() {
        for _ in 0..2000 {
            let x: u32 = rand::random();
            let p: u32 = rand::random();
            let res = modp_norm(x, p);
            let res_c = unsafe { modp_norm_func(x, p) };
            assert_eq!(res, res_c);
        }
    }

    #[test]
    fn test_modp_ninv31() {
        for _ in 0..2000 {
            let p: u32 = rand::random();
            let res = modp_ninv31(p);
            let res_c = unsafe { modp_ninv31_func(p) };
            assert_eq!(res, res_c);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_modp_R() {
        for _ in 0..2000 {
            let p: u32 = rand::random();
            let res = modp_R(p);
            let res_c = unsafe { modp_R_func(p) };
            assert_eq!(res, res_c);
        }
    }

    #[test]
    fn test_modp_add() {
        for _ in 0..2000 {
            let p: u32 = rand::random();
            let a: u32 = rand::random();
            let b: u32 = rand::random();
            let res = modp_add(a, b, p);
            let res_c = unsafe { modp_add_func(a, b, p) };
            assert_eq!(res, res_c);
        }
    }

    #[test]
    fn test_modp_sub() {
        for _ in 0..2000 {
            let p: u32 = rand::random();
            let a: u32 = rand::random();
            let b: u32 = rand::random();
            let res = modp_sub(a, b, p);
            let res_c = unsafe { modp_sub_func(a, b, p) };
            assert_eq!(res, res_c);
        }
    }

    #[test]
    fn test_modp_montymul() {
        for _ in 0..2000 {
            let p: u32 = rand::random();
            let p0i: u32 = rand::random();
            let a: u32 = rand::random();
            let b: u32 = rand::random();
            let res = modp_montymul(a, b, p, p0i);
            let res_c = unsafe { modp_montymul_func(a, b, p, p0i) };
            assert_eq!(res, res_c);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_modp_R2() {
        for _ in 0..2000 {
            let p: u32 = rand::random();
            let p0i: u32 = rand::random();
            let res = modp_R2(p, p0i);
            let res_c = unsafe { modp_R2_func(p, p0i) };
            assert_eq!(res, res_c);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_modp_Rx() {
        for _ in 0..2000 {
            let p: u32 = rand::random();
            let p0i: u32 = rand::random();
            let mut x: u16 = rand::random();
            x = x >> 5;
            if x == 0 {
                x = 1;
            }
            let R2: u32 = rand::random();
            let res = modp_Rx(x as u32, p, p0i, R2);
            let res_c = unsafe { modp_Rx_func(x as u32, p, p0i, R2) };
            assert_eq!(res, res_c);
        }
    }

    #[test]
    fn test_modp_div() {
        for _ in 0..2000 {
            let a: u32 = rand::random();
            let b: u32 = rand::random();
            let p: u32 = rand::random();
            let p0i: u32 = rand::random();
            let r: u32 = rand::random();
            let res = modp_div(a, b, p, p0i, r);
            let res_c = unsafe { modp_div_func(a, b, p, p0i, r) };
            assert_eq!(res, res_c);
        }
    }

    #[test]
    fn test_modp_mkgm2() {
        for _ in 0..200 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut gm: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
                let gm_c: [u32; 1024] = gm.clone();
                let mut igm: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
                let igm_c: [u32; 1024] = igm.clone();
                let p: u32 = rand::random();
                let p0i: u32 = rand::random();
                let g: u32 = rand::random();
                modp_mkgm2(&mut gm, &mut igm, logn, g, p, p0i);
                unsafe { modp_mkgm2_func(gm_c.as_ptr(), igm_c.as_ptr(), logn, g, p, p0i) };
                assert_eq!(gm, gm_c);
                assert_eq!(igm, igm_c);
            }
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_modp_NTT2_ext() {
        for _ in 0..50 {
            for logn in 1..10 {
                for stride in 1usize..2 {
                    let mut rng = rand::thread_rng();
                    let mut a: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
                    let a_c: [u32; 1024] = a.clone();
                    let mut gm: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
                    let gm_c: [u32; 1024] = gm.clone();
                    let p: u32 = rand::random();
                    let p0i: u32 = rand::random();
                    modp_NTT2_ext(&mut a, stride, &mut gm, logn, p, p0i);
                    unsafe { modp_NTT2_ext_func(a_c.as_ptr(), stride, gm_c.as_ptr(), logn, p, p0i) };
                    assert_eq!(a, a_c);
                }
            }
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_modp_iNTT2_ext() {
        for _ in 0..50 {
            for logn in 1..10 {
                for stride in 1usize..2 {
                    let mut rng = rand::thread_rng();
                    let mut a: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
                    let a_c: [u32; 1024] = a.clone();
                    let mut gm: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
                    let gm_c: [u32; 1024] = gm.clone();
                    let p: u32 = rand::random();
                    let p0i: u32 = rand::random();
                    modp_iNTT2_ext(&mut a, stride, &mut gm, logn, p, p0i);
                    unsafe { modp_iNTT2_ext_func(a_c.as_ptr(), stride, gm_c.as_ptr(), logn, p, p0i) };
                    assert_eq!(a, a_c);
                }
            }
        }
    }

    #[test]
    fn test_modp_poly_rec_res() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut f: [u32; 2048] = core::array::from_fn(|_| rng.gen::<u32>());
                let f_c: [u32; 2048] = f.clone();
                let p: u32 = rand::random();
                let p0i: u32 = rand::random();
                let r2: u32 = rand::random();
                modp_poly_rec_res(&mut f, logn, p, p0i, r2);
                unsafe { modp_poly_rec_res_func(f_c.as_ptr(), logn, p, p0i, r2) };
                assert_eq!(f, f_c);
            }
        }
    }

    #[test]
    fn test_zint_sub() {
        for _ in 0..200 {
            let mut rng = rand::thread_rng();
            let mut a: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let a_c: [u32; 1024] = a.clone();
            let mut b: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let b_c: [u32; 1024] = b.clone();
            let len: usize = a.len();
            let ctl: bool = rand::random();
            let res = zint_sub(&mut a, &mut b, len, ctl as u32);
            let c_res = unsafe { zint_sub_func(a_c.as_ptr(), b_c.as_ptr(), len, ctl as u32) };
            assert_eq!(res, c_res);
        }
    }

    #[test]
    fn test_zint_mul_small() {
        for _ in 0..200 {
            let mut rng = rand::thread_rng();
            let mut m: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let m_c: [u32; 1024] = m.clone();
            let len: usize = m.len();
            let x: u32 = rand::random();
            let res = zint_mul_small(&mut m, len, x);
            let c_res = unsafe { zint_mul_small_func(m_c.as_ptr(), len, x) };
            assert_eq!(res, c_res);
        }
    }

    #[test]
    fn test_zint_mod_small_unsigned() {
        for _ in 0..200 {
            let mut rng = rand::thread_rng();
            let mut d: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let d_c: [u32; 1024] = d.clone();
            let len: usize = d.len();
            let p: u32 = rand::random();
            let p0i: u32 = rand::random();
            let r2: u32 = rand::random();
            let res = zint_mod_small_unsigned(&mut d, len, p, p0i, r2);
            let c_res = unsafe { zint_mod_small_unsigned_func(d_c.as_ptr(), len, p, p0i, r2) };
            assert_eq!(res, c_res);
        }
    }

    #[test]
    fn test_zint_mod_small_signed() {
        for _ in 0..200 {
            let mut rng = rand::thread_rng();
            let mut d: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let len: usize = d.len();
            let p: u32 = rand::random();
            let p0i: u32 = rand::random();
            let r2: u32 = rand::random();
            let rx: u32 = rand::random();
            let res = zint_mod_small_signed(&mut d, len, p, p0i, r2, rx);
            let c_res = unsafe { zint_mod_small_signed_func(d.as_ptr(), len, p, p0i, r2, rx) };
            assert_eq!(res, c_res);
        }
    }

    #[test]
    fn test_zint_add_mul_small() {
        for _ in 0..200 {
            let mut rng = rand::thread_rng();
            let mut x: [u32; 1025] = core::array::from_fn(|_| rng.gen::<u32>());
            let x_c: [u32; 1025] = x.clone();
            let mut y: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let y_c: [u32; 1024] = y.clone();
            let len: usize = y.len();
            let s: u32 = rand::random();
            let res = zint_add_mul_small(&mut x, &mut y, len, s);
            let c_res = unsafe { zint_add_mul_small_func(x_c.as_ptr(), y_c.as_ptr(), len, s) };
            assert_eq!(res, c_res);
        }
    }

    #[test]
    fn test_zint_norm_zero() {
        for _ in 0..200 {
            let mut rng = rand::thread_rng();
            let mut x: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let x_c: [u32; 1024] = x.clone();
            let mut p: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let p_c: [u32; 1024] = p.clone();
            let len: usize = x.len();
            let res = zint_norm_zero(&mut x, &mut p, len);
            let c_res = unsafe { zint_norm_zero_func(x_c.as_ptr(), p_c.as_ptr(), len) };
            assert_eq!(res, c_res);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_zint_rebuild_CRT() {
        for _ in 0..50 {
            for stride in 1usize..3 {
                let mut rng = rand::thread_rng();
                let mut xx: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
                let num: u64 = rng.gen_range(0..10);
                let len: usize = 100;
                let mut tmp: [u32; 512] = core::array::from_fn(|_| rng.gen::<u32>());
                let tmp_c: [u32; 512] = tmp.clone();
                zint_rebuild_CRT(&mut xx, len, stride, num, &PRIMES, false, &mut tmp);
                unsafe { zint_rebuild_CRT_func(xx.as_ptr(), len, stride, num, PRIMES_C.as_ptr(), 0, tmp_c.as_ptr()) };
                assert_eq!(tmp, tmp_c);
            }
        }
        for _ in 0..200 {
            for stride in 1usize..5 {
                let mut rng = rand::thread_rng();
                let mut xx: [u32; 2048] = core::array::from_fn(|_| rng.gen::<u32>());
                let num: u64 = rng.gen_range(0..10);
                let len: usize = 100;
                let mut tmp: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
                let tmp_c: [u32; 1024] = tmp.clone();
                zint_rebuild_CRT(&mut xx, len, stride, num, &PRIMES, true, &mut tmp);
                unsafe { zint_rebuild_CRT_func(xx.as_ptr(), len, stride, num, PRIMES_C.as_ptr(), 1, tmp_c.as_ptr()) };
                assert_eq!(tmp, tmp_c);
            }
        }
    }

    #[test]
    fn test_zint_negate() {
        for _ in 0..200 {
            let mut rng = rand::thread_rng();
            let mut a: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let a_c: [u32; 1024] = a.clone();
            let len: usize = a.len();
            let ctl: u32 = rand::random();
            zint_negate(&mut a, len, ctl);
            unsafe { zint_negate_func(a_c.as_ptr(), len, ctl) };
            assert_eq!(a, a_c);
        }
    }

    #[test]
    fn test_zint_co_reduce() {
        for _ in 0..200 {
            let mut rng = rand::thread_rng();
            let mut a: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let a_c: [u32; 1024] = a.clone();
            let mut b: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let b_c: [u32; 1024] = b.clone();
            let len: usize = a.len();
            let xa: i64 = rand::random();
            let xb: i64 = rand::random();
            let ya: i64 = rand::random();
            let yb: i64 = rand::random();
            let res = zint_co_reduce(&mut a, &mut b, len, xa, xb, ya, yb);
            let c_res = unsafe { zint_co_reduce_func(a_c.as_ptr(), b_c.as_ptr(), len, xa, xb, ya, yb) };
            assert_eq!(res, c_res);
        }
    }

    #[test]
    fn test_zint_finish_mod() {
        for _ in 0..200 {
            let mut rng = rand::thread_rng();
            let mut a: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let a_c: [u32; 1024] = a.clone();
            let mut m: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let m_c: [u32; 1024] = m.clone();
            let len: usize = a.len();
            let neg: u32 = rand::random();
            zint_finish_mod(&mut a, len, &mut m, neg);
            unsafe { zint_finish_mod_func(a_c.as_ptr(), len, m_c.as_ptr(), neg) };
            assert_eq!(a, a_c);
        }
    }

    #[test]
    fn test_zint_co_reduce_mod() {
        for _ in 0..200 {
            let mut rng = rand::thread_rng();
            let mut a: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let a_c: [u32; 1024] = a.clone();
            let mut b: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let b_c: [u32; 1024] = b.clone();
            let mut m: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let m_c: [u32; 1024] = m.clone();
            let len: usize = a.len();
            let m0i: u32 = rand::random();
            let xa: i64 = rand::random();
            let xb: i64 = rand::random();
            let ya: i64 = rand::random();
            let yb: i64 = rand::random();

            zint_co_reduce_mod(&mut a, &mut b, &mut m, len, m0i, xa, xb, ya, yb);
            unsafe { zint_co_reduce_mod_func(a_c.as_ptr(), b_c.as_ptr(), m_c.as_ptr(), len, m0i, xa, xb, ya, yb) };
            assert_eq!(a, a_c);
            assert_eq!(b, b_c);
        }
    }

    #[test]
    fn test_zint_bezout() {
        for _ in 0..1000 {
            let mut rng = rand::thread_rng();
            let mut u: [u32; 300] = core::array::from_fn(|_| rng.gen::<u32>());
            let u_c: [u32; 300] = u.clone();
            let mut v: [u32; 300] = core::array::from_fn(|_| rng.gen::<u32>());
            let v_c: [u32; 300] = v.clone();
            let mut x: [u32; 300] = core::array::from_fn(|_| rng.gen::<u32>());
            let x_c: [u32; 300] = x.clone();
            let mut y: [u32; 300] = core::array::from_fn(|_| rng.gen::<u32>());
            let y_c: [u32; 300] = y.clone();
            let mut tmp: [u32; 300 * 4] = [0; 1200];
            let tmp_c: [u32; 300 * 4] = tmp.clone();
            let len: usize = u.len();
            let res = zint_bezout(&mut u, &mut v, &mut x, &mut y, len, &mut tmp);
            let c_res = unsafe { zint_bezout_func(u_c.as_ptr(), v_c.as_ptr(), x_c.as_ptr(), y_c.as_ptr(), len, tmp_c.as_ptr()) };
            assert_eq!(x, x_c);
            assert_eq!(y, y_c);
            assert_eq!(res, c_res);
            assert_eq!(tmp, tmp_c);
            assert_eq!(v, v_c);
            assert_eq!(u, u_c);
        }
    }

    #[test]
    fn test_zint_add_scaled_mul_small() {
        for _ in 0..200 {
            let mut rng = rand::thread_rng();
            let mut x: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let x_c: [u32; 1024] = x.clone();
            let xlen: usize = x.len();
            let mut y: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let y_c: [u32; 1024] = y.clone();
            let ylen: usize = y.len();
            let k: i32 = rand::random();
            let sch: u32 = rand::random();
            let scl: u32 = rand::random();
            zint_add_scaled_mul_small(&mut x, xlen, &mut y, ylen, k, sch as usize, scl);
            unsafe { zint_add_scaled_mul_small_func(x_c.as_ptr(), xlen, y_c.as_ptr(), ylen, k, sch, scl) };
            assert_eq!(x, x_c);
            assert_eq!(y, y_c);
        }
    }

    #[test]
    fn test_zint_sub_scaled() {
        for _ in 0..200 {
            let mut rng = rand::thread_rng();
            let mut x: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let x_c: [u32; 1024] = x.clone();
            let xlen: usize = x.len();
            let mut y: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let y_c: [u32; 1024] = y.clone();
            let ylen: usize = y.len();
            let sch: u32 = rand::random();
            let scl: u32 = rand::random();
            zint_sub_scaled(&mut x, xlen, &mut y, ylen, sch as usize, scl);
            unsafe { zint_sub_scaled_func(x_c.as_ptr(), xlen, y_c.as_ptr(), ylen, sch, scl) };
            assert_eq!(x, x_c);
            assert_eq!(y, y_c);
        }
    }

    #[test]
    fn test_zint_one_to_plain() {
        for _ in 0..200 {
            let mut rng = rand::thread_rng();
            let mut x: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
            let x_c: [u32; 1024] = x.clone();
            let res = zint_one_to_plain(&mut x);
            let c_res = unsafe { zint_one_to_plain_func(x_c.as_ptr()) };
            assert_eq!(res, c_res);
        }
    }


    #[test]
    fn test_poly_big_to_fp() {
        for _ in 0..200 {
            for fstride in 1usize..5 {
                for logn in 1..3 {
                    let mut rng = rand::thread_rng();
                    let mut d: [u64; 1024] = [0; 1024];
                    let d_c: [u64; 1024] = d.clone();
                    let mut f: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
                    let f_c: [u32; 1024] = f.clone();
                    poly_big_to_fp(&mut d, &mut f, 5, fstride, logn);
                    unsafe { poly_big_to_fp_func(d_c.as_ptr(), f_c.as_ptr(), 5, fstride, logn) };
                    println!("{:?}", d);
                    assert_eq!(d, d_c);
                }
            }
        }
    }

    #[test]
    fn test_poly_big_to_small() {
        for _ in 0..1000 {
            for logn in 1..5 {
                let mut rng = rand::thread_rng();
                let mut d: [i8; 1024] = [0; 1024];
                let d_c: [i8; 1024] = d.clone();
                let mut s: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
                let s_c: [u32; 1024] = s.clone();
                let lim: i32 = rand::random();
                let res = poly_big_to_small(&mut d, &mut s, lim, logn);
                let res_c = unsafe { poly_big_to_small_func(d_c.as_ptr(), s_c.as_ptr(), lim, logn) };
                assert_eq!(res, res_c);
                assert_eq!(d, d_c);
            }
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_poly_sub_scaled() {
        for _ in 0..100 {
            for logn in 1..5 {
                for stride in 1..5 {
                    let mut rng = rand::thread_rng();
                    let mut F: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
                    let F_c: [u32; 1024] = F.clone();
                    let mut f: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
                    let f_c: [u32; 1024] = f.clone();
                    let mut k: [i32; 1024] = core::array::from_fn(|_| rng.gen::<i32>());
                    let k_c: [i32; 1024] = k.clone();
                    let sch: u32 = rand::random();
                    let scl: u32 = rand::random();
                    let lengthF = 205;
                    let lengthf = 205;
                    poly_sub_scaled(&mut F, lengthF, stride, &mut f, lengthf, stride, &mut k, sch, scl, logn);
                    unsafe { poly_sub_scaled_func(F_c.as_ptr(), lengthF, stride, f_c.as_ptr(), lengthf, stride, k_c.as_ptr(), sch, scl, logn) };
                    assert_eq!(F, F_c);
                }
            }
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_poly_sub_scaled_ntt() {
        for _ in 0..100 {
            for logn in 1..3 {
                for stride in 1..3 {
                    let mut rng = rand::thread_rng();
                    let mut F: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
                    let F_c: [u32; 1024] = F.clone();
                    let mut f: [u32; 1024] = core::array::from_fn(|_| rng.gen::<u32>());
                    let f_c: [u32; 1024] = f.clone();
                    let mut k: [i32; 1024] = core::array::from_fn(|_| rng.gen::<i32>());
                    let k_c: [i32; 1024] = k.clone();
                    let mut tmp: [u32; 7 * 1024] = core::array::from_fn(|_| rng.gen::<u32>());
                    let tmp_c: [u32; 7 * 1024] = tmp.clone();
                    let sch: u32 = rand::random();
                    let scl: u32 = rand::random();
                    let lengthF = 205;
                    let lengthf = 205;
                    poly_sub_scaled_ntt(&mut F, lengthF, stride, &mut f, lengthf, stride, &mut k, sch, scl, logn, &mut tmp);
                    unsafe { poly_sub_scaled_ntt_func(F_c.as_ptr(), lengthF, stride, f_c.as_ptr(), lengthf, stride, k_c.as_ptr(), sch, scl, logn, tmp_c.as_ptr()) };
                    assert_eq!(F, F_c);
                }
            }
        }
    }

    #[test]
    fn test_get_rng_u64() {
        for _ in 0..1000 {
            let (mut sc_rust, sc_c) = init_shake_with_random_context();
            let res = get_rng_u64(&mut sc_rust);
            let res_c = unsafe { get_rng_u64_func(&sc_c) };
            assert_eq!(res, res_c);
        }
    }

    #[test]
    fn test_mkgauss() {
        for _ in 0..100 {
            for logn in 0..10 {
                let (mut sc_rust, sc_c) = init_shake_with_random_context();
                let res = mkgauss(&mut sc_rust, logn);
                let res_c = unsafe { mkgauss_func(&sc_c, logn) };
                assert_eq!(res, res_c);
            }
        }
    }

    #[test]
    fn test_poly_small_sqnorm() {
        for _ in 0..100 {
            for logn in 0..10 {
                let mut rng = rand::thread_rng();
                let mut f: [i8; 1024] = core::array::from_fn(|_| rng.gen::<i8>());
                let f_c: [i8; 1024] = f.clone();
                let res = poly_small_sqnorm(&mut f, logn);
                let res_c = unsafe { poly_small_sqnorm_func(f_c.as_ptr(), logn) };
                assert_eq!(res, res_c);
            }
        }
    }

    #[test]
    fn test_poly_small_to_fp() {
        for _ in 0..100 {
            for logn in 0..10 {
                let mut rng = rand::thread_rng();
                let mut x: [u64; 1024] = core::array::from_fn(|_| rng.gen::<u64>());
                let x_c: [u64; 1024] = x.clone();
                let mut f: [i8; 1024] = core::array::from_fn(|_| rng.gen::<i8>());
                let f_c: [i8; 1024] = f.clone();
                poly_small_to_fp(&mut x, &mut f, logn);
                unsafe { poly_small_to_fp_func(x_c.as_ptr(), f_c.as_ptr(), logn) };
                assert_eq!(x, x_c);
            }
        }
    }

    #[test]
    fn test_make_fg_step() {
        for _ in 0..10 {
            for logn in 1..10 {
                for depth in 0..3usize {
                    let mut rng = rand::thread_rng();
                    let mut data: [u32; 2048 * 4] = core::array::from_fn(|_| rng.gen::<u32>());
                    let data_c: [u32; 2048 * 4] = data.clone();
                    let in_ntt: bool = rand::random();
                    let in_out: bool = rand::random();
                    make_fg_step(&mut data, logn, depth, in_ntt, in_out);
                    unsafe { make_fg_step_func(data_c.as_ptr(), logn, depth, in_ntt, in_out) };
                    assert_eq!(data, data_c);
                }
            }
        }
    }

    #[test]
    fn test_make_fg_step_pointer() {
        for _ in 0..10 {
            for logn in 1..10 {
                for depth in 0..3usize {
                    let mut rng = rand::thread_rng();
                    let mut data: [u32; 2048 * 4] = core::array::from_fn(|_| rng.gen::<u32>());
                    let mut data_c: [u32; 2048 * 4] = data.clone();
                    let in_ntt: bool = rand::random();
                    let in_out: bool = rand::random();
                    make_fg_step(&mut data, logn, depth, in_ntt, in_out);
                    make_fg_step_pointer(data_c.as_mut_ptr(), logn, depth, in_ntt, in_out);
                    assert_eq!(data, data_c);
                }
            }
        }
    }

    #[test]
    fn test_make_fg() {
        for _ in 0..10 {
            for logn in 3..10 {
                for depth in 0..3 {
                    let mut rng = rand::thread_rng();
                    let mut data: [u32; 2048 * 4] = core::array::from_fn(|_| rng.gen::<u32>());
                    let data_c: [u32; 2048 * 4] = data.clone();
                    let mut f: [i8; 2048 * 4] = core::array::from_fn(|_| rng.gen::<i8>());
                    let f_c: [i8; 2048 * 4] = f.clone();
                    let mut g: [i8; 2048 * 4] = core::array::from_fn(|_| rng.gen::<i8>());
                    let g_c: [i8; 2048 * 4] = g.clone();
                    let in_out: bool = rand::random();
                    make_fg(&mut data, &mut f, &mut g, logn, depth, in_out);
                    unsafe { make_fg_func(data_c.as_ptr(), f_c.as_ptr(), g_c.as_ptr(), logn, depth, in_out) };
                    assert_eq!(data, data_c);
                }
            }
        }
    }

    #[test]
    fn test_make_fg_point() {
        for _ in 0..10 {
            for logn in 3..10 {
                for depth in 0..3 {
                    let mut rng = rand::thread_rng();
                    let mut data: [u32; 2048 * 4] = core::array::from_fn(|_| rng.gen::<u32>());
                    let mut data_c: [u32; 2048 * 4] = data.clone();
                    let mut f: [i8; 2048 * 4] = core::array::from_fn(|_| rng.gen::<i8>());
                    let mut f_c: [i8; 2048 * 4] = f.clone();
                    let mut g: [i8; 2048 * 4] = core::array::from_fn(|_| rng.gen::<i8>());
                    let mut g_c: [i8; 2048 * 4] = g.clone();
                    let in_out: bool = rand::random();
                    make_fg_pointer(data.as_mut_ptr(), f.as_mut_ptr(), g.as_mut_ptr(), logn, depth, in_out);
                    make_fg(&mut data_c, &mut f_c, &mut g_c, logn, depth, in_out);
                    assert_eq!(data, data_c);
                }
            }
        }
    }

    #[test]
    fn test_make_fg_index() {
        for _ in 0..10 {
            for logn in 3..10 {
                for depth in 0..3 {
                    let mut rng = rand::thread_rng();
                    let mut data: [u32; 2048 * 4] = core::array::from_fn(|_| rng.gen::<u32>());
                    let mut data_c: [u32; 2048 * 4] = data.clone();
                    let mut f: [i8; 2048 * 4] = core::array::from_fn(|_| rng.gen::<i8>());
                    let mut f_c: [i8; 2048 * 4] = f.clone();
                    let mut g: [i8; 2048 * 4] = core::array::from_fn(|_| rng.gen::<i8>());
                    let mut g_c: [i8; 2048 * 4] = g.clone();
                    let in_out: bool = rand::random();
                    make_fg(&mut data, &mut f, &mut g, logn, depth, in_out);
                    make_fg_index(&mut data_c, 0, &mut f_c, &mut g_c, logn, depth, in_out);

                    assert_eq!(data, data_c);
                }
            }
        }
    }


    #[test]
    fn test_solve_ntru_deepest() {
        for logn_top in 1..10 {
            let mut rng = rand::thread_rng();
            let mut tmp: [u32; 2048 * 4] = core::array::from_fn(|_| rng.gen::<u32>());
            let tmp_c: [u32; 2048 * 4] = tmp.clone();
            let mut f: [i8; 2048 * 4] = core::array::from_fn(|_| rng.gen::<i8>());
            let f_c: [i8; 2048 * 4] = f.clone();
            let mut g: [i8; 2048 * 4] = core::array::from_fn(|_| rng.gen::<i8>());
            let g_c: [i8; 2048 * 4] = g.clone();
            let res = solve_ntru_deepest(logn_top, f.as_mut_ptr(), g.as_mut_ptr(), tmp.as_mut_ptr());
            let res_c = unsafe { solve_NTRU_deepest_func(logn_top, f_c.as_ptr(), g_c.as_ptr(), tmp_c.as_ptr()) };
            assert_eq!(tmp, tmp_c);
            assert_eq!(res, res_c != 0);
        }
    }

    #[test]
    fn test_solve_ntru_intermediate() {
        for _ in 0..10 {
            for logn_top in 4..10 {
                for depth in 1..4 {
                    let mut rng = rand::thread_rng();
                    let mut tmp: [u32; 2048 * 4] = core::array::from_fn(|_| rng.gen::<u32>());
                    let tmp_c: [u32; 2048 * 4] = tmp.clone();
                    let mut f: [i8; 2048 * 4] = core::array::from_fn(|_| rng.gen::<i8>());
                    let f_c: [i8; 2048 * 4] = f.clone();
                    let mut g: [i8; 2048 * 4] = core::array::from_fn(|_| rng.gen::<i8>());
                    let g_c: [i8; 2048 * 4] = g.clone();
                    let res = solve_ntru_intermediate_point(logn_top, f.as_mut_ptr(), g.as_mut_ptr(), depth, tmp.as_mut_ptr());
                    let res_c = unsafe { solve_NTRU_intermediate_func(logn_top, f_c.as_ptr(), g_c.as_ptr(), depth, tmp_c.as_ptr()) };
                    assert_eq!(tmp, tmp_c);
                    assert_eq!(res, res_c != 0);
                }
            }
        }
    }

    #[test]
    fn test_solve_ntru_binary_depth1() {
        for _ in 0..10 {
            for logn_top in 2..10 {
                let mut rng = rand::thread_rng();
                let mut tmp: [u32; 2048 * 4] = core::array::from_fn(|_| rng.gen::<u32>());
                let tmp_c: [u32; 2048 * 4] = tmp.clone();
                let mut f: [i8; 2048 * 4] = core::array::from_fn(|_| rng.gen::<i8>());
                let f_c: [i8; 2048 * 4] = f.clone();
                let mut g: [i8; 2048 * 4] = core::array::from_fn(|_| rng.gen::<i8>());
                let g_c: [i8; 2048 * 4] = g.clone();
                let res = solve_ntru_binary_depth1(logn_top, f.as_mut_ptr(), g.as_mut_ptr(), tmp.as_mut_ptr());
                let res_c = unsafe { solve_NTRU_binary_depth1_func(logn_top, f_c.as_ptr(), g_c.as_ptr(), tmp_c.as_ptr()) };
                assert_eq!(tmp, tmp_c);
                assert_eq!(res, res_c != 0);
            }
        }
    }

    #[test]
    fn test_solve_ntru_binary_depth0() {
        for _ in 0..10 {
            for logn_top in 2..10 {
                let mut rng = rand::thread_rng();
                let mut tmp: [u32; 2048 * 4] = core::array::from_fn(|_| rng.gen::<u32>());
                let tmp_c: [u32; 2048 * 4] = tmp.clone();
                let mut f: [i8; 2048 * 4] = core::array::from_fn(|_| rng.gen::<i8>());
                let f_c: [i8; 2048 * 4] = f.clone();
                let mut g: [i8; 2048 * 4] = core::array::from_fn(|_| rng.gen::<i8>());
                let g_c: [i8; 2048 * 4] = g.clone();
                let res = solve_ntru_binary_depth0(logn_top, f.as_mut_ptr(), g.as_mut_ptr(), tmp.as_mut_ptr());
                let res_c = unsafe { solve_NTRU_binary_depth0_func(logn_top, f_c.as_ptr(), g_c.as_ptr(), tmp_c.as_ptr()) };
                assert_eq!(tmp, tmp_c);
                assert_eq!(res, res_c != 0);
            }
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_solve_ntru() {
        for _ in 0..10 {
            for logn_top in 2..10 {
                let mut rng = rand::thread_rng();
                let mut tmp: [u32; 2048 * 4] = core::array::from_fn(|_| rng.gen::<u32>());
                let tmp_c: [u32; 2048 * 4] = tmp.clone();
                let mut F: [i8; 2048 * 4] = core::array::from_fn(|_| rng.gen::<i8>());
                let F_c: [i8; 2048 * 4] = F.clone();
                let mut G: [i8; 2048 * 4] = core::array::from_fn(|_| rng.gen::<i8>());
                let G_c: [i8; 2048 * 4] = G.clone();
                let mut f: [i8; 2048 * 4] = core::array::from_fn(|_| rng.gen::<i8>());
                let f_c: [i8; 2048 * 4] = f.clone();
                let mut g: [i8; 2048 * 4] = core::array::from_fn(|_| rng.gen::<i8>());
                let g_c: [i8; 2048 * 4] = g.clone();
                let lim: i32 = i32::MAX;
                let res = solve_ntru(logn_top, F.as_mut_ptr(), G.as_mut_ptr(), f.as_mut_ptr(), g.as_mut_ptr(), lim, tmp.as_mut_ptr());
                let res_c = unsafe { solve_NTRU_func(logn_top, F_c.as_ptr(), G_c.as_ptr(), f_c.as_ptr(), g_c.as_ptr(), lim, tmp_c.as_ptr()) };
                assert_eq!(tmp, tmp_c);
                assert_eq!(res, res_c != 0);
                assert_eq!(F, F_c);
                assert_eq!(G, G_c);
                assert_eq!(f, f_c);
                assert_eq!(g, g_c);
            }
        }
    }

    #[test]
    fn test_poly_small_mkgauss() {
        for _ in 0..100 {
            for logn in 2..10 {
                let (mut rng_rust, rng_c) = init_shake_with_random_context();
                let mut f: [i8; 2048] = [0; 2048];
                let f_c: [i8; 2048] = [0; 2048];
                poly_small_mkgauss(&mut rng_rust, f.as_mut_ptr(), logn);
                unsafe { poly_small_mkgauss_func(&rng_c, f_c.as_ptr(), logn); }
                assert_eq!(f, f_c);
            }
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_keygen() {
        for _ in 0..5 {
            for logn in 1..11 {
                let buffer_size = falcon_tmpsize_keygen!(logn);
                let (mut rng_rust, rng_c) = init_shake_with_random_context();
                let mut h: Vec<u16> = vec![0u16; buffer_size];
                let h_c: Vec<u16> = vec![0u16; buffer_size];
                let mut tmp: Vec<u8> = vec![0; buffer_size];
                let tmp_c: Vec<u8> = vec![0; buffer_size];
                let mut F: Vec<i8> = vec![0; buffer_size];
                let F_c: Vec<i8> = vec![0; buffer_size];
                let mut G: Vec<i8> = vec![0; buffer_size];
                let G_c: Vec<i8> = vec![0; buffer_size];
                let mut f: Vec<i8> = vec![0; buffer_size];
                let f_c: Vec<i8> = vec![0; buffer_size];
                let mut g: Vec<i8> = vec![0; buffer_size];
                let g_c: Vec<i8> = vec![0; buffer_size];
                keygen(&mut rng_rust, f.as_mut_ptr(), g.as_mut_ptr(), F.as_mut_ptr(), G.as_mut_ptr(), h.as_mut_ptr(), logn, tmp.as_mut_ptr());
                unsafe { falcon_inner_keygen(&rng_c, f_c.as_ptr(), g_c.as_ptr(), F_c.as_ptr(), G_c.as_ptr(), h_c.as_ptr(), logn, tmp_c.as_ptr()); }
                assert_eq!(f, f_c);
                assert_eq!(g, g_c);
                assert_eq!(G, G_c);
                assert_eq!(F, F_c);
                assert_eq!(h, h_c);
                assert_eq!(tmp, tmp_c);
            }
        }
    }


    pub fn init_shake_with_random_context() -> (InnerShake256Context, InnerShake256ContextC) {
        let random_state: [u64; 25] = rand::random();
        let random_dptr: u64 = rand::random();
        let mut sc_rust = InnerShake256Context { st: random_state, dptr: random_dptr };
        let sc_c = InnerShake256ContextC { st: StC { a: random_state.clone() }, dptr: random_dptr };
        unsafe { falcon_inner_i_shake256_init(&sc_c as *const InnerShake256ContextC) }
        i_shake256_init(&mut sc_rust);
        let input_rust: [u8; 25] = rand::random();
        let input_c: [u8; 25] = input_rust.clone();
        i_shake256_inject(&mut sc_rust, &input_rust);
        unsafe { falcon_inner_i_shake256_inject(&sc_c as *const InnerShake256ContextC, input_c.as_ptr(), input_c.len() as u64) }
        (sc_rust, sc_c)
    }


    pub(crate) static PRIMES_C: [small_prime; 522] = [
        small_prime { p: 2147473409, g: 383167813, s: 10239 },
        small_prime { p: 2147389441, g: 211808905, s: 471403745 },
        small_prime { p: 2147387393, g: 37672282, s: 1329335065 },
        small_prime { p: 2147377153, g: 1977035326, s: 968223422 },
        small_prime { p: 2147358721, g: 1067163706, s: 132460015 },
        small_prime { p: 2147352577, g: 1606082042, s: 598693809 },
        small_prime { p: 2147346433, g: 2033915641, s: 1056257184 },
        small_prime { p: 2147338241, g: 1653770625, s: 421286710 },
        small_prime { p: 2147309569, g: 631200819, s: 1111201074 },
        small_prime { p: 2147297281, g: 2038364663, s: 1042003613 },
        small_prime { p: 2147295233, g: 1962540515, s: 19440033 },
        small_prime { p: 2147239937, g: 2100082663, s: 353296760 },
        small_prime { p: 2147235841, g: 1991153006, s: 1703918027 },
        small_prime { p: 2147217409, g: 516405114, s: 1258919613 },
        small_prime { p: 2147205121, g: 409347988, s: 1089726929 },
        small_prime { p: 2147196929, g: 927788991, s: 1946238668 },
        small_prime { p: 2147178497, g: 1136922411, s: 1347028164 },
        small_prime { p: 2147100673, g: 868626236, s: 701164723 },
        small_prime { p: 2147082241, g: 1897279176, s: 617820870 },
        small_prime { p: 2147074049, g: 1888819123, s: 158382189 },
        small_prime { p: 2147051521, g: 25006327, s: 522758543 },
        small_prime { p: 2147043329, g: 327546255, s: 37227845 },
        small_prime { p: 2147039233, g: 766324424, s: 1133356428 },
        small_prime { p: 2146988033, g: 1862817362, s: 73861329 },
        small_prime { p: 2146963457, g: 404622040, s: 653019435 },
        small_prime { p: 2146959361, g: 1936581214, s: 995143093 },
        small_prime { p: 2146938881, g: 1559770096, s: 634921513 },
        small_prime { p: 2146908161, g: 422623708, s: 1985060172 },
        small_prime { p: 2146885633, g: 1751189170, s: 298238186 },
        small_prime { p: 2146871297, g: 578919515, s: 291810829 },
        small_prime { p: 2146846721, g: 1114060353, s: 915902322 },
        small_prime { p: 2146834433, g: 2069565474, s: 47859524 },
        small_prime { p: 2146818049, g: 1552824584, s: 646281055 },
        small_prime { p: 2146775041, g: 1906267847, s: 1597832891 },
        small_prime { p: 2146756609, g: 1847414714, s: 1228090888 },
        small_prime { p: 2146744321, g: 1818792070, s: 1176377637 },
        small_prime { p: 2146738177, g: 1118066398, s: 1054971214 },
        small_prime { p: 2146736129, g: 52057278, s: 933422153 },
        small_prime { p: 2146713601, g: 592259376, s: 1406621510 },
        small_prime { p: 2146695169, g: 263161877, s: 1514178701 },
        small_prime { p: 2146656257, g: 685363115, s: 384505091 },
        small_prime { p: 2146650113, g: 927727032, s: 537575289 },
        small_prime { p: 2146646017, g: 52575506, s: 1799464037 },
        small_prime { p: 2146643969, g: 1276803876, s: 1348954416 },
        small_prime { p: 2146603009, g: 814028633, s: 1521547704 },
        small_prime { p: 2146572289, g: 1846678872, s: 1310832121 },
        small_prime { p: 2146547713, g: 919368090, s: 1019041349 },
        small_prime { p: 2146508801, g: 671847612, s: 38582496 },
        small_prime { p: 2146492417, g: 283911680, s: 532424562 },
        small_prime { p: 2146490369, g: 1780044827, s: 896447978 },
        small_prime { p: 2146459649, g: 327980850, s: 1327906900 },
        small_prime { p: 2146447361, g: 1310561493, s: 958645253 },
        small_prime { p: 2146441217, g: 412148926, s: 287271128 },
        small_prime { p: 2146437121, g: 293186449, s: 2009822534 },
        small_prime { p: 2146430977, g: 179034356, s: 1359155584 },
        small_prime { p: 2146418689, g: 1517345488, s: 1790248672 },
        small_prime { p: 2146406401, g: 1615820390, s: 1584833571 },
        small_prime { p: 2146404353, g: 826651445, s: 607120498 },
        small_prime { p: 2146379777, g: 3816988, s: 1897049071 },
        small_prime { p: 2146363393, g: 1221409784, s: 1986921567 },
        small_prime { p: 2146355201, g: 1388081168, s: 849968120 },
        small_prime { p: 2146336769, g: 1803473237, s: 1655544036 },
        small_prime { p: 2146312193, g: 1023484977, s: 273671831 },
        small_prime { p: 2146293761, g: 1074591448, s: 467406983 },
        small_prime { p: 2146283521, g: 831604668, s: 1523950494 },
        small_prime { p: 2146203649, g: 712865423, s: 1170834574 },
        small_prime { p: 2146154497, g: 1764991362, s: 1064856763 },
        small_prime { p: 2146142209, g: 627386213, s: 1406840151 },
        small_prime { p: 2146127873, g: 1638674429, s: 2088393537 },
        small_prime { p: 2146099201, g: 1516001018, s: 690673370 },
        small_prime { p: 2146093057, g: 1294931393, s: 315136610 },
        small_prime { p: 2146091009, g: 1942399533, s: 973539425 },
        small_prime { p: 2146078721, g: 1843461814, s: 2132275436 },
        small_prime { p: 2146060289, g: 1098740778, s: 360423481 },
        small_prime { p: 2146048001, g: 1617213232, s: 1951981294 },
        small_prime { p: 2146041857, g: 1805783169, s: 2075683489 },
        small_prime { p: 2146019329, g: 272027909, s: 1753219918 },
        small_prime { p: 2145986561, g: 1206530344, s: 2034028118 },
        small_prime { p: 2145976321, g: 1243769360, s: 1173377644 },
        small_prime { p: 2145964033, g: 887200839, s: 1281344586 },
        small_prime { p: 2145906689, g: 1651026455, s: 906178216 },
        small_prime { p: 2145875969, g: 1673238256, s: 1043521212 },
        small_prime { p: 2145871873, g: 1226591210, s: 1399796492 },
        small_prime { p: 2145841153, g: 1465353397, s: 1324527802 },
        small_prime { p: 2145832961, g: 1150638905, s: 554084759 },
        small_prime { p: 2145816577, g: 221601706, s: 427340863 },
        small_prime { p: 2145785857, g: 608896761, s: 316590738 },
        small_prime { p: 2145755137, g: 1712054942, s: 1684294304 },
        small_prime { p: 2145742849, g: 1302302867, s: 724873116 },
        small_prime { p: 2145728513, g: 516717693, s: 431671476 },
        small_prime { p: 2145699841, g: 524575579, s: 1619722537 },
        small_prime { p: 2145691649, g: 1925625239, s: 982974435 },
        small_prime { p: 2145687553, g: 463795662, s: 1293154300 },
        small_prime { p: 2145673217, g: 771716636, s: 881778029 },
        small_prime { p: 2145630209, g: 1509556977, s: 837364988 },
        small_prime { p: 2145595393, g: 229091856, s: 851648427 },
        small_prime { p: 2145587201, g: 1796903241, s: 635342424 },
        small_prime { p: 2145525761, g: 715310882, s: 1677228081 },
        small_prime { p: 2145495041, g: 1040930522, s: 200685896 },
        small_prime { p: 2145466369, g: 949804237, s: 1809146322 },
        small_prime { p: 2145445889, g: 1673903706, s: 95316881 },
        small_prime { p: 2145390593, g: 806941852, s: 1428671135 },
        small_prime { p: 2145372161, g: 1402525292, s: 159350694 },
        small_prime { p: 2145361921, g: 2124760298, s: 1589134749 },
        small_prime { p: 2145359873, g: 1217503067, s: 1561543010 },
        small_prime { p: 2145355777, g: 338341402, s: 83865711 },
        small_prime { p: 2145343489, g: 1381532164, s: 641430002 },
        small_prime { p: 2145325057, g: 1883895478, s: 1528469895 },
        small_prime { p: 2145318913, g: 1335370424, s: 65809740 },
        small_prime { p: 2145312769, g: 2000008042, s: 1919775760 },
        small_prime { p: 2145300481, g: 961450962, s: 1229540578 },
        small_prime { p: 2145282049, g: 910466767, s: 1964062701 },
        small_prime { p: 2145232897, g: 816527501, s: 450152063 },
        small_prime { p: 2145218561, g: 1435128058, s: 1794509700 },
        small_prime { p: 2145187841, g: 33505311, s: 1272467582 },
        small_prime { p: 2145181697, g: 269767433, s: 1380363849 },
        small_prime { p: 2145175553, g: 56386299, s: 1316870546 },
        small_prime { p: 2145079297, g: 2106880293, s: 1391797340 },
        small_prime { p: 2145021953, g: 1347906152, s: 720510798 },
        small_prime { p: 2145015809, g: 206769262, s: 1651459955 },
        small_prime { p: 2145003521, g: 1885513236, s: 1393381284 },
        small_prime { p: 2144960513, g: 1810381315, s: 31937275 },
        small_prime { p: 2144944129, g: 1306487838, s: 2019419520 },
        small_prime { p: 2144935937, g: 37304730, s: 1841489054 },
        small_prime { p: 2144894977, g: 1601434616, s: 157985831 },
        small_prime { p: 2144888833, g: 98749330, s: 2128592228 },
        small_prime { p: 2144880641, g: 1772327002, s: 2076128344 },
        small_prime { p: 2144864257, g: 1404514762, s: 2029969964 },
        small_prime { p: 2144827393, g: 801236594, s: 406627220 },
        small_prime { p: 2144806913, g: 349217443, s: 1501080290 },
        small_prime { p: 2144796673, g: 1542656776, s: 2084736519 },
        small_prime { p: 2144778241, g: 1210734884, s: 1746416203 },
        small_prime { p: 2144759809, g: 1146598851, s: 716464489 },
        small_prime { p: 2144757761, g: 286328400, s: 1823728177 },
        small_prime { p: 2144729089, g: 1347555695, s: 1836644881 },
        small_prime { p: 2144727041, g: 1795703790, s: 520296412 },
        small_prime { p: 2144696321, g: 1302475157, s: 852964281 },
        small_prime { p: 2144667649, g: 1075877614, s: 504992927 },
        small_prime { p: 2144573441, g: 198765808, s: 1617144982 },
        small_prime { p: 2144555009, g: 321528767, s: 155821259 },
        small_prime { p: 2144550913, g: 814139516, s: 1819937644 },
        small_prime { p: 2144536577, g: 571143206, s: 962942255 },
        small_prime { p: 2144524289, g: 1746733766, s: 2471321 },
        small_prime { p: 2144512001, g: 1821415077, s: 124190939 },
        small_prime { p: 2144468993, g: 917871546, s: 1260072806 },
        small_prime { p: 2144458753, g: 378417981, s: 1569240563 },
        small_prime { p: 2144421889, g: 175229668, s: 1825620763 },
        small_prime { p: 2144409601, g: 1699216963, s: 351648117 },
        small_prime { p: 2144370689, g: 1071885991, s: 958186029 },
        small_prime { p: 2144348161, g: 1763151227, s: 540353574 },
        small_prime { p: 2144335873, g: 1060214804, s: 919598847 },
        small_prime { p: 2144329729, g: 663515846, s: 1448552668 },
        small_prime { p: 2144327681, g: 1057776305, s: 590222840 },
        small_prime { p: 2144309249, g: 1705149168, s: 1459294624 },
        small_prime { p: 2144296961, g: 325823721, s: 1649016934 },
        small_prime { p: 2144290817, g: 738775789, s: 447427206 },
        small_prime { p: 2144243713, g: 962347618, s: 893050215 },
        small_prime { p: 2144237569, g: 1655257077, s: 900860862 },
        small_prime { p: 2144161793, g: 242206694, s: 1567868672 },
        small_prime { p: 2144155649, g: 769415308, s: 1247993134 },
        small_prime { p: 2144137217, g: 320492023, s: 515841070 },
        small_prime { p: 2144120833, g: 1639388522, s: 770877302 },
        small_prime { p: 2144071681, g: 1761785233, s: 964296120 },
        small_prime { p: 2144065537, g: 419817825, s: 204564472 },
        small_prime { p: 2144028673, g: 666050597, s: 2091019760 },
        small_prime { p: 2144010241, g: 1413657615, s: 1518702610 },
        small_prime { p: 2143952897, g: 1238327946, s: 475672271 },
        small_prime { p: 2143940609, g: 307063413, s: 1176750846 },
        small_prime { p: 2143918081, g: 2062905559, s: 786785803 },
        small_prime { p: 2143899649, g: 1338112849, s: 1562292083 },
        small_prime { p: 2143891457, g: 68149545, s: 87166451 },
        small_prime { p: 2143885313, g: 921750778, s: 394460854 },
        small_prime { p: 2143854593, g: 719766593, s: 133877196 },
        small_prime { p: 2143836161, g: 1149399850, s: 1861591875 },
        small_prime { p: 2143762433, g: 1848739366, s: 1335934145 },
        small_prime { p: 2143756289, g: 1326674710, s: 102999236 },
        small_prime { p: 2143713281, g: 808061791, s: 1156900308 },
        small_prime { p: 2143690753, g: 388399459, s: 1926468019 },
        small_prime { p: 2143670273, g: 1427891374, s: 1756689401 },
        small_prime { p: 2143666177, g: 1912173949, s: 986629565 },
        small_prime { p: 2143645697, g: 2041160111, s: 371842865 },
        small_prime { p: 2143641601, g: 1279906897, s: 2023974350 },
        small_prime { p: 2143635457, g: 720473174, s: 1389027526 },
        small_prime { p: 2143621121, g: 1298309455, s: 1732632006 },
        small_prime { p: 2143598593, g: 1548762216, s: 1825417506 },
        small_prime { p: 2143567873, g: 620475784, s: 1073787233 },
        small_prime { p: 2143561729, g: 1932954575, s: 949167309 },
        small_prime { p: 2143553537, g: 354315656, s: 1652037534 },
        small_prime { p: 2143541249, g: 577424288, s: 1097027618 },
        small_prime { p: 2143531009, g: 357862822, s: 478640055 },
        small_prime { p: 2143522817, g: 2017706025, s: 1550531668 },
        small_prime { p: 2143506433, g: 2078127419, s: 1824320165 },
        small_prime { p: 2143488001, g: 613475285, s: 1604011510 },
        small_prime { p: 2143469569, g: 1466594987, s: 502095196 },
        small_prime { p: 2143426561, g: 1115430331, s: 1044637111 },
        small_prime { p: 2143383553, g: 9778045, s: 1902463734 },
        small_prime { p: 2143377409, g: 1557401276, s: 2056861771 },
        small_prime { p: 2143363073, g: 652036455, s: 1965915971 },
        small_prime { p: 2143260673, g: 1464581171, s: 1523257541 },
        small_prime { p: 2143246337, g: 1876119649, s: 764541916 },
        small_prime { p: 2143209473, g: 1614992673, s: 1920672844 },
        small_prime { p: 2143203329, g: 981052047, s: 2049774209 },
        small_prime { p: 2143160321, g: 1847355533, s: 728535665 },
        small_prime { p: 2143129601, g: 965558457, s: 603052992 },
        small_prime { p: 2143123457, g: 2140817191, s: 8348679 },
        small_prime { p: 2143100929, g: 1547263683, s: 694209023 },
        small_prime { p: 2143092737, g: 643459066, s: 1979934533 },
        small_prime { p: 2143082497, g: 188603778, s: 2026175670 },
        small_prime { p: 2143062017, g: 1657329695, s: 377451099 },
        small_prime { p: 2143051777, g: 114967950, s: 979255473 },
        small_prime { p: 2143025153, g: 1698431342, s: 1449196896 },
        small_prime { p: 2143006721, g: 1862741675, s: 1739650365 },
        small_prime { p: 2142996481, g: 756660457, s: 996160050 },
        small_prime { p: 2142976001, g: 927864010, s: 1166847574 },
        small_prime { p: 2142965761, g: 905070557, s: 661974566 },
        small_prime { p: 2142916609, g: 40932754, s: 1787161127 },
        small_prime { p: 2142892033, g: 1987985648, s: 675335382 },
        small_prime { p: 2142885889, g: 797497211, s: 1323096997 },
        small_prime { p: 2142871553, g: 2068025830, s: 1411877159 },
        small_prime { p: 2142861313, g: 1217177090, s: 1438410687 },
        small_prime { p: 2142830593, g: 409906375, s: 1767860634 },
        small_prime { p: 2142803969, g: 1197788993, s: 359782919 },
        small_prime { p: 2142785537, g: 643817365, s: 513932862 },
        small_prime { p: 2142779393, g: 1717046338, s: 218943121 },
        small_prime { p: 2142724097, g: 89336830, s: 416687049 },
        small_prime { p: 2142707713, g: 5944581, s: 1356813523 },
        small_prime { p: 2142658561, g: 887942135, s: 2074011722 },
        small_prime { p: 2142638081, g: 151851972, s: 1647339939 },
        small_prime { p: 2142564353, g: 1691505537, s: 1483107336 },
        small_prime { p: 2142533633, g: 1989920200, s: 1135938817 },
        small_prime { p: 2142529537, g: 959263126, s: 1531961857 },
        small_prime { p: 2142527489, g: 453251129, s: 1725566162 },
        small_prime { p: 2142502913, g: 1536028102, s: 182053257 },
        small_prime { p: 2142498817, g: 570138730, s: 701443447 },
        small_prime { p: 2142416897, g: 326965800, s: 411931819 },
        small_prime { p: 2142363649, g: 1675665410, s: 1517191733 },
        small_prime { p: 2142351361, g: 968529566, s: 1575712703 },
        small_prime { p: 2142330881, g: 1384953238, s: 1769087884 },
        small_prime { p: 2142314497, g: 1977173242, s: 1833745524 },
        small_prime { p: 2142289921, g: 95082313, s: 1714775493 },
        small_prime { p: 2142283777, g: 109377615, s: 1070584533 },
        small_prime { p: 2142277633, g: 16960510, s: 702157145 },
        small_prime { p: 2142263297, g: 553850819, s: 431364395 },
        small_prime { p: 2142208001, g: 241466367, s: 2053967982 },
        small_prime { p: 2142164993, g: 1795661326, s: 1031836848 },
        small_prime { p: 2142097409, g: 1212530046, s: 712772031 },
        small_prime { p: 2142087169, g: 1763869720, s: 822276067 },
        small_prime { p: 2142078977, g: 644065713, s: 1765268066 },
        small_prime { p: 2142074881, g: 112671944, s: 643204925 },
        small_prime { p: 2142044161, g: 1387785471, s: 1297890174 },
        small_prime { p: 2142025729, g: 783885537, s: 1000425730 },
        small_prime { p: 2142011393, g: 905662232, s: 1679401033 },
        small_prime { p: 2141974529, g: 799788433, s: 468119557 },
        small_prime { p: 2141943809, g: 1932544124, s: 449305555 },
        small_prime { p: 2141933569, g: 1527403256, s: 841867925 },
        small_prime { p: 2141931521, g: 1247076451, s: 743823916 },
        small_prime { p: 2141902849, g: 1199660531, s: 401687910 },
        small_prime { p: 2141890561, g: 150132350, s: 1720336972 },
        small_prime { p: 2141857793, g: 1287438162, s: 663880489 },
        small_prime { p: 2141833217, g: 618017731, s: 1819208266 },
        small_prime { p: 2141820929, g: 999578638, s: 1403090096 },
        small_prime { p: 2141786113, g: 81834325, s: 1523542501 },
        small_prime { p: 2141771777, g: 120001928, s: 463556492 },
        small_prime { p: 2141759489, g: 122455485, s: 2124928282 },
        small_prime { p: 2141749249, g: 141986041, s: 940339153 },
        small_prime { p: 2141685761, g: 889088734, s: 477141499 },
        small_prime { p: 2141673473, g: 324212681, s: 1122558298 },
        small_prime { p: 2141669377, g: 1175806187, s: 1373818177 },
        small_prime { p: 2141655041, g: 1113654822, s: 296887082 },
        small_prime { p: 2141587457, g: 991103258, s: 1585913875 },
        small_prime { p: 2141583361, g: 1401451409, s: 1802457360 },
        small_prime { p: 2141575169, g: 1571977166, s: 712760980 },
        small_prime { p: 2141546497, g: 1107849376, s: 1250270109 },
        small_prime { p: 2141515777, g: 196544219, s: 356001130 },
        small_prime { p: 2141495297, g: 1733571506, s: 1060744866 },
        small_prime { p: 2141483009, g: 321552363, s: 1168297026 },
        small_prime { p: 2141458433, g: 505818251, s: 733225819 },
        small_prime { p: 2141360129, g: 1026840098, s: 948342276 },
        small_prime { p: 2141325313, g: 945133744, s: 2129965998 },
        small_prime { p: 2141317121, g: 1871100260, s: 1843844634 },
        small_prime { p: 2141286401, g: 1790639498, s: 1750465696 },
        small_prime { p: 2141267969, g: 1376858592, s: 186160720 },
        small_prime { p: 2141255681, g: 2129698296, s: 1876677959 },
        small_prime { p: 2141243393, g: 2138900688, s: 1340009628 },
        small_prime { p: 2141214721, g: 1933049835, s: 1087819477 },
        small_prime { p: 2141212673, g: 1898664939, s: 1786328049 },
        small_prime { p: 2141202433, g: 990234828, s: 940682169 },
        small_prime { p: 2141175809, g: 1406392421, s: 993089586 },
        small_prime { p: 2141165569, g: 1263518371, s: 289019479 },
        small_prime { p: 2141073409, g: 1485624211, s: 507864514 },
        small_prime { p: 2141052929, g: 1885134788, s: 311252465 },
        small_prime { p: 2141040641, g: 1285021247, s: 280941862 },
        small_prime { p: 2141028353, g: 1527610374, s: 375035110 },
        small_prime { p: 2141011969, g: 1400626168, s: 164696620 },
        small_prime { p: 2140999681, g: 632959608, s: 966175067 },
        small_prime { p: 2140997633, g: 2045628978, s: 1290889438 },
        small_prime { p: 2140993537, g: 1412755491, s: 375366253 },
        small_prime { p: 2140942337, g: 719477232, s: 785367828 },
        small_prime { p: 2140925953, g: 45224252, s: 836552317 },
        small_prime { p: 2140917761, g: 1157376588, s: 1001839569 },
        small_prime { p: 2140887041, g: 278480752, s: 2098732796 },
        small_prime { p: 2140837889, g: 1663139953, s: 924094810 },
        small_prime { p: 2140788737, g: 802501511, s: 2045368990 },
        small_prime { p: 2140766209, g: 1820083885, s: 1800295504 },
        small_prime { p: 2140764161, g: 1169561905, s: 2106792035 },
        small_prime { p: 2140696577, g: 127781498, s: 1885987531 },
        small_prime { p: 2140684289, g: 16014477, s: 1098116827 },
        small_prime { p: 2140653569, g: 665960598, s: 1796728247 },
        small_prime { p: 2140594177, g: 1043085491, s: 377310938 },
        small_prime { p: 2140579841, g: 1732838211, s: 1504505945 },
        small_prime { p: 2140569601, g: 302071939, s: 358291016 },
        small_prime { p: 2140567553, g: 192393733, s: 1909137143 },
        small_prime { p: 2140557313, g: 406595731, s: 1175330270 },
        small_prime { p: 2140549121, g: 1748850918, s: 525007007 },
        small_prime { p: 2140477441, g: 499436566, s: 1031159814 },
        small_prime { p: 2140469249, g: 1886004401, s: 1029951320 },
        small_prime { p: 2140426241, g: 1483168100, s: 1676273461 },
        small_prime { p: 2140420097, g: 1779917297, s: 846024476 },
        small_prime { p: 2140413953, g: 522948893, s: 1816354149 },
        small_prime { p: 2140383233, g: 1931364473, s: 1296921241 },
        small_prime { p: 2140366849, g: 1917356555, s: 147196204 },
        small_prime { p: 2140354561, g: 16466177, s: 1349052107 },
        small_prime { p: 2140348417, g: 1875366972, s: 1860485634 },
        small_prime { p: 2140323841, g: 456498717, s: 1790256483 },
        small_prime { p: 2140321793, g: 1629493973, s: 150031888 },
        small_prime { p: 2140315649, g: 1904063898, s: 395510935 },
        small_prime { p: 2140280833, g: 1784104328, s: 831417909 },
        small_prime { p: 2140250113, g: 256087139, s: 697349101 },
        small_prime { p: 2140229633, g: 388553070, s: 243875754 },
        small_prime { p: 2140223489, g: 747459608, s: 1396270850 },
        small_prime { p: 2140200961, g: 507423743, s: 1895572209 },
        small_prime { p: 2140162049, g: 580106016, s: 2045297469 },
        small_prime { p: 2140149761, g: 712426444, s: 785217995 },
        small_prime { p: 2140137473, g: 1441607584, s: 536866543 },
        small_prime { p: 2140119041, g: 346538902, s: 1740434653 },
        small_prime { p: 2140090369, g: 282642885, s: 21051094 },
        small_prime { p: 2140076033, g: 1407456228, s: 319910029 },
        small_prime { p: 2140047361, g: 1619330500, s: 1488632070 },
        small_prime { p: 2140041217, g: 2089408064, s: 2012026134 },
        small_prime { p: 2140008449, g: 1705524800, s: 1613440760 },
        small_prime { p: 2139924481, g: 1846208233, s: 1280649481 },
        small_prime { p: 2139906049, g: 989438755, s: 1185646076 },
        small_prime { p: 2139867137, g: 1522314850, s: 372783595 },
        small_prime { p: 2139842561, g: 1681587377, s: 216848235 },
        small_prime { p: 2139826177, g: 2066284988, s: 1784999464 },
        small_prime { p: 2139824129, g: 480888214, s: 1513323027 },
        small_prime { p: 2139789313, g: 847937200, s: 858192859 },
        small_prime { p: 2139783169, g: 1642000434, s: 1583261448 },
        small_prime { p: 2139770881, g: 940699589, s: 179702100 },
        small_prime { p: 2139768833, g: 315623242, s: 964612676 },
        small_prime { p: 2139666433, g: 331649203, s: 764666914 },
        small_prime { p: 2139641857, g: 2118730799, s: 1313764644 },
        small_prime { p: 2139635713, g: 519149027, s: 519212449 },
        small_prime { p: 2139598849, g: 1526413634, s: 1769667104 },
        small_prime { p: 2139574273, g: 551148610, s: 820739925 },
        small_prime { p: 2139568129, g: 1386800242, s: 472447405 },
        small_prime { p: 2139549697, g: 813760130, s: 1412328531 },
        small_prime { p: 2139537409, g: 1615286260, s: 1609362979 },
        small_prime { p: 2139475969, g: 1352559299, s: 1696720421 },
        small_prime { p: 2139455489, g: 1048691649, s: 1584935400 },
        small_prime { p: 2139432961, g: 836025845, s: 950121150 },
        small_prime { p: 2139424769, g: 1558281165, s: 1635486858 },
        small_prime { p: 2139406337, g: 1728402143, s: 1674423301 },
        small_prime { p: 2139396097, g: 1727715782, s: 1483470544 },
        small_prime { p: 2139383809, g: 1092853491, s: 1741699084 },
        small_prime { p: 2139369473, g: 690776899, s: 1242798709 },
        small_prime { p: 2139351041, g: 1768782380, s: 2120712049 },
        small_prime { p: 2139334657, g: 1739968247, s: 1427249225 },
        small_prime { p: 2139332609, g: 1547189119, s: 623011170 },
        small_prime { p: 2139310081, g: 1346827917, s: 1605466350 },
        small_prime { p: 2139303937, g: 369317948, s: 828392831 },
        small_prime { p: 2139301889, g: 1560417239, s: 1788073219 },
        small_prime { p: 2139283457, g: 1303121623, s: 595079358 },
        small_prime { p: 2139248641, g: 1354555286, s: 573424177 },
        small_prime { p: 2139240449, g: 60974056, s: 885781403 },
        small_prime { p: 2139222017, g: 355573421, s: 1221054839 },
        small_prime { p: 2139215873, g: 566477826, s: 1724006500 },
        small_prime { p: 2139150337, g: 871437673, s: 1609133294 },
        small_prime { p: 2139144193, g: 1478130914, s: 1137491905 },
        small_prime { p: 2139117569, g: 1854880922, s: 964728507 },
        small_prime { p: 2139076609, g: 202405335, s: 756508944 },
        small_prime { p: 2139062273, g: 1399715741, s: 884826059 },
        small_prime { p: 2139045889, g: 1051045798, s: 1202295476 },
        small_prime { p: 2139033601, g: 1707715206, s: 632234634 },
        small_prime { p: 2139006977, g: 2035853139, s: 231626690 },
        small_prime { p: 2138951681, g: 183867876, s: 838350879 },
        small_prime { p: 2138945537, g: 1403254661, s: 404460202 },
        small_prime { p: 2138920961, g: 310865011, s: 1282911681 },
        small_prime { p: 2138910721, g: 1328496553, s: 103472415 },
        small_prime { p: 2138904577, g: 78831681, s: 993513549 },
        small_prime { p: 2138902529, g: 1319697451, s: 1055904361 },
        small_prime { p: 2138816513, g: 384338872, s: 1706202469 },
        small_prime { p: 2138810369, g: 1084868275, s: 405677177 },
        small_prime { p: 2138787841, g: 401181788, s: 1964773901 },
        small_prime { p: 2138775553, g: 1850532988, s: 1247087473 },
        small_prime { p: 2138767361, g: 874261901, s: 1576073565 },
        small_prime { p: 2138757121, g: 1187474742, s: 993541415 },
        small_prime { p: 2138748929, g: 1782458888, s: 1043206483 },
        small_prime { p: 2138744833, g: 1221500487, s: 800141243 },
        small_prime { p: 2138738689, g: 413465368, s: 1450660558 },
        small_prime { p: 2138695681, g: 739045140, s: 342611472 },
        small_prime { p: 2138658817, g: 1355845756, s: 672674190 },
        small_prime { p: 2138644481, g: 608379162, s: 1538874380 },
        small_prime { p: 2138632193, g: 1444914034, s: 686911254 },
        small_prime { p: 2138607617, g: 484707818, s: 1435142134 },
        small_prime { p: 2138591233, g: 539460669, s: 1290458549 },
        small_prime { p: 2138572801, g: 2093538990, s: 2011138646 },
        small_prime { p: 2138552321, g: 1149786988, s: 1076414907 },
        small_prime { p: 2138546177, g: 840688206, s: 2108985273 },
        small_prime { p: 2138533889, g: 209669619, s: 198172413 },
        small_prime { p: 2138523649, g: 1975879426, s: 1277003968 },
        small_prime { p: 2138490881, g: 1351891144, s: 1976858109 },
        small_prime { p: 2138460161, g: 1817321013, s: 1979278293 },
        small_prime { p: 2138429441, g: 1950077177, s: 203441928 },
        small_prime { p: 2138400769, g: 908970113, s: 628395069 },
        small_prime { p: 2138398721, g: 219890864, s: 758486760 },
        small_prime { p: 2138376193, g: 1306654379, s: 977554090 },
        small_prime { p: 2138351617, g: 298822498, s: 2004708503 },
        small_prime { p: 2138337281, g: 441457816, s: 1049002108 },
        small_prime { p: 2138320897, g: 1517731724, s: 1442269609 },
        small_prime { p: 2138290177, g: 1355911197, s: 1647139103 },
        small_prime { p: 2138234881, g: 531313247, s: 1746591962 },
        small_prime { p: 2138214401, g: 1899410930, s: 781416444 },
        small_prime { p: 2138202113, g: 1813477173, s: 1622508515 },
        small_prime { p: 2138191873, g: 1086458299, s: 1025408615 },
        small_prime { p: 2138183681, g: 1998800427, s: 827063290 },
        small_prime { p: 2138173441, g: 1921308898, s: 749670117 },
        small_prime { p: 2138103809, g: 1620902804, s: 2126787647 },
        small_prime { p: 2138099713, g: 828647069, s: 1892961817 },
        small_prime { p: 2138085377, g: 179405355, s: 1525506535 },
        small_prime { p: 2138060801, g: 615683235, s: 1259580138 },
        small_prime { p: 2138044417, g: 2030277840, s: 1731266562 },
        small_prime { p: 2138042369, g: 2087222316, s: 1627902259 },
        small_prime { p: 2138032129, g: 126388712, s: 1108640984 },
        small_prime { p: 2138011649, g: 715026550, s: 1017980050 },
        small_prime { p: 2137993217, g: 1693714349, s: 1351778704 },
        small_prime { p: 2137888769, g: 1289762259, s: 1053090405 },
        small_prime { p: 2137853953, g: 199991890, s: 1254192789 },
        small_prime { p: 2137833473, g: 941421685, s: 896995556 },
        small_prime { p: 2137817089, g: 750416446, s: 1251031181 },
        small_prime { p: 2137792513, g: 798075119, s: 368077456 },
        small_prime { p: 2137786369, g: 878543495, s: 1035375025 },
        small_prime { p: 2137767937, g: 9351178, s: 1156563902 },
        small_prime { p: 2137755649, g: 1382297614, s: 1686559583 },
        small_prime { p: 2137724929, g: 1345472850, s: 1681096331 },
        small_prime { p: 2137704449, g: 834666929, s: 630551727 },
        small_prime { p: 2137673729, g: 1646165729, s: 1892091571 },
        small_prime { p: 2137620481, g: 778943821, s: 48456461 },
        small_prime { p: 2137618433, g: 1730837875, s: 1713336725 },
        small_prime { p: 2137581569, g: 805610339, s: 1378891359 },
        small_prime { p: 2137538561, g: 204342388, s: 1950165220 },
        small_prime { p: 2137526273, g: 1947629754, s: 1500789441 },
        small_prime { p: 2137516033, g: 719902645, s: 1499525372 },
        small_prime { p: 2137491457, g: 230451261, s: 556382829 },
        small_prime { p: 2137440257, g: 979573541, s: 412760291 },
        small_prime { p: 2137374721, g: 927841248, s: 1954137185 },
        small_prime { p: 2137362433, g: 1243778559, s: 861024672 },
        small_prime { p: 2137313281, g: 1341338501, s: 980638386 },
        small_prime { p: 2137311233, g: 937415182, s: 1793212117 },
        small_prime { p: 2137255937, g: 795331324, s: 1410253405 },
        small_prime { p: 2137243649, g: 150756339, s: 1966999887 },
        small_prime { p: 2137182209, g: 163346914, s: 1939301431 },
        small_prime { p: 2137171969, g: 1952552395, s: 758913141 },
        small_prime { p: 2137159681, g: 570788721, s: 218668666 },
        small_prime { p: 2137147393, g: 1896656810, s: 2045670345 },
        small_prime { p: 2137141249, g: 358493842, s: 518199643 },
        small_prime { p: 2137139201, g: 1505023029, s: 674695848 },
        small_prime { p: 2137133057, g: 27911103, s: 830956306 },
        small_prime { p: 2137122817, g: 439771337, s: 1555268614 },
        small_prime { p: 2137116673, g: 790988579, s: 1871449599 },
        small_prime { p: 2137110529, g: 432109234, s: 811805080 },
        small_prime { p: 2137102337, g: 1357900653, s: 1184997641 },
        small_prime { p: 2137098241, g: 515119035, s: 1715693095 },
        small_prime { p: 2137090049, g: 408575203, s: 2085660657 },
        small_prime { p: 2137085953, g: 2097793407, s: 1349626963 },
        small_prime { p: 2137055233, g: 1556739954, s: 1449960883 },
        small_prime { p: 2137030657, g: 1545758650, s: 1369303716 },
        small_prime { p: 2136987649, g: 332602570, s: 103875114 },
        small_prime { p: 2136969217, g: 1499989506, s: 1662964115 },
        small_prime { p: 2136924161, g: 857040753, s: 4738842 },
        small_prime { p: 2136895489, g: 1948872712, s: 570436091 },
        small_prime { p: 2136893441, g: 58969960, s: 1568349634 },
        small_prime { p: 2136887297, g: 2127193379, s: 273612548 },
        small_prime { p: 2136850433, g: 111208983, s: 1181257116 },
        small_prime { p: 2136809473, g: 1627275942, s: 1680317971 },
        small_prime { p: 2136764417, g: 1574888217, s: 14011331 },
        small_prime { p: 2136741889, g: 14011055, s: 1129154251 },
        small_prime { p: 2136727553, g: 35862563, s: 1838555253 },
        small_prime { p: 2136721409, g: 310235666, s: 1363928244 },
        small_prime { p: 2136698881, g: 1612429202, s: 1560383828 },
        small_prime { p: 2136649729, g: 1138540131, s: 800014364 },
        small_prime { p: 2136606721, g: 602323503, s: 1433096652 },
        small_prime { p: 2136563713, g: 182209265, s: 1919611038 },
        small_prime { p: 2136555521, g: 324156477, s: 165591039 },
        small_prime { p: 2136549377, g: 195513113, s: 217165345 },
        small_prime { p: 2136526849, g: 1050768046, s: 939647887 },
        small_prime { p: 2136508417, g: 1886286237, s: 1619926572 },
        small_prime { p: 2136477697, g: 609647664, s: 35065157 },
        small_prime { p: 2136471553, g: 679352216, s: 1452259468 },
        small_prime { p: 2136457217, g: 128630031, s: 824816521 },
        small_prime { p: 2136422401, g: 19787464, s: 1526049830 },
        small_prime { p: 2136420353, g: 698316836, s: 1530623527 },
        small_prime { p: 2136371201, g: 1651862373, s: 1804812805 },
        small_prime { p: 2136334337, g: 326596005, s: 336977082 },
        small_prime { p: 2136322049, g: 63253370, s: 1904972151 },
        small_prime { p: 2136297473, g: 312176076, s: 172182411 },
        small_prime { p: 2136248321, g: 381261841, s: 369032670 },
        small_prime { p: 2136242177, g: 358688773, s: 1640007994 },
        small_prime { p: 2136229889, g: 512677188, s: 75585225 },
        small_prime { p: 2136219649, g: 2095003250, s: 1970086149 },
        small_prime { p: 2136207361, g: 1909650722, s: 537760675 },
        small_prime { p: 2136176641, g: 1334616195, s: 1533487619 },
        small_prime { p: 2136158209, g: 2096285632, s: 1793285210 },
        small_prime { p: 2136143873, g: 1897347517, s: 293843959 },
        small_prime { p: 2136133633, g: 923586222, s: 1022655978 },
        small_prime { p: 2136096769, g: 1464868191, s: 1515074410 },
        small_prime { p: 2136094721, g: 2020679520, s: 2061636104 },
        small_prime { p: 2136076289, g: 290798503, s: 1814726809 },
        small_prime { p: 2136041473, g: 156415894, s: 1250757633 },
        small_prime { p: 2135996417, g: 297459940, s: 1132158924 },
        small_prime { p: 2135955457, g: 538755304, s: 1688831340 },
        small_prime { p: 0, g: 0, s: 0 }
    ];
}

