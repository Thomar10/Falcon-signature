#[cfg(test)]
pub(crate) mod tests {
    use std::ffi::c_void;

    use rand::Rng;

    use crate::falcon_c::rng_c::{Buf as BufC,
                                 Prng as PrngC,
                                 prng_get_bytes as prng_get_bytes_c,
                                 prng_get_u64_func as prng_get_u64_c,
                                 prng_get_u8_func as prng_get_u8_c,
                                 prng_init as prng_init_c,
                                 prng_refill as prng_refill_c,
                                 State as StateC};
    use crate::falcon_c::shake_c::{falcon_inner_i_shake256_init, InnerShake256Context as InnerShake256ContextC, St as StC};
    use crate::rng::{Prng, prng_get_bytes, prng_get_u64, prng_get_u8, prng_init, prng_refill, State};
    use crate::shake::{i_shake256_init, InnerShake256Context, St};

    #[test]
    fn test_prng_refill() {
        for _ in 0..100 {
            let (mut prng, prng_c): (Prng, PrngC) = create_random_prngs();

            prng_refill(&mut prng);

            unsafe {
                assert!(!test_prng_equality(&prng, &prng_c));

                prng_refill_c(&prng_c as *const PrngC);
                assert!(test_prng_equality(&prng, &prng_c))
            }
        }
    }

    #[test]
    fn test_prng_init() {
        for _ in 0..100 {
            let (mut prng, prng_c): (Prng, PrngC) = create_random_prngs();

            let random_state: [u64; 25] = rand::random();
            let random_dptr: u64 = rand::random();

            let st = St { a: random_state };

            let mut sc_rust = InnerShake256Context { st, dptr: random_dptr };

            let sc_c = InnerShake256ContextC { st: StC { a: random_state.clone() }, dptr: random_dptr };

            i_shake256_init(&mut sc_rust);
            prng_init(&mut prng, &mut sc_rust);

            unsafe {
                falcon_inner_i_shake256_init(&sc_c);
                assert!(!test_prng_equality(&prng, &prng_c));

                prng_init_c(&prng_c, &sc_c);
                assert!(test_prng_equality(&prng, &prng_c));
            }
        }
    }

    #[test]
    fn test_prng_get_bytes() {
        for _ in 0..100 {
            let (mut prng, prng_c): (Prng, PrngC) = create_random_prngs();
            init_prngs(&mut prng, &prng_c);

            for _ in 0..20 {
                let output_rust = prng_get_bytes(&mut prng, 100);
                let output_c: [u8; 100] = [0; 100];

                unsafe {
                    prng_get_bytes_c(&prng_c, output_c.as_ptr() as *mut c_void, 100);
                }

                assert_eq!(output_rust.as_slice(), output_c);
            }
        }
    }

    #[test]
    fn test_prng_get_u64() {
        for _ in 0..100 {
            let (mut prng, prng_c): (Prng, PrngC) = create_random_prngs();
            init_prngs(&mut prng, &prng_c);

            for _ in 0..20 {
                let output_rust:u64 = prng_get_u64(&mut prng);
                let output_c: u64;

                unsafe {
                    output_c = prng_get_u64_c(&prng_c);
                }

                assert_eq!(output_rust, output_c);
            }
        }
    }

    #[test]
    fn test_prng_get_u8() {
        for _ in 0..100 {
            let (mut prng, prng_c): (Prng, PrngC) = create_random_prngs();
            init_prngs(&mut prng, &prng_c);

            for _ in 0..20 {
                let output_rust:u8 = prng_get_u8(&mut prng);
                let output_c: u8;

                unsafe {
                    output_c = prng_get_u8_c(&prng_c);
                }

                assert_eq!(output_rust, output_c);
            }
        }
    }

    pub fn init_prngs(prng: &mut Prng, prng_c: &PrngC) {
        let random_state: [u64; 25] = rand::random();
        let random_dptr: u64 = rand::random();

        let st = St { a: random_state };

        let mut sc_rust = InnerShake256Context { st, dptr: random_dptr };

        let sc_c = InnerShake256ContextC { st: StC { a: random_state.clone() }, dptr: random_dptr };

        i_shake256_init(&mut sc_rust);
        prng_init(prng, &mut sc_rust);


        unsafe {
            falcon_inner_i_shake256_init(&sc_c);
            prng_init_c(prng_c, &sc_c);
        }
    }

    pub fn create_random_prngs() -> (Prng, PrngC) {
        let mut rng = rand::thread_rng();
        let buf: [u8; 512] = core::array::from_fn(|_| rng.gen::<u8>());
        let state_d: [u8; 256] = core::array::from_fn(|_| rng.gen::<u8>());

        let ptr:usize = rand::random();
        let typ:i32 = rand::random();

        let state = State {d: state_d};

        let buf_c: BufC = BufC {d: buf.clone()};
        let state_c: StateC = StateC {d: state_d.clone()};

        let prng = Prng {buf, ptr, state, typ};
        let prng_c = PrngC {buf: buf_c, ptr: ptr as u64, state: state_c, typ};

        return (prng, prng_c);
    }

    unsafe fn test_prng_equality(prng: &Prng, prng_c: &PrngC) -> bool {
        if prng.buf != prng_c.buf.d {
            return false;
        }

        if prng.ptr != prng_c.ptr as usize {
            return false;
        }

        if prng.state.d != prng_c.state.d {
            return false;
        }

        if prng.typ != prng_c.typ {
            return false;
        }

        return true;
    }
}