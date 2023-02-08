#[link(name = "fpr", kind = "static")]

extern "C" {
    pub fn fpr_add_func(x: u64, y: u64) -> u64;
    pub fn fpr_mul_func(x: u64, y: u64) -> u64;
    pub fn fpr_div_func(x: u64, y: u64) -> u64;
    pub fn fpr_sqrt_func(x: u64) -> u64;
    pub fn fpr_expm_p63_func(x: u64, ccs: u64) -> u64;
    pub fn fpr_scaled_func(i: i64, sc: i32) -> u64;
    pub fn fpr_trunc_func(x: u64) -> i64;
    pub fn fpr_sqr_func(x: u64) -> u64;
    pub fn fpr_inv_func(x: u64) -> u64;
    pub fn fpr_lt_func(x: u64, y: u64) -> i32;
    pub fn fpr_double_func(x: u64) -> u64;
    pub fn fpr_half_func(x: u64) -> u64;
    pub fn fpr_neg_func(x: u64) -> u64;
    pub fn fpr_sub_func(x: u64, y: u64) -> u64;
    pub fn fpr_floor_func(x: u64) -> i64;
    pub fn fpr_rint_func(x: u64) -> i64;
    pub fn fpr_of_func(x: i64) -> u64;
    pub fn fpr_ulsh_func(x: u64, n: i32) -> u64;
    pub fn fpr_irsh_func(x: i64, n: i32) -> i64;
    pub fn fpr_ursh_func(x: u64, n: i32) -> u64;
}