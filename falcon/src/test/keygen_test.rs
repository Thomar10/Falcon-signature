#[cfg(test)]
mod tests {
    use rand::Rng;
    use crate::falcon_c::keygen_c::{modp_add_func, modp_div_func, modp_mkgm2_func, modp_montymul_func, modp_ninv31_func, modp_norm_func, modp_NTT2_ext_func, modp_R2_func, modp_R_func, modp_Rx_func, modp_set_func, modp_sub_func};
    use crate::keygen::{modp_add, modp_div, modp_mkgm2, modp_montymul, modp_ninv31, modp_norm, modp_NTT2_ext, modp_R, modp_R2, modp_Rx, modp_set, modp_sub};


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
            let x: u16 = rand::random();
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
        for _ in 0..100 {
            for logn in 1..10 {
                for stride in 1usize..5 {
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
                    assert_eq!(gm, gm_c);
                }
            }
        }
    }
}