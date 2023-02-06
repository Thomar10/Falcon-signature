#[cfg(test)]
mod tests {
    // use crate::addd;
    // use crate::falcon_c::common_c::hash_to_point_vartime;
    // use crate::falcon_c::fpr_c::fpr_mul_func;
    // use crate::falcon_c::nist_c::crypto_sign_keypair;
    // use crate::falcon_c::rng_c::{Buf, Prng, prng_init, prng_refill, State};
    // use crate::falcon_c::shake_c::{i_shake256_extract, i_shake256_flip, i_shake256_init, i_shake256_inject, InnerShake256Context, MyUnion, process_block};
    //
    // #[test]
    // fn test_add() {
    //     let i = 3;
    //     assert_eq!(addd(1, 2), i);
    // }
    //
    // #[test]
    // fn fpr_test_mul() {
    //     let res = unsafe { fpr_mul_func(2, 2) };
    //
    //     assert_eq!(0, res);
    // }
    //
    // #[test]
    // fn shake_process_block() {
    //     let mut state: [u64; 64] = [7; 64];
    //     unsafe { process_block(state.as_ptr()) };
    // }
    //
    // #[test]
    // fn shake_i_shake256_init() {
    //     let myStruct = InnerShake256Context {
    //         st: MyUnion {
    //             a: [2; 25],
    //         },
    //         dptr: 10,
    //     };
    //
    //
    //     unsafe { i_shake256_init(&myStruct) };
    //
    //     assert_ne!(myStruct.dptr, 10);
    //     assert_eq!(myStruct.dptr, 0);
    //     unsafe { assert_eq!(myStruct.st.a, [0; 25]) };
    //     unsafe { assert_ne!(myStruct.st.a, [2; 25]) };
    // }
    //
    // #[test]
    // fn shake_i_shake256_inject() {
    //     let myStruct = InnerShake256Context {
    //         st: MyUnion {
    //             a: [0; 25],
    //         },
    //         dptr: 0,
    //     };
    //
    //     const inn: u8 = 12;
    //     let len: u64 = 256;
    //
    //     unsafe { i_shake256_inject(&myStruct, &inn, 256) }
    // }
    //
    // #[test]
    // fn shake_i_shake256_flip() {
    //     let myStruct = InnerShake256Context {
    //         st: MyUnion {
    //             a: [0; 25],
    //         },
    //         dptr: 0,
    //     };
    //
    //     unsafe { i_shake256_flip(&myStruct) }
    // }
    //
    // // #[test]
    // // fn common_hash_to_point_vartime() {
    // //     let myStruct = InnerShake256Context {
    // //         st: MyUnion {
    // //             a: [0; 25],
    // //         },
    // //         dptr: 0,
    // //     };
    // //
    // //     let x: u16 = 32;
    // //     let logn: u32 = 16;
    // //
    // //     unsafe { hash_to_point_vartime(&myStruct, &x, logn) }
    // // }
    //
    // // #[test]
    // // fn shake_i_shake256_extract() {
    // //     let myStruct = InnerShake256Context {
    // //         st: MyUnion {
    // //             a: [0; 25],
    // //         },
    // //         dptr: 0,
    // //     };
    // //
    // //     unsafe { i_shake256_init(&myStruct) }
    // //
    // //     const inn: u8 = 12;
    // //     let len: u64 = 256;
    // //
    // //     unsafe { i_shake256_inject(&myStruct, &inn, 256) }
    // //
    // //     let out: u8 = 12;
    // //     let len: u64 = 256;
    // //
    // //     unsafe { i_shake256_extract(&myStruct, &out, len) }
    // // }
    //
    // #[test]
    // fn prng_init_test() {
    //     let shake = InnerShake256Context {
    //         st: MyUnion {
    //             a: [0; 25],
    //         },
    //         dptr: 0,
    //     };
    //     let Prng = Prng {
    //         buf: Buf {
    //             d: [0; 512]
    //         },
    //         ptr: 0,
    //         state: State {
    //             d: [0; 256]
    //         },
    //         typ: 0,
    //     };
    //
    //     unsafe { prng_init(&Prng, &shake); }
    // }
    //
    // #[test]
    // fn prng_refill_test() {
    //     let Prng = Prng {
    //         buf: Buf {
    //             d: [0; 512]
    //         },
    //         ptr: 10,
    //         state: State {
    //             d: [0; 256]
    //         },
    //         typ: 0,
    //     };
    //
    //     unsafe { prng_refill(&Prng); }
    //     assert_eq!(Prng.ptr, 0);
    // }

    use crate::falcon_c::nist_c::crypto_sign_keypair;

    #[test]
    fn create_keypair() {
        let pk = [0u8; 2000];
        let sk = [0u8; 2000];

        unsafe { crypto_sign_keypair(pk.as_ptr(), sk.as_ptr()); }
        assert_eq!(pk, [0u8; 2000]);
    }
}