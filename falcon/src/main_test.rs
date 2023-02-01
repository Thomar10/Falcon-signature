#[cfg(test)]
mod tests {
    use crate::addd;
    use crate::falcon_c::fpr_c::fpr_mul_func;
    use crate::falcon_c::rng_c::{Buf, prng, prng_init, prng_refill, State};
    use crate::falcon_c::shake_c::{i_shake256_flip, i_shake256_init, i_shake256_inject, inner_shake256_context, MyUnion, process_block};

    #[test]
    fn test_add() {
        let i = 3;
        assert_eq!(addd(1, 2), i);
    }

    #[test]
    fn fpr_test_mul() {
        let res = unsafe { fpr_mul_func(2, 2) };

        assert_eq!(0, res);
    }

    #[test]
    fn shake_process_block() {
        let mut state: [u64; 64] = [7; 64];
        unsafe { process_block(state.as_ptr()) };
    }

    #[test]
    fn shake_i_shake256_init() {
        let myStruct = inner_shake256_context {
            st: MyUnion {
                a: [2; 25],
            },
            dptr: 10,
        };


        unsafe { i_shake256_init(&myStruct) };

        assert_ne!(myStruct.dptr, 10);
        assert_eq!(myStruct.dptr, 0);
        unsafe { assert_eq!(myStruct.st.a, [0; 25]) };
        unsafe { assert_ne!(myStruct.st.a, [2; 25]) };
    }

    #[test]
    fn shake_i_shake256_inject() {
        // let myStruct = inner_shake256_context {
        //     st: MyUnion {
        //         a: [0; 25],
        //     },
        //     dptr: 0,
        // };
        //
        // const inn: u8 = 12;
        // let len: u64 = 256;
        //
        // unsafe { i_shake256_inject(&myStruct, *inn, 256) }
    }

    #[test]
    fn shake_i_shake256_flip() {}

    #[test]
    fn shake_i_shake256_extract() {}

    #[test]
    fn prng_init_test() {
        let shake = inner_shake256_context {
            st: MyUnion {
                a: [0; 25],
            },
            dptr: 0,
        };
        let prng = prng {
            buf: Buf {
                d: [0; 512]
            },
            ptr: 0,
            state: State {
                d: [0; 256]
            },
            typ: 0,
        };

        unsafe { prng_init(&prng, &shake); }
    }

    #[test]
    fn prng_refill_test() {
        let prng = prng {
            buf: Buf {
                d: [0; 512]
            },
            ptr: 10,
            state: State {
                d: [0; 256]
            },
            typ: 0,
        };

        unsafe { prng_refill(&prng); }
        assert_eq!(prng.ptr, 0);
    }
}