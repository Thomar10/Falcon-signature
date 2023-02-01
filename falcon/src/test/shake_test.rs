#[cfg(test)]
mod tests {
    use crate::falcon_c::shake_c::{falcon_inner_i_shake256_init, process_block as process_block_c};
    use crate::shake::{i_shake256_init, InnerShake256Context, process_block, St};

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
    fn i_shake_init() {
        let random_state: [u64; 25] = rand::random();
        let random_dptr: u64 = rand::random();

        let st = St {a: random_state};

        let sc_rust = InnerShake256Context {st, dptr: random_dptr};
        
        let sc_c = sc_rust.clone();

        i_shake256_init(sc_rust);
        assert_ne!(sc_rust, sc_c);

        unsafe { falcon_inner_i_shake256_init(sc_c) };
        assert_eq!(sc_rust, sc_c);
    }
}