use crate::shake::InnerShake256Context;
#[macro_export]
macro_rules! falcon_tmpsize_keygen {
    ($arg:expr) => {
        if $arg <= 3 {272} else {(28 << $arg) + (3 << $arg + 7)}
    }
}

#[macro_export]
macro_rules! falcon_privatekey_size {
    ($arg:expr) => {
        if $arg <= 3 {3 << $arg}
        else {((10u - (($arg) >> 1)) << ((logn) - 2)) + (1 << ($arg)) + 1}
    }
}

#[macro_export]
macro_rules! falcon_publickey_size {
    ($arg:expr) => {
        if $arg <= 1 {3 << $arg}
        else {((10u - (($arg) >> 1)) << ((logn) - 2)) + (1 << ($arg))} + 1
    }
}

#[macro_export]
macro_rules! falcon_tmpsize_makepub {
    ($arg:expr) => {(78u << $arg) + 7}
}

#[macro_export]
macro_rules! falcon_tmpsize_signtree {
    ($arg:expr) => {(50u << $arg) + 7}
}

#[macro_export]
macro_rules! falcon_tmpsize_expandprivate {
    ($arg:expr) => {(52u << $arg) + 7}
}

#[macro_export]
macro_rules! falcon_tmpsize_expanded_key_size {
    ($arg:expr) => {((8u * $arg + 40) << $arg) + 8}
}

#[macro_export]
macro_rules! falcon_tmpsize_verify {
    ($arg:expr) => {(8u << $arg) + 1}
}

pub fn falcon_keygen_make(rng: &mut InnerShake256Context, logn: u32, private_key: &mut [u8],
                          private_len: usize, public_key: &mut [u8], public_len: usize,
                          tmp: &mut [u8], tmp_len: usize) -> i32 {
    0
}

pub fn falcon_make_public(private_key: &mut [u8], private_len: usize,
                          public_key: &mut [u8], public_len: usize,
                          tmp: &mut [u8], tmp_len: usize) -> i32 {
    0
}

pub fn falcon_get_logn(obj: &mut [u8], len: usize) -> i32 {
    0
}

pub fn falcon_sign_dyn(rng: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize,
                       signature_type: i32, private_key: &mut [u8],
                       private_len: usize, public_key: &mut [u8], public_len: usize,
                       tmp: &mut [u8], tmp_len: usize) -> i32 {
    0
}

pub fn falcon_expand_privatekey(expanded_key: &mut [u8], expanded_len: usize,
                                private_key: &mut [u8], private_len: usize,
                                tmp: &mut [u8], tmp_len: usize) -> i32 {
    0
}

pub fn falcon_sign_tree(rng: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize,
                        signature_type: i32, expanded_key: &mut [u8],
                        expanded_len: usize, data: &mut [u8], data_len: usize,
                        tmp: &mut [u8], tmp_len: usize) -> i32 {
    0
}

pub fn falcon_sign_start(rng: &mut InnerShake256Context, nonce: &mut [u8],
                         hash_data: &mut InnerShake256Context) -> i32 {
    0
}

pub fn falcon_sign_dyn_finish(rng: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize,
                              signature_type: i32, private_key: &mut [u8],
                              private_len: usize,
                              hash_data: &mut InnerShake256Context, nonce: &mut [u8],
                              tmp: &mut [u8], tmp_len: usize) -> i32 {
    0
}

pub fn falcon_sign_tree_finish(rng: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize,
                               signature_type: i32, expanded_key: &mut [u8],
                               hash_data: &mut InnerShake256Context,
                               nonce: &mut [u8],
                               tmp: &mut [u8], tmp_len: usize) -> i32 {
    0
}

pub fn falcon_verify(signature: &mut [u8], signature_len: usize, signature_type: i32,
                     public_key: &mut [u8], public_len: usize,
                     data: &mut [u8], data_len: usize,
                     tmp: &mut [u8], tmp_len: usize) -> i32 {
    0
}

pub fn falcon_verify_start(hash_data: &mut InnerShake256Context, signature: &mut [u8], signature_len: usize) -> i32 {
    0
}

pub fn falcon_verify_finish(signature: &mut [u8], signature_len: usize, signature_type: i32,
                            public_key: &mut [u8], public_len: usize,
                            hash_data: &mut InnerShake256Context,
                            tmp: &mut [u8], tmp_len: usize) -> i32 {
    0
}