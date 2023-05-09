#![no_std]

pub mod random;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
extern crate std;
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
