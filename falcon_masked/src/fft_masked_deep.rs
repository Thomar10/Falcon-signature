pub fn fpc_add(a_re: &[fpr], a_im: &[fpr], b_re: &[fpr], b_im: &[fpr]) -> ([fpr; 2], [fpr; 2]) {
    let fpct_re: [fpr; 2] = fpr_add(a_re, b_re);
    let fpct_im: [fpr; 2] = fpr_add(a_im, b_im);
    return (fpct_re, fpct_im);
}