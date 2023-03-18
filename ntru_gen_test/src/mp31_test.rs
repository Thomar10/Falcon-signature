#[cfg(test)]
mod tests {
    use ntru_gen::mp31::{mp_div, mp_intt, mp_mkgm, mp_mkgm7, mp_mkgmigm, mp_mkigm, mp_ntt};
    use ntru_gen_c::mp31::{ntrugen_mp_div, ntrugen_mp_iNTT, ntrugen_mp_mkgm, ntrugen_mp_mkgm7, ntrugen_mp_mkgmigm, ntrugen_mp_mkigm, ntrugen_mp_NTT};

    const P: u32 = 12289;

    #[test]
    fn div() {
        for _ in 0..1000 {
            let x: u32 = rand::random();
            let y: u32 = rand::random();
            assert_eq!(mp_div(x, y, P), unsafe { ntrugen_mp_div(x, y, P) });
        }
    }

    #[test]
    fn mkgmigm() {
        for _ in 0..1000 {
            let logn = 10;
            let g: u32 = rand::random();
            let ig: u32 = rand::random();
            let p0i: u32 = rand::random();
            let mut gm: [u32; 1024] = [0; 1024];
            let gmc: [u32; 1024] = [0; 1024];
            let mut igm: [u32; 1024] = [0; 1024];
            let igmc: [u32; 1024] = [0; 1024];
            mp_mkgmigm(logn, &mut gm, &mut igm, g, ig, P, p0i);
            unsafe { ntrugen_mp_mkgmigm(logn, gmc.as_ptr(), igmc.as_ptr(), g, ig, P, p0i) };
            assert_eq!(gm, gmc);
            assert_eq!(igm, igmc);
        }
    }

    #[test]
    fn mkgm() {
        for _ in 0..1000 {
            let logn = 10;
            let g: u32 = rand::random();
            let p0i: u32 = rand::random();
            let mut gm: [u32; 1024] = [0; 1024];
            let gmc: [u32; 1024] = [0; 1024];
            mp_mkgm(logn, &mut gm, g, P, p0i);
            unsafe { ntrugen_mp_mkgm(logn, gmc.as_ptr(), g, P, p0i) };
            assert_eq!(gm, gmc);
        }
    }

    #[test]
    fn mkgm7() {
        for _ in 0..1000 {
            let g: u32 = rand::random();
            let p0i: u32 = rand::random();
            let mut gm: [u32; 1024] = [0; 1024];
            let gm7: [u32; 1024] = [0; 1024];
            mp_mkgm7(&mut gm, g, P, p0i);
            unsafe { ntrugen_mp_mkgm7(gm7.as_ptr(), g, P, p0i) };
            assert_eq!(gm, gm7);
        }
    }

    #[test]
    fn mkigm() {
        for _ in 0..1000 {
            let logn = 10;
            let ig: u32 = rand::random();
            let p0i: u32 = rand::random();
            let mut igm: [u32; 1024] = [0; 1024];
            let igmc: [u32; 1024] = [0; 1024];
            mp_mkigm(logn, &mut igm, ig, P, p0i);
            unsafe { ntrugen_mp_mkigm(logn, igmc.as_ptr(), ig, P, p0i) };
            assert_eq!(igm, igmc);
        }
    }

    #[test]
    fn ntt_test() {
        for _ in 0..1000 {
            let logn = 10;
            let g: u32 = rand::random();
            let ig: u32 = rand::random();
            let p0i: u32 = rand::random();
            let mut gm: [u32; 1024] = [0; 1024];
            let mut igm: [u32; 1024] = [0; 1024];
            let mut a: [u32; 1024] = [0; 1024];
            let ac: [u32; 1024] = [0; 1024];
            mp_mkgmigm(logn, &mut gm, &mut igm, g, ig, P, p0i);
            mp_ntt(logn, &mut a, &gm, P, p0i);
            unsafe { ntrugen_mp_NTT(logn, ac.as_ptr(), gm.as_ptr(), P, p0i) };
            assert_eq!(a, ac);
            mp_intt(logn, &mut a, &igm, P, p0i);
            unsafe { ntrugen_mp_iNTT(logn, ac.as_ptr(), igm.as_ptr(), P, p0i) };
            assert_eq!(a, ac);

        }
    }
}