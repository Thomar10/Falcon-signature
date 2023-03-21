use crate::ntru::NtruProfile;

pub const FALCON_256: NtruProfile = NtruProfile {
    q: 12289,
    min_logn: 2,
    max_logn: 8,
    max_bl_small: [1, 1, 2, 3, 4, 8, 14, 27, 53, 104, 207],
    max_bl_large: [1, 2, 3, 6, 11, 21, 40, 78, 155, 308],
    word_win: [1, 1, 2, 2, 2, 3, 3, 4, 5, 7],
    reduce_bits: 16,
    coeff_FG_limit: [0, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127],
    min_save_fg: [0, 0, 1, 2, 2, 2, 2, 2, 2, 3, 3],
};

pub const FALCON_512: NtruProfile = NtruProfile {
    q: 12289,
    min_logn: 9,
    max_logn: 9,
    max_bl_small: [1, 1, 2, 3, 4, 8, 14, 27, 53, 104, 207],
    max_bl_large: [1, 2, 3, 6, 11, 21, 40, 78, 155, 308],
    word_win: [1, 1, 2, 2, 2, 3, 3, 4, 5, 7],
    reduce_bits: 13,
    coeff_FG_limit: [0, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127],
    min_save_fg: [0, 0, 1, 2, 2, 2, 2, 2, 2, 3, 3],
};

pub const FALCON_1024: NtruProfile = NtruProfile {
    q: 12289,
    min_logn: 10,
    max_logn: 10,
    max_bl_small: [1, 1, 2, 3, 4, 8, 14, 27, 53, 104, 207],
    max_bl_large: [1, 2, 3, 6, 11, 21, 40, 78, 155, 308],
    word_win: [1, 1, 2, 2, 2, 3, 3, 4, 5, 7],
    reduce_bits: 11,
    coeff_FG_limit: [0, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127],
    min_save_fg: [0, 0, 1, 2, 2, 2, 2, 2, 2, 3, 3],
};

pub const GAUSS_FALCON_256: [u16; 49] = [
    24,
    1, 3, 6, 11, 22, 40, 73, 129,
    222, 371, 602, 950, 1460, 2183, 3179, 4509,
    6231, 8395, 11032, 14150, 17726, 21703, 25995, 30487,
    35048, 39540, 43832, 47809, 51385, 54503, 57140, 59304,
    61026, 62356, 63352, 64075, 64585, 64933, 65164, 65313,
    65406, 65462, 65495, 65513, 65524, 65529, 65532, 65534
];

pub const GAUSS_FALCON_512: [u16; 35] = [
    17,
    1, 4, 11, 28, 65, 146, 308, 615,
    1164, 2083, 3535, 5692, 8706, 12669, 17574, 23285,
    29542, 35993, 42250, 47961, 52866, 56829, 59843, 62000,
    63452, 64371, 64920, 65227, 65389, 65470, 65507, 65524,
    65531, 65534
];

pub const GAUSS_FALCON_1024: [u16; 25] = [
    12,
    2, 8, 28, 94, 280, 742, 1761, 3753,
    7197, 12472, 19623, 28206, 37329, 45912, 53063, 58338,
    61782, 63774, 64793, 65255, 65441, 65507, 65527, 65533
];