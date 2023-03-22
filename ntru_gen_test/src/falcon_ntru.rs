#![allow(non_snake_case)]

#[cfg(test)]
mod tests {
    use rand::Rng;

    use ntru_gen::falcon_ntru::falcon_keygen;
    use ntru_gen_c::falcon_ntru::ntrugen_Falcon_keygen;

    #[test]
    fn falcon_ntru_test() {
        for logn in 2..11 {
            let mut tmp: [u32; 24 * 1024 + 18] = [0; 24 * 1024 + 18];
            let tmpc: [u32; 24 * 1024 + 18] = [0; 24 * 1024 + 18];
            let mut f: [i8; 1024] = [0; 1024];
            let fc: [i8; 1024] = [0; 1024];
            let mut g: [i8; 1024] = [0; 1024];
            let gc: [i8; 1024] = [0; 1024];
            let mut G: [i8; 1024] = [0; 1024];
            let Gc: [i8; 1024] = [0; 1024];
            let mut F: [i8; 1024] = [0; 1024];
            let Fc: [i8; 1024] = [0; 1024];
            let res = falcon_keygen(logn, &mut f, &mut g, &mut F, &mut G, &mut tmp);
            let resc = unsafe { ntrugen_Falcon_keygen(logn as u32, f.as_ptr(), g.as_ptr(), F.as_ptr(), G.as_ptr(), tmp.as_ptr()) };
            assert_eq!(f, fc);
        }
    }
}