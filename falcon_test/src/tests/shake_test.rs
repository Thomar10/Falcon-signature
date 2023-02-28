#[cfg(test)]
mod tests {
    use rand::Rng;

    use falcon::shake::{i_shake256_extract, i_shake256_flip, i_shake256_init, i_shake256_inject, InnerShake256Context, process_block};

    use crate::falcon_c::shake_c::{falcon_inner_i_shake256_extract, falcon_inner_i_shake256_flip, falcon_inner_i_shake256_init, falcon_inner_i_shake256_inject, InnerShake256Context as InnerShake256ContextC, process_block as process_block_c, St as StC};

    #[test]
    fn test_process_block() {
        for _ in 0..100 {
            let mut random_state_rust: [u64; 25] = rand::random();
            let random_state_c: [u64; 25] = random_state_rust.clone();

            process_block(&mut random_state_rust);
            assert_ne!(random_state_rust, random_state_c);

            unsafe { process_block_c(random_state_c.as_ptr()) };
            assert_eq!(random_state_rust, random_state_c);
        }
    }

    #[test]
    fn test_i_shake_init() {
        let random_state: [u64; 25] = rand::random();
        let random_dptr: u64 = rand::random();

        let mut sc_rust = InnerShake256Context {st: random_state, dptr: random_dptr};

        let sc_c = InnerShake256ContextC {st: StC {a: random_state.clone()}, dptr: random_dptr};

        i_shake256_init(&mut sc_rust);
        unsafe {
            assert!(!test_shake_context_equality(&sc_rust, &sc_c));

            falcon_inner_i_shake256_init(&sc_c as *const InnerShake256ContextC);
            assert!(test_shake_context_equality(&sc_rust, &sc_c));
        };
    }

    #[test]
    fn test_i_shake256_inject_small() {
        for _ in 0..1000 {
            let random_state: [u64; 25] = rand::random();
            let random_dptr: u64 = rand::random();

            let mut sc_rust = InnerShake256Context { st: random_state, dptr: random_dptr };

            let sc_c = InnerShake256ContextC { st: StC { a: random_state.clone() }, dptr: random_dptr };

            i_shake256_init(&mut sc_rust);
            unsafe {
                falcon_inner_i_shake256_init(&sc_c as *const InnerShake256ContextC)
            }

            for _ in 0..20 {
                let input_rust: [u8; 25] = rand::random();
                let input_c: [u8; 25] = input_rust.clone();

                i_shake256_inject(&mut sc_rust, &input_rust);

                unsafe {
                    assert!(!test_shake_context_equality(&sc_rust, &sc_c));

                    falcon_inner_i_shake256_inject(&sc_c as *const InnerShake256ContextC, input_c.as_ptr(), input_c.len() as u64);
                    assert!(test_shake_context_equality(&sc_rust, &sc_c));
                }
            }
        }
    }

    #[test]
    fn test_i_shake256_inject_large() {
        for _ in 0..1000 {
            let random_state: [u64; 25] = rand::random();
            let random_dptr: u64 = rand::random();

            let mut sc_rust = InnerShake256Context { st: random_state, dptr: random_dptr };

            let sc_c = InnerShake256ContextC { st: StC { a: random_state.clone() }, dptr: random_dptr };

            i_shake256_init(&mut sc_rust);
            unsafe {
                falcon_inner_i_shake256_init(&sc_c as *const InnerShake256ContextC)
            }

            let mut rng = rand::thread_rng();

            for _ in 0..20 {
                let input_rust: [u8; 3000] = core::array::from_fn(|_| rng.gen::<u8>());
                let input_c: [u8; 3000] = input_rust.clone();

                i_shake256_inject(&mut sc_rust, &input_rust);

                unsafe {
                    assert!(!test_shake_context_equality(&sc_rust, &sc_c));

                    falcon_inner_i_shake256_inject(&sc_c as *const InnerShake256ContextC, input_c.as_ptr(), input_c.len() as u64);
                    assert!(test_shake_context_equality(&sc_rust, &sc_c));
                }
            }
        }
    }

    #[test]
    fn test_i_shake256_flip() {
        for _ in 0..30 {
            let random_state: [u64; 25] = rand::random();
            let random_dptr: u64 = rand::random();

            let mut sc_rust = InnerShake256Context { st: random_state, dptr: random_dptr };

            let sc_c = InnerShake256ContextC { st: StC { a: random_state.clone() }, dptr: random_dptr };

            i_shake256_init(&mut sc_rust);
            unsafe {
                falcon_inner_i_shake256_init(&sc_c as *const InnerShake256ContextC)
            }

            let input_rust: [u8; 25] = rand::random();
            let input_c: [u8; 25] = input_rust.clone();

            i_shake256_inject(&mut sc_rust, &input_rust);

            unsafe {
                falcon_inner_i_shake256_inject(&sc_c as *const InnerShake256ContextC, input_c.as_ptr(), input_c.len() as u64);
            }

            i_shake256_flip(&mut sc_rust);
            unsafe {
                assert!(!test_shake_context_equality(&sc_rust, &sc_c));

                falcon_inner_i_shake256_flip(&sc_c as *const InnerShake256ContextC);
                assert!(test_shake_context_equality(&sc_rust, &sc_c));
            }
        }
    }

    #[test]
    fn test_i_shake256_extract() {
        for _ in 0..1000 {
            let random_state: [u64; 25] = rand::random();
            let random_dptr: u64 = rand::random();

            let mut sc_rust = InnerShake256Context { st: random_state, dptr: random_dptr };

            let sc_c = InnerShake256ContextC { st: StC { a: random_state.clone() }, dptr: random_dptr };

            i_shake256_init(&mut sc_rust);
            unsafe {
                falcon_inner_i_shake256_init(&sc_c as *const InnerShake256ContextC)
            }

            let mut rng = rand::thread_rng();

            let input_rust: [u8; 3000] = core::array::from_fn(|_| rng.gen::<u8>());
            let input_c: [u8; 3000] = input_rust.clone();

            i_shake256_inject(&mut sc_rust, &input_rust);

            unsafe {
                falcon_inner_i_shake256_inject(&sc_c as *const InnerShake256ContextC, input_c.as_ptr(), input_c.len() as u64);
            }

            i_shake256_flip(&mut sc_rust);
            unsafe {
                falcon_inner_i_shake256_flip(&sc_c as *const InnerShake256ContextC);
            }

            for _ in 0..20 {
                let output_rust = i_shake256_extract(&mut sc_rust, 100);
                let output_c: [u8; 100] = [0; 100];
                unsafe {
                    falcon_inner_i_shake256_extract(&sc_c as *const InnerShake256ContextC, output_c.as_ptr(), 100);
                }

                assert_eq!(output_rust.as_slice(), output_c);
            }
        }
    }

    unsafe fn test_shake_context_equality(sc_rust: &InnerShake256Context, sc_c: &InnerShake256ContextC) -> bool {
        if sc_rust.dptr != sc_c.dptr {
            return false;
        }

        if sc_rust.st != sc_c.st.a {
            return false;
        }

        return true;
    }
}