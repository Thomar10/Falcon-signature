#[cfg(test)]
mod tests {
    use crate::tests::test_falcon::run_falcon_tests;

    #[test]
    fn run_all_tests() {
        println!("Running test_falcon!");
        run_falcon_tests();
        assert_eq!(1, 1);
    }
}
pub mod fft_test;
pub mod fpr_test;
pub mod shake_test;
pub mod rng_test;
pub mod sign_test;
pub mod keygen_test;
pub mod vrfy_test;
pub mod codec_test;
pub mod nist_test;
pub mod katrng_test;
pub mod common_test;
pub mod test_falcon;
