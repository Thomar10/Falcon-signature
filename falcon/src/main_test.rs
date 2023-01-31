#[cfg(test)]
mod tests {
    use crate::addd;
    use crate::falcon_c::nist_c::add;

    #[test]
    fn test_add() {
        let i = 3;
        assert_eq!(addd(1, 2), i);
    }
}