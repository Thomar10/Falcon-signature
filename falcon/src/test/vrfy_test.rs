#[cfg(test)]
mod tests {
    use crate::falcon_c::vrfy_c::{mq_add_func, mq_sub_func, mq_montymul_func};
    use crate::vrfy::{mq_add, mq_montymul, mq_sub};

    #[test]
    fn test_monty_mul() {
        for _ in 0..1000 {
            let x: u32 = rand::random();
            let y: u32 = rand::random();
            let res = mq_montymul(x, y);
            let res_c = unsafe { mq_montymul_func(x, y) };
            assert_eq!(res, res_c);
        }
    }

    #[test]
    fn test_monty_add() {
        for _ in 0..1000 {
            let x: u32 = rand::random();
            let y: u32 = rand::random();
            let res = mq_add(x, y);
            let res_c = unsafe { mq_add_func(x, y) };
            assert_eq!(res, res_c);
        }
    }

    #[test]
    fn test_monty_sub() {
        for _ in 0..1000 {
            let x: u32 = rand::random();
            let y: u32 = rand::random();
            let res = mq_sub(x, y);
            let res_c = unsafe { mq_sub_func(x, y) };
            assert_eq!(res, res_c);
        }
    }
}