#[cfg(test)]
mod tests {
    use crate::add;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}