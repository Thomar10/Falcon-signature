#![allow(non_snake_case)]

#[cfg(test)]
mod tests {
    use rand::Rng;

    use ntru_gen::falcon_ntru::GAUSS_FALCON_1024;
    use ntru_gen::gauss::gauss_sample_poly;
    use ntru_gen::prng::{NtruPrngChacha8Context, prng_chacha8_init, prng_chacha8_out};
    use ntru_gen_c::gauss::ntrugen_gauss_sample_poly;
    use ntru_gen_c::prng::{ntrugen_prng_chacha8_init, ntrugen_prng_chacha8_out, NtruPrngChacha8ContextC};

    #[test]
    pub fn gauss_sample_poly_test() {
        let mut rng = rand::thread_rng();
        let (mut context, mut contextc) = get_contexts();
        let seed: [u8; 32] = core::array::from_fn(|_| rng.gen::<u8>());
        prng_chacha8_init(&mut context, &seed, 32);
        unsafe { ntrugen_prng_chacha8_init(&mut contextc, seed.as_ptr(), 32); }
        let mut f: [i8; 1024] = [0; 1024];
        let fc: [i8; 1024] = [0; 1024];
        unsafe { ntrugen_gauss_sample_poly(10, fc.as_ptr(), GAUSS_FALCON_1024.as_ptr(), ntrugen_prng_chacha8_out, &contextc) }
        gauss_sample_poly(10, &mut f, &GAUSS_FALCON_1024, prng_chacha8_out, &mut context);
        assert_eq!(f, fc, "f");
        let mut g: [i8; 1024] = [0; 1024];
        let gc: [i8; 1024] = [0; 1024];
        unsafe { ntrugen_gauss_sample_poly(10, gc.as_ptr(), GAUSS_FALCON_1024.as_ptr(), ntrugen_prng_chacha8_out, &contextc) }
        gauss_sample_poly(10, &mut g, &GAUSS_FALCON_1024, prng_chacha8_out, &mut context);
        assert_eq!(g, gc, "g");
    }


    pub fn get_contexts() -> (NtruPrngChacha8Context, NtruPrngChacha8ContextC) {
        (NtruPrngChacha8Context {
            d: [0; 40],
        }, NtruPrngChacha8ContextC {
            d: [0; 40],
        })
    }
}