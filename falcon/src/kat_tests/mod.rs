
mod gen_kat;
mod test_falcon;

#[cfg(test)]
mod tests {
    use crate::kat_tests::gen_kat::genkat;
    use crate::kat_tests::test_falcon::run_falcon_tests;

    #[test]
    fn run_test_falcon() {
       run_falcon_tests();
        assert_eq!(1, 1);
    }

    #[test]
    fn gen_kat() {
        genkat();
        assert_eq!(1, 1);
    }
}
