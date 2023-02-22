#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::common::{hash_to_point_ct, hash_to_point_vartime, is_short, is_short_half};
    use crate::falcon_c::common_c::{hash_to_point_ct_func, hash_to_point_vartime_func, is_short_func, is_short_half_func};
    use crate::falcon_c::shake_c::{falcon_inner_i_shake256_init, falcon_inner_i_shake256_inject, InnerShake256Context as InnerShake256ContextC, St as StC};
    use crate::shake::{i_shake256_init, i_shake256_inject, InnerShake256Context};

    #[test]
    fn test_hash_to_point_vartime() {
        for _ in 0..100 {
            for logn in 1..11 {
                let mut x: [u16; 4096] = [0; 4096];
                let xc: [u16; 4096] = [0; 4096];
                let (mut sc_rust, sc_c) = init_shake_with_random_context();
                hash_to_point_vartime(&mut sc_rust, &mut x, logn);
                unsafe { hash_to_point_vartime_func(&sc_c, xc.as_ptr(), logn) };
                assert_eq!(x, xc);
            }
        }
    }

    #[test]
    fn test_hash_to_point_ct() {
        for _ in 0..100 {
            for logn in 1..11 {
                let mut x: [u16; 4096] = [0; 4096];
                let xc: [u16; 4096] = [0; 4096];
                let mut tmp: [u8; 10000] = [0; 10000];
                let tmpc: [u8; 10000] = [0; 10000];
                let (mut sc_rust, sc_c) = init_shake_with_random_context();
                hash_to_point_ct(&mut sc_rust, &mut x, logn, &mut tmp);
                unsafe { hash_to_point_ct_func(&sc_c, xc.as_ptr(), logn, tmpc.as_ptr()) };
                assert_eq!(x, xc);
            }
        }
    }

    #[test]
    fn test_is_short() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut s1: [i16; 512] = core::array::from_fn(|_| rng.gen::<i16>());
                let s1_c = s1.clone();
                let mut s2: [i16; 512] = core::array::from_fn(|_| rng.gen::<i16>());
                let s2_c = s2.clone();
                let res = is_short(s1.as_mut_ptr(), &mut s2, logn);
                let res_c = unsafe { is_short_func(s1_c.as_ptr(), s2_c.as_ptr(), logn) };
                assert_eq!(res, res_c);
            }
        }
    }

    #[test]
    fn test_is_short_half() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let sqn: u32 = rand::random();
                let mut s2: [i16; 512] = core::array::from_fn(|_| rng.gen::<i16>());
                let s2_c = s2.clone();
                let res = is_short_half(sqn, &mut s2, logn);
                let res_c = unsafe { is_short_half_func(sqn, s2_c.as_ptr(), logn) };
                assert_eq!(res, res_c);
            }
        }
    }

    // TODO import instead how??
    pub fn init_shake_with_random_context() -> (InnerShake256Context, InnerShake256ContextC) {
        let random_state: [u64; 25] = rand::random();
        let random_dptr: u64 = rand::random();
        let mut sc_rust = InnerShake256Context { st: random_state, dptr: random_dptr };
        let sc_c = InnerShake256ContextC { st: StC { a: random_state.clone() }, dptr: random_dptr };
        unsafe { falcon_inner_i_shake256_init(&sc_c as *const InnerShake256ContextC) }
        i_shake256_init(&mut sc_rust);
        let input_rust: [u8; 25] = rand::random();
        let input_c: [u8; 25] = input_rust.clone();
        i_shake256_inject(&mut sc_rust, &input_rust);
        unsafe { falcon_inner_i_shake256_inject(&sc_c as *const InnerShake256ContextC, input_c.as_ptr(), input_c.len() as u64) }
        (sc_rust, sc_c)
    }
}