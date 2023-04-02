use falcon::falcon::fpr;

use crate::fpr_masked_deep::{secure_fpr_add, secure_fpr_sub, secure_mul};

pub fn secure_fpc_add<const ORDER: usize>(a_re: &[fpr], a_im: &[fpr], b_re: &[fpr], b_im: &[fpr]) -> ([fpr; ORDER], [fpr; ORDER]) {
    let fpct_re: [fpr; ORDER] = secure_fpr_add(a_re, b_re);
    let fpct_im: [fpr; ORDER] = secure_fpr_add(a_im, b_im);
    return (fpct_re, fpct_im);
}

pub fn secure_fpc_sub<const ORDER: usize>(a_re: &[fpr], a_im: &[fpr], b_re: &[fpr], b_im: &[fpr]) -> ([fpr; ORDER], [fpr; ORDER]) {
    let fpct_re: [fpr; ORDER] = secure_fpr_sub(a_re, b_re);
    let fpct_im: [fpr; ORDER] = secure_fpr_sub(a_im, b_im);
    return (fpct_re, fpct_im);
}

pub fn secure_fpc_mul<const ORDER: usize>(a_re: &[fpr], a_im: &[fpr], b_re: &[fpr], b_im: &[fpr]) -> ([fpr; ORDER], [fpr; ORDER]) {
    let fpct_d_re: [fpr; ORDER] = secure_fpr_sub(
        &secure_mul::<ORDER>(a_re, b_re),
        &secure_mul::<ORDER>(a_im, b_im));
    let fpct_d_im: [fpr; ORDER] = secure_fpr_add(
        &secure_mul::<ORDER>(a_re, b_im),
        &secure_mul::<ORDER>(a_im, b_re));
    return (fpct_d_re, fpct_d_im);
}