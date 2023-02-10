#[macro_export]
macro_rules! falcon_tmpsize_keygen {
    ($arg:expr) => {
        if $arg <= 3 {272} else {(28 << $arg) + (3 << $arg + 7)}
    }
}

// pub fn falcon_make_public(pubkey: *mut (), pubkey_len: usize, privkey: *const (), privkey_len: usize, tmp: *mut (), tmp_len: usize) -> bool {
//
//     true
// }