#![allow(non_snake_case)]

#[cfg(test)]
mod tests {
    use rand::Rng;

    use ntru_gen::prng::{NtruPrngChacha8Context, prng_chacha8_init, prng_chacha8_out};
    use ntru_gen_c::prng::{ntrugen_prng_chacha8_init, ntrugen_prng_chacha8_out, NtruPrngChacha8ContextC};

    #[test]
    pub fn chacha8_init() {
        let mut rng = rand::thread_rng();
        let seed: [u8; 32] = core::array::from_fn(|_| rng.gen::<u8>());
        let (mut context, mut contextc) = get_contexts();
        prng_chacha8_init(&mut context, &seed, 32);
        unsafe { ntrugen_prng_chacha8_init(&mut contextc, seed.as_ptr(), 32); }
        assert_eq!(contextc.d, context.d);
    }

    #[test]
    pub fn chacha8_out() {
        let mut rng = rand::thread_rng();
        let seed: [u8; 32] = core::array::from_fn(|_| rng.gen::<u8>());
        let (mut context, mut contextc) = get_contexts();
        prng_chacha8_init(&mut context, &seed, 32);
        unsafe { ntrugen_prng_chacha8_init(&mut contextc, seed.as_ptr(), 32); }
        let mut out: [u8; 512] = [0; 512];
        let outc: [u8; 512] = [0; 512];
        prng_chacha8_out(&mut context, &mut out, 512);
        unsafe { ntrugen_prng_chacha8_out(&mut contextc, outc.as_ptr(), 512); }

        assert_eq!(contextc.d, context.d);
        assert_eq!(out, outc);
    }


    fn get_contexts() -> (NtruPrngChacha8Context, NtruPrngChacha8ContextC) {
        (NtruPrngChacha8Context {
            d: [0; 40],
        }, NtruPrngChacha8ContextC {
            d: [0; 40],
        })
    }
}