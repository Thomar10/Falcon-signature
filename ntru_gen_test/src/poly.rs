#[cfg(test)]
mod tests {
/*    use rand::Rng;

    use ntru_gen::poly::poly_mp_set_small;
    use ntru_gen_c::poly::ntrugen_poly_mp_set_small;

    const P: u32 = 12289;

    #[test]
    fn poly_mp_set_small_test() {
        let logn = 10;
        let mut rng = rand::thread_rng();
        let f: [i8; 1024] = core::array::from_fn(|_| rng.gen::<i8>());
        let mut d: [u32; 1024] = [0; 1024];
        let dc: [u32; 1024] = [0; 1024];
        poly_mp_set_small(logn, &mut d, &f, P);
        unsafe { ntrugen_poly_mp_set_small(logn as u32, dc.as_ptr(), f.as_ptr(), P); }
        assert_eq!(d, dc);
    }*/
}