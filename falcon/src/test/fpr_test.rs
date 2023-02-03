#[cfg(test)]
mod tests {
    use crate::falcon_c::fpr_c::{fpr_add_func, fpr_div_func, fpr_double_func, fpr_expm_p63_func, fpr_floor_func, fpr_half_func, fpr_inv_func, fpr_irsh_func, fpr_lt_func, fpr_mul_func, fpr_neg_func, fpr_of_func, fpr_rint_func, fpr_scaled_func, fpr_sqr_func, fpr_sqrt_func, fpr_sub_func, fpr_trunc_func, fpr_ulsh_func, fpr_ursh_func};
    use crate::fpr::{fpr_add, fpr_div, fpr_double, fpr_expm_p63, fpr_floor, fpr_half, fpr_inv, fpr_irsh, fpr_lt, fpr_mul, fpr_neg, fpr_of, fpr_rint, fpr_scaled, fpr_sqr, fpr_sqrt, fpr_sub, fpr_trunc, fpr_ulsh, fpr_ursh};


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

    #[test]
    fn test_fpr_div() {
        let mut i = 20000;
        while i > 0 {
            let x: u64 = rand::random();
            let y: u64 = rand::random();
            let res = fpr_div(x, y);
            let res_c = unsafe { fpr_div_func(x, y) };
            assert_eq!(res, res_c);
            i -= 1;
        }
    }

    #[test]
    fn test_fpr_ursh() {
        let mut i = 20000;
        while i > 0 {
            let x: u64 = rand::random();
            let n: i32 = rand::random();
            let res = fpr_ursh(x, n);
            let res_c = unsafe { fpr_ursh_func(x, n) };
            assert_eq!(res, res_c);
            i -= 1;
        }
    }

    #[test]
    fn test_fpr_ulsh() {
        let mut i = 20000;
        while i > 0 {
            let x: u64 = rand::random();
            let n: i32 = rand::random();
            let res = fpr_ulsh(x, n);
            let res_c = unsafe { fpr_ulsh_func(x, n) };
            assert_eq!(res, res_c);
            i -= 1;
        }
    }

    #[test]
    fn test_fpr_irsh() {
        let mut i = 20000;
        while i > 0 {
            let x: i64 = rand::random();
            let n: i32 = rand::random();
            let res = fpr_irsh(x, n);
            let res_c = unsafe { fpr_irsh_func(x, n) };
            assert_eq!(res, res_c);
            i -= 1;
        }
    }

    #[test]
    fn test_fpr_of() {
        let mut i = 20000;
        while i > 0 {
            let x: i64 = rand::random();
            let res = fpr_of(x);
            let res_c = unsafe { fpr_of_func(x) };
            assert_eq!(res, res_c);
            i -= 1;
        }
    }

    #[test]
    fn test_fpr_rint() {
        let mut i = 20000;
        while i > 0 {
            let x: u64 = rand::random();
            let res = fpr_rint(x);
            let res_c = unsafe { fpr_rint_func(x) };
            assert_eq!(res, res_c);
            i -= 1;
        }
    }

    #[test]
    fn test_fpr_floor() {
        let mut i = 20000;
        while i > 0 {
            let x: u64 = rand::random();
            let res = fpr_floor(x);
            let res_c = unsafe { fpr_floor_func(x) };
            assert_eq!(res, res_c);
            i -= 1;
        }
    }

    #[test]
    fn test_fpr_sub() {
        let mut i = 20000;
        while i > 0 {
            let x: u64 = rand::random();
            let y: u64 = rand::random();
            let res = fpr_sub(x, y);
            let res_c = unsafe { fpr_sub_func(x, y) };
            assert_eq!(res, res_c);
            i -= 1;
        }
    }

    #[test]
    fn test_fpr_neg() {
        let mut i = 20000;
        while i > 0 {
            let x: u64 = rand::random();
            let res = fpr_neg(x);
            let res_c = unsafe { fpr_neg_func(x) };
            assert_eq!(res, res_c);
            i -= 1;
        }
    }

    #[test]
    fn test_fpr_half() {
        let mut i = 20000;
        while i > 0 {
            let x: u64 = rand::random();
            let res = fpr_half(x);
            let res_c = unsafe { fpr_half_func(x) };
            assert_eq!(res, res_c);
            i -= 1;
        }
    }

    #[test]
    fn test_fpr_double() {
        let mut i = 20000;
        while i > 0 {
            let x: u64 = rand::random();
            let res = fpr_double(x);
            let res_c = unsafe { fpr_double_func(x) };
            assert_eq!(res, res_c);
            i -= 1;
        }
    }

    #[test]
    fn test_fpr_sqr() {
        let mut i = 20000;
        while i > 0 {
            let x: u64 = rand::random();
            let res = fpr_sqr(x);
            let res_c = unsafe { fpr_sqr_func(x) };
            assert_eq!(res, res_c);
            i -= 1;
        }
    }

    #[test]
    fn test_fpr_inv() {
        let mut i = 20000;
        while i > 0 {
            let x: u64 = rand::random();
            let res = fpr_inv(x);
            let res_c = unsafe { fpr_inv_func(x) };
            assert_eq!(res, res_c);
            i -= 1;
        }
    }

    #[test]
    fn test_fpr_lt() {
        let mut i = 20000;
        while i > 0 {
            let x: u64 = rand::random();
            let y: u64 = rand::random();
            let res = fpr_lt(x, y);
            let res_c = unsafe { fpr_lt_func(x, y) };
            assert_eq!(res, res_c);
            i -= 1;
        }
    }
}