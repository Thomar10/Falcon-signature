#[cfg(test)]
mod tests {
    use crate::falcon_c::fpr_c::{fpr_add_func, fpr_add_inter_c_xu};
    use crate::falcon_c::fpr_c::fpr_norm as norm_c;

    use crate::fpr::{fpr_add, fpr_add_inter, fpr_norm64};

    /*
    #[test]
    fn test_add() {
        let x: u64 = rand::random();
        let y: u64 = rand::random();
        let i = fpr_add(x, y);
        let res = unsafe { fpr_add_func(x, y) };
        assert_eq!(i, res);
    } */

    #[test]
    fn test_fpr_norm() {
        let mut i = 50;
        while i > 0 {
            let m_c: u64 = rand::random();
            let e_c: i32 = rand::random();

            let m_r: u64 = m_c.clone();
            let e_r: i32 = e_c.clone();

            let (m, e) = fpr_norm64(m_r, e_r);
            unsafe { norm_c(m_c, e_c); }
            assert_eq!(m, m_c);
            assert_eq!(e, e_c);
            i -= 1;
        }
    }

    #[test]
    fn test_fpr_add_inter() {
        let x_r: u64 = rand::random();
        let y_r: u64 = rand::random();
        let (m,  xu, yu, za, x, y, cs, ex, ey, sx, sy, cc) = fpr_add_inter(x_r, y_r);
        let x_c: u64 = x_r.clone();
        let y_c: u64 = y_r.clone();

        let xu_c = unsafe { fpr_add_inter_c_xu(x_c, y_c) };
        assert_eq!(xu_c, xu);
    }
}