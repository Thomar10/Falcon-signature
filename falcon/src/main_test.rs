#[cfg(test)]
mod tests {
    use crate::addd;
    use crate::falcon_c::fpr_c::fpr_mul_func;
    use crate::falcon_c::shake_c::{i_shake256_init, inner_shake256_context, MyUnion, process_block};

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
}