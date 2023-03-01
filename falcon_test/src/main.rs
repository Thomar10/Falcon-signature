use falcon::fpr::{fpr_add, fpr_of, fpr_rint, fpr_sub};

mod katrng;
mod nist;
mod tests;
mod kat_tests;


fn main() {
    let y: i32 = rand::random();
    let k: i32 = rand::random();
    let x_fpr = fpr_of(5000);
    let z_fpr = fpr_of(5000);
    let random_fpr = fpr_of(y as i64);
    let random2_fpr = fpr_of(k as i64);
    let x_mask_fpr = fpr_sub(x_fpr, random_fpr);
    let z_mask_fpr = fpr_sub(z_fpr, random2_fpr);
    // println!("{}", x_fpr);
    // println!("{}", fpr_rint(x_fpr));
    // println!("{}", x_mask_fpr);
    // println!("{}", random_fpr);
    // println!("{}", fpr_add(x_mask_fpr, random_fpr));
    let random_fpr = fpr_add(random2_fpr, random_fpr);
    let x_mask_fpr = fpr_add(x_mask_fpr, z_mask_fpr);
    println!("{}", fpr_rint(fpr_add(x_mask_fpr, random_fpr)));
}