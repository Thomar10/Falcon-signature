#[cfg(test)]
mod tests {
    use std::ptr::null;

    use rand::Rng;

    use crate::falcon_c::nist_c::{randombytes_func, randombytes_init_func};
    use crate::katrng::{randombytes, randombytes_init};

    #[test]
    fn test_randombytes() {
        let mut entropy: [u8; 48] = [0; 48];
        let entropy_c: [u8; 48] = [0; 48];
        let mut seed: [u8; 48] = [0; 48];
        let seed_c: [u8; 48] = [0; 48];
        unsafe { randombytes_init_func(entropy_c.as_ptr(), null(), 256); }
        randombytes_init(&mut entropy);
        for _ in 0..10 {
            unsafe { randombytes_func(seed_c.as_ptr(), 48); }
            randombytes(&mut seed);
            assert_eq!(seed, seed_c);
        }
    }

    #[test]
    fn test_randombytes_init() {
        for _ in 0..10 {
            let mut rng = rand::thread_rng();
            let mut entropy: [u8; 48] = core::array::from_fn(|_| rng.gen::<u8>());
            let entropy_c: [u8; 48] = entropy.clone();
            let mut seed: [u8; 48] = [0; 48];
            let seed_c: [u8; 48] = [0; 48];
            unsafe { randombytes_init_func(entropy_c.as_ptr(), null(), 256); }
            randombytes_init(&mut entropy);
            assert_eq!(entropy, entropy_c);
            for _ in 0..10 {
                // Call random bytes to see if init was the same.
                unsafe { randombytes_func(seed_c.as_ptr(), 48); }
                randombytes(&mut seed);
                assert_eq!(seed, seed_c);
            }
        }
    }
}