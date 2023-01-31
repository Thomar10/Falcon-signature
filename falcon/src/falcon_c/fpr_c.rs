#[link(name = "fpr", kind = "static")]

extern "C" {

    pub fn fpr_add_func(x: u64, y: u64) -> u64;
    pub fn fpr_mul_func(x: u64, y: u64) -> u64;
    pub fn fpr_div_func(x: u64, y: u64) -> u64;
    pub fn fpr_sqrt_func(x: u64) -> u64;
    pub fn fpr_expm_p63_func(x: u64, ccs: u64) -> u64;
    pub fn fpr_scaled_func(i: u64, sc: i32) -> u64;
}