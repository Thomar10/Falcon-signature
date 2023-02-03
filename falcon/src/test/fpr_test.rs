#[cfg(test)]
mod tests {
    use crate::falcon_c::fpr_c::{fpr_add_func, fpr_expm_p63_func, fpr_mul_func, fpr_scaled_func, fpr_sqrt_func, fpr_trunc_func};

    use crate::fpr::{fpr_add, fpr_expm_p63, fpr_mul, fpr_scaled, fpr_sqrt, fpr_trunc};


    #[test]
    fn test_add() {
        let mut i = 20000;
        while i > 0 {
            let x: u64 = rand::random();
            let y: u64 = rand::random();
            let res = fpr_add(x, y);
            let res_c = unsafe { fpr_add_func(x, y) };
            assert_eq!(res, res_c);
            i -= 1;
        }
    }

    #[test]
    fn test_fpr_scaled() {
        let mut i = 20000;
        while i > 0 {
            let x: i64 = rand::random();
            let y: i32 = rand::random();
            let res = fpr_scaled(x, y);
            let res_c = unsafe { fpr_scaled_func(x, y) };
            assert_eq!(res, res_c);
            i -= 1;
        }
    }

    #[test]
    fn test_fpr_trunc() {
        let mut i = 20000;
        while i > 0 {
            let x: u64 = rand::random();
            let res = fpr_trunc(x);
            let res_c = unsafe { fpr_trunc_func(x) };
            assert_eq!(res, res_c);
            i -= 1;
        }
    }

    #[test]
    fn test_fpr_mul() {
        let mut i = 20000;
        while i > 0 {
            let x: u64 = rand::random();
            let y: u64 = rand::random();
            let res = fpr_mul(x, y);
            let res_c = unsafe { fpr_mul_func(x, y) };
            assert_eq!(res, res_c);
            i -= 1;
        }
    }

    #[test]
    fn test_fpr_expm_p63() {
        let mut i = 20000;
        while i > 0 {
            let x: u64 = rand::random();
            let ccs: u64 = rand::random();
            let res = fpr_expm_p63(x, ccs);
            let res_c = unsafe { fpr_expm_p63_func(x, ccs) };
            assert_eq!(res, res_c);
            i -= 1;
        }
    }

    #[test]
    fn test_fpr_sqrt() {
        let mut i = 20000;
        while i > 0 {
            let x: u64 = rand::random();
            let res = fpr_sqrt(x);
            let res_c = unsafe { fpr_sqrt_func(x) };
            assert_eq!(res, res_c);
            i -= 1;
        }
    }
}