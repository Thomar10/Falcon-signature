#[link(name = "fpr", kind = "static")]

extern "C" {
    #[allow(dead_code)]
    pub fn fpr_add_func(x: u64, y: u64) -> u64;
    #[allow(dead_code)]
    pub fn fpr_norm(m: u64, e: i32) -> u64;
    #[allow(dead_code)]
    pub fn fpr_mul_func(x: u64, y: u64) -> u64;
    #[allow(dead_code)]
    pub fn fpr_div_func(x: u64, y: u64) -> u64;
    #[allow(dead_code)]
    pub fn fpr_sqrt_func(x: u64) -> u64;
    #[allow(dead_code)]
    pub fn fpr_expm_p63_func(x: u64, ccs: u64) -> u64;
    #[allow(dead_code)]
    pub fn fpr_scaled_func(i: u64, sc: i32) -> u64;

    #[allow(dead_code)]
    pub fn fpr_add_inter_c_xu(x: u64, y: u64) -> u64;
}