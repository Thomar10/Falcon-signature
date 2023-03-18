#[cfg(test)]
mod tests {
    use ntru_gen::mp31::mp_div;
    use ntru_gen_c::mp31::ntrugen_mp_div;

    const P: u32 = 12289;

    #[test]
    fn div() {
        for _ in 0..1000 {
            let x: u32 = rand::random();
            let y: u32 = rand::random();
            assert_eq!(mp_div(x, y, P), unsafe { ntrugen_mp_div(x, y, P) });
        }
    }
}