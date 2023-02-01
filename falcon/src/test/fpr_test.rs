#[cfg(test)]
mod tests {
    use crate::falcon_c::fpr_c::fpr_add_func;
    use crate::fpr::fpr_add;

    #[test]
    fn test_add() {
        let x: u64 = rand::random();
        let y: u64 = rand::random();
        let i = fpr_add(x, y);
        let res = unsafe { fpr_add_func(x, y) };
        assert_eq!(i, res);
    }
}