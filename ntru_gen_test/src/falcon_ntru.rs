#![allow(non_snake_case)]

#[cfg(test)]
mod tests {
    use rand::Rng;

    use ntru_gen::falcon_ntru::falcon_keygen;
    use ntru_gen::prng::{NtruPrngChacha8Context, prng_chacha8_out};
    use ntru_gen_c::falcon_ntru::ntrugen_Falcon_keygen;
    use ntru_gen_c::prng::{ntrugen_prng_chacha8_out, NtruPrngChacha8ContextC};

    #[test]
    fn falcon_ntru_test() {
        for logn in 10..11 {
            let (mut context, mut contextc) = get_contexts();
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
            let resc = unsafe { ntrugen_Falcon_keygen(logn as u32, fc.as_ptr(), gc.as_ptr(), Fc.as_ptr(), Gc.as_ptr(), ntrugen_prng_chacha8_out, &mut contextc, tmpc.as_ptr(), 24 * 1024 + 18) };
            let res = falcon_keygen(logn, &mut f, &mut g, &mut F, &mut G, prng_chacha8_out, &mut context, &mut tmp);
            assert_eq!(res, resc == 0);
            assert_eq!(f, fc);
            assert_eq!(g, gc);
            //assert_eq!(F, Fc);
            //assert_eq!(G, Gc);
        }
    }

    pub fn get_contexts() -> (NtruPrngChacha8Context, NtruPrngChacha8ContextC) {
        (NtruPrngChacha8Context {
            d: [0; 40],
        }, NtruPrngChacha8ContextC {
            d: [0; 40],
        })
    }
}