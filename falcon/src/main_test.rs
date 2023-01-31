#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
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
        let mut myStruct = inner_shake256_context {
            st: MyUnion{
                dbuf: [0; 200],
            },
            dptr: 0,
        };

        unsafe { i_shake256_init(**myStruct.borrow()) };
    }
}