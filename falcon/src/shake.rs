pub struct InnerShake256Context {
    pub st: [u64; 25],
    pub dptr: u64,
}

//Round constants
static RC: [u64; 24] = [
    0x0000000000000001, 0x0000000000008082,
    0x800000000000808A, 0x8000000080008000,
    0x000000000000808B, 0x0000000080000001,
    0x8000000080008081, 0x8000000000008009,
    0x000000000000008A, 0x0000000000000088,
    0x0000000080008009, 0x000000008000000A,
    0x000000008000808B, 0x800000000000008B,
    0x8000000000008089, 0x8000000000008003,
    0x8000000000008002, 0x8000000000000080,
    0x000000000000800A, 0x800000008000000A,
    0x8000000080008081, 0x8000000000008080,
    0x0000000080000001, 0x8000000080008008
];

/*
 * Process the provided state.
 */
pub fn process_block(a: &mut [u64]) -> () {
    let (mut t0, mut t1, mut t2, mut t3, mut t4): (u64, u64, u64, u64, u64);
    let (mut tt0, mut tt1, mut tt2, mut tt3): (u64, u64, u64, u64);
    let (mut t, mut kt): (u64, u64);
    let (mut c0, mut c1, mut c2, mut c3, mut c4, mut bnn): (u64, u64, u64, u64, u64, u64);
    let mut j: usize;

    a[1] = !a[1];
    a[2] = !a[2];
    a[8] = !a[8];
    a[12] = !a[12];
    a[17] = !a[17];
    a[20] = !a[20];

    /*
	 * Invert some words (alternate internal representation, which
	 * saves some operations).
	 */
    j = 0;
    while j < 24 {
        tt0 = a[1] ^ a[6];
        tt1 = a[11] ^ a[16];
        tt0 ^= a[21] ^ tt1;
        tt0 = (tt0 << 1) | (tt0 >> 63);
        tt2 = a[4] ^ a[9];
        tt3 = a[14] ^ a[19];
        tt0 ^= a[24];
        tt2 ^= tt3;
        t0 = tt0 ^ tt2;

        tt0 = a[2] ^ a[7];
        tt1 = a[12] ^ a[17];
        tt0 ^= a[22] ^ tt1;
        tt0 = (tt0 << 1) | (tt0 >> 63);
        tt2 = a[0] ^ a[5];
        tt3 = a[10] ^ a[15];
        tt0 ^= a[20];
        tt2 ^= tt3;
        t1 = tt0 ^ tt2;

        tt0 = a[3] ^ a[8];
        tt1 = a[13] ^ a[18];
        tt0 ^= a[23] ^ tt1;
        tt0 = (tt0 << 1) | (tt0 >> 63);
        tt2 = a[1] ^ a[6];
        tt3 = a[11] ^ a[16];
        tt0 ^= a[21];
        tt2 ^= tt3;
        t2 = tt0 ^ tt2;

        tt0 = a[4] ^ a[9];
        tt1 = a[14] ^ a[19];
        tt0 ^= a[24] ^ tt1;
        tt0 = (tt0 << 1) | (tt0 >> 63);
        tt2 = a[2] ^ a[7];
        tt3 = a[12] ^ a[17];
        tt0 ^= a[22];
        tt2 ^= tt3;
        t3 = tt0 ^ tt2;

        tt0 = a[0] ^ a[5];
        tt1 = a[10] ^ a[15];
        tt0 ^= a[20] ^ tt1;
        tt0 = (tt0 << 1) | (tt0 >> 63);
        tt2 = a[3] ^ a[8];
        tt3 = a[13] ^ a[18];
        tt0 ^= a[23];
        tt2 ^= tt3;
        t4 = tt0 ^ tt2;

        a[0] = a[0] ^ t0;
        a[5] = a[5] ^ t0;
        a[10] = a[10] ^ t0;
        a[15] = a[15] ^ t0;
        a[20] = a[20] ^ t0;
        a[1] = a[1] ^ t1;
        a[6] = a[6] ^ t1;
        a[11] = a[11] ^ t1;
        a[16] = a[16] ^ t1;
        a[21] = a[21] ^ t1;
        a[2] = a[2] ^ t2;
        a[7] = a[7] ^ t2;
        a[12] = a[12] ^ t2;
        a[17] = a[17] ^ t2;
        a[22] = a[22] ^ t2;
        a[3] = a[3] ^ t3;
        a[8] = a[8] ^ t3;
        a[13] = a[13] ^ t3;
        a[18] = a[18] ^ t3;
        a[23] = a[23] ^ t3;
        a[4] = a[4] ^ t4;
        a[9] = a[9] ^ t4;
        a[14] = a[14] ^ t4;
        a[19] = a[19] ^ t4;
        a[24] = a[24] ^ t4;
        a[5] = (a[5] << 36) | (a[5] >> (64 - 36));
        a[10] = (a[10] << 3) | (a[10] >> (64 - 3));
        a[15] = (a[15] << 41) | (a[15] >> (64 - 41));
        a[20] = (a[20] << 18) | (a[20] >> (64 - 18));
        a[1] = (a[1] << 1) | (a[1] >> (64 - 1));
        a[6] = (a[6] << 44) | (a[6] >> (64 - 44));
        a[11] = (a[11] << 10) | (a[11] >> (64 - 10));
        a[16] = (a[16] << 45) | (a[16] >> (64 - 45));
        a[21] = (a[21] << 2) | (a[21] >> (64 - 2));
        a[2] = (a[2] << 62) | (a[2] >> (64 - 62));
        a[7] = (a[7] << 6) | (a[7] >> (64 - 6));
        a[12] = (a[12] << 43) | (a[12] >> (64 - 43));
        a[17] = (a[17] << 15) | (a[17] >> (64 - 15));
        a[22] = (a[22] << 61) | (a[22] >> (64 - 61));
        a[3] = (a[3] << 28) | (a[3] >> (64 - 28));
        a[8] = (a[8] << 55) | (a[8] >> (64 - 55));
        a[13] = (a[13] << 25) | (a[13] >> (64 - 25));
        a[18] = (a[18] << 21) | (a[18] >> (64 - 21));
        a[23] = (a[23] << 56) | (a[23] >> (64 - 56));
        a[4] = (a[4] << 27) | (a[4] >> (64 - 27));
        a[9] = (a[9] << 20) | (a[9] >> (64 - 20));
        a[14] = (a[14] << 39) | (a[14] >> (64 - 39));
        a[19] = (a[19] << 8) | (a[19] >> (64 - 8));
        a[24] = (a[24] << 14) | (a[24] >> (64 - 14));

        bnn = !a[12];
        kt = a[6] | a[12];
        c0 = a[0] ^ kt;
        kt = bnn | a[18];
        c1 = a[6] ^ kt;
        kt = a[18] & a[24];
        c2 = a[12] ^ kt;
        kt = a[24] | a[0];
        c3 = a[18] ^ kt;
        kt = a[0] & a[6];
        c4 = a[24] ^ kt;
        a[0] = c0;
        a[6] = c1;
        a[12] = c2;
        a[18] = c3;
        a[24] = c4;
        bnn = !a[22];
        kt = a[9] | a[10];
        c0 = a[3] ^ kt;
        kt = a[10] & a[16];
        c1 = a[9] ^ kt;
        kt = a[16] | bnn;
        c2 = a[10] ^ kt;
        kt = a[22] | a[3];
        c3 = a[16] ^ kt;
        kt = a[3] & a[9];
        c4 = a[22] ^ kt;
        a[3] = c0;
        a[9] = c1;
        a[10] = c2;
        a[16] = c3;
        a[22] = c4;
        bnn = !a[19];
        kt = a[7] | a[13];
        c0 = a[1] ^ kt;
        kt = a[13] & a[19];
        c1 = a[7] ^ kt;
        kt = bnn & a[20];
        c2 = a[13] ^ kt;
        kt = a[20] | a[1];
        c3 = bnn ^ kt;
        kt = a[1] & a[7];
        c4 = a[20] ^ kt;
        a[1] = c0;
        a[7] = c1;
        a[13] = c2;
        a[19] = c3;
        a[20] = c4;
        bnn = !a[17];
        kt = a[5] & a[11];
        c0 = a[4] ^ kt;
        kt = a[11] | a[17];
        c1 = a[5] ^ kt;
        kt = bnn | a[23];
        c2 = a[11] ^ kt;
        kt = a[23] & a[4];
        c3 = bnn ^ kt;
        kt = a[4] | a[5];
        c4 = a[23] ^ kt;
        a[4] = c0;
        a[5] = c1;
        a[11] = c2;
        a[17] = c3;
        a[23] = c4;
        bnn = !a[8];
        kt = bnn & a[14];
        c0 = a[2] ^ kt;
        kt = a[14] | a[15];
        c1 = bnn ^ kt;
        kt = a[15] & a[21];
        c2 = a[14] ^ kt;
        kt = a[21] | a[2];
        c3 = a[15] ^ kt;
        kt = a[2] & a[8];
        c4 = a[21] ^ kt;
        a[2] = c0;
        a[8] = c1;
        a[14] = c2;
        a[15] = c3;
        a[21] = c4;
        a[0] = a[0] ^ RC[j + 0];

        tt0 = a[6] ^ a[9];
        tt1 = a[7] ^ a[5];
        tt0 ^= a[8] ^ tt1;
        tt0 = (tt0 << 1) | (tt0 >> 63);
        tt2 = a[24] ^ a[22];
        tt3 = a[20] ^ a[23];
        tt0 ^= a[21];
        tt2 ^= tt3;
        t0 = tt0 ^ tt2;

        tt0 = a[12] ^ a[10];
        tt1 = a[13] ^ a[11];
        tt0 ^= a[14] ^ tt1;
        tt0 = (tt0 << 1) | (tt0 >> 63);
        tt2 = a[0] ^ a[3];
        tt3 = a[1] ^ a[4];
        tt0 ^= a[2];
        tt2 ^= tt3;
        t1 = tt0 ^ tt2;

        tt0 = a[18] ^ a[16];
        tt1 = a[19] ^ a[17];
        tt0 ^= a[15] ^ tt1;
        tt0 = (tt0 << 1) | (tt0 >> 63);
        tt2 = a[6] ^ a[9];
        tt3 = a[7] ^ a[5];
        tt0 ^= a[8];
        tt2 ^= tt3;
        t2 = tt0 ^ tt2;

        tt0 = a[24] ^ a[22];
        tt1 = a[20] ^ a[23];
        tt0 ^= a[21] ^ tt1;
        tt0 = (tt0 << 1) | (tt0 >> 63);
        tt2 = a[12] ^ a[10];
        tt3 = a[13] ^ a[11];
        tt0 ^= a[14];
        tt2 ^= tt3;
        t3 = tt0 ^ tt2;

        tt0 = a[0] ^ a[3];
        tt1 = a[1] ^ a[4];
        tt0 ^= a[2] ^ tt1;
        tt0 = (tt0 << 1) | (tt0 >> 63);
        tt2 = a[18] ^ a[16];
        tt3 = a[19] ^ a[17];
        tt0 ^= a[15];
        tt2 ^= tt3;
        t4 = tt0 ^ tt2;

        a[0] = a[0] ^ t0;
        a[3] = a[3] ^ t0;
        a[1] = a[1] ^ t0;
        a[4] = a[4] ^ t0;
        a[2] = a[2] ^ t0;
        a[6] = a[6] ^ t1;
        a[9] = a[9] ^ t1;
        a[7] = a[7] ^ t1;
        a[5] = a[5] ^ t1;
        a[8] = a[8] ^ t1;
        a[12] = a[12] ^ t2;
        a[10] = a[10] ^ t2;
        a[13] = a[13] ^ t2;
        a[11] = a[11] ^ t2;
        a[14] = a[14] ^ t2;
        a[18] = a[18] ^ t3;
        a[16] = a[16] ^ t3;
        a[19] = a[19] ^ t3;
        a[17] = a[17] ^ t3;
        a[15] = a[15] ^ t3;
        a[24] = a[24] ^ t4;
        a[22] = a[22] ^ t4;
        a[20] = a[20] ^ t4;
        a[23] = a[23] ^ t4;
        a[21] = a[21] ^ t4;
        a[3] = (a[3] << 36) | (a[3] >> (64 - 36));
        a[1] = (a[1] << 3) | (a[1] >> (64 - 3));
        a[4] = (a[4] << 41) | (a[4] >> (64 - 41));
        a[2] = (a[2] << 18) | (a[2] >> (64 - 18));
        a[6] = (a[6] << 1) | (a[6] >> (64 - 1));
        a[9] = (a[9] << 44) | (a[9] >> (64 - 44));
        a[7] = (a[7] << 10) | (a[7] >> (64 - 10));
        a[5] = (a[5] << 45) | (a[5] >> (64 - 45));
        a[8] = (a[8] << 2) | (a[8] >> (64 - 2));
        a[12] = (a[12] << 62) | (a[12] >> (64 - 62));
        a[10] = (a[10] << 6) | (a[10] >> (64 - 6));
        a[13] = (a[13] << 43) | (a[13] >> (64 - 43));
        a[11] = (a[11] << 15) | (a[11] >> (64 - 15));
        a[14] = (a[14] << 61) | (a[14] >> (64 - 61));
        a[18] = (a[18] << 28) | (a[18] >> (64 - 28));
        a[16] = (a[16] << 55) | (a[16] >> (64 - 55));
        a[19] = (a[19] << 25) | (a[19] >> (64 - 25));
        a[17] = (a[17] << 21) | (a[17] >> (64 - 21));
        a[15] = (a[15] << 56) | (a[15] >> (64 - 56));
        a[24] = (a[24] << 27) | (a[24] >> (64 - 27));
        a[22] = (a[22] << 20) | (a[22] >> (64 - 20));
        a[20] = (a[20] << 39) | (a[20] >> (64 - 39));
        a[23] = (a[23] << 8) | (a[23] >> (64 - 8));
        a[21] = (a[21] << 14) | (a[21] >> (64 - 14));

        bnn = !a[13];
        kt = a[9] | a[13];
        c0 = a[0] ^ kt;
        kt = bnn | a[17];
        c1 = a[9] ^ kt;
        kt = a[17] & a[21];
        c2 = a[13] ^ kt;
        kt = a[21] | a[0];
        c3 = a[17] ^ kt;
        kt = a[0] & a[9];
        c4 = a[21] ^ kt;
        a[0] = c0;
        a[9] = c1;
        a[13] = c2;
        a[17] = c3;
        a[21] = c4;
        bnn = !a[14];
        kt = a[22] | a[1];
        c0 = a[18] ^ kt;
        kt = a[1] & a[5];
        c1 = a[22] ^ kt;
        kt = a[5] | bnn;
        c2 = a[1] ^ kt;
        kt = a[14] | a[18];
        c3 = a[5] ^ kt;
        kt = a[18] & a[22];
        c4 = a[14] ^ kt;
        a[18] = c0;
        a[22] = c1;
        a[1] = c2;
        a[5] = c3;
        a[14] = c4;
        bnn = !a[23];
        kt = a[10] | a[19];
        c0 = a[6] ^ kt;
        kt = a[19] & a[23];
        c1 = a[10] ^ kt;
        kt = bnn & a[2];
        c2 = a[19] ^ kt;
        kt = a[2] | a[6];
        c3 = bnn ^ kt;
        kt = a[6] & a[10];
        c4 = a[2] ^ kt;
        a[6] = c0;
        a[10] = c1;
        a[19] = c2;
        a[23] = c3;
        a[2] = c4;
        bnn = !a[11];
        kt = a[3] & a[7];
        c0 = a[24] ^ kt;
        kt = a[7] | a[11];
        c1 = a[3] ^ kt;
        kt = bnn | a[15];
        c2 = a[7] ^ kt;
        kt = a[15] & a[24];
        c3 = bnn ^ kt;
        kt = a[24] | a[3];
        c4 = a[15] ^ kt;
        a[24] = c0;
        a[3] = c1;
        a[7] = c2;
        a[11] = c3;
        a[15] = c4;
        bnn = !a[16];
        kt = bnn & a[20];
        c0 = a[12] ^ kt;
        kt = a[20] | a[4];
        c1 = bnn ^ kt;
        kt = a[4] & a[8];
        c2 = a[20] ^ kt;
        kt = a[8] | a[12];
        c3 = a[4] ^ kt;
        kt = a[12] & a[16];
        c4 = a[8] ^ kt;
        a[12] = c0;
        a[16] = c1;
        a[20] = c2;
        a[4] = c3;
        a[8] = c4;
        a[0] = a[0] ^ RC[j + 1];
        t = a[5];
        a[5] = a[18];
        a[18] = a[11];
        a[11] = a[10];
        a[10] = a[6];
        a[6] = a[22];
        a[22] = a[20];
        a[20] = a[12];
        a[12] = a[19];
        a[19] = a[15];
        a[15] = a[24];
        a[24] = a[8];
        a[8] = t;
        t = a[1];
        a[1] = a[9];
        a[9] = a[14];
        a[14] = a[2];
        a[2] = a[13];
        a[13] = a[23];
        a[23] = a[4];
        a[4] = a[21];
        a[21] = a[16];
        a[16] = a[3];
        a[3] = a[17];
        a[17] = a[7];
        a[7] = t;

        j += 2;
    }

    /*
     * Invert some words back to normal representation.
     */
    a[1] = !a[1];
    a[2] = !a[2];
    a[8] = !a[8];
    a[12] = !a[12];
    a[17] = !a[17];
    a[20] = !a[20];
}

/*
 * Initialize a SHAKE256 context to its initial state. The state is
 * then ready to receive data (with shake256_inject()).
 */
pub fn i_shake256_init(sc: &mut InnerShake256Context) -> () {
    sc.dptr = 0;

    sc.st = [0; 25];
}

/*
 * Inject some data bytes into the SHAKE256 context ("absorb" operation).
 * This function can be called several times, to inject several chunks
 * of data of arbitrary length.
 */
pub fn i_shake256_inject(sc: &mut InnerShake256Context, input: &[u8]) -> () {
    let mut dptr: usize = sc.dptr as usize; //What point in the internal state are we currently looking at
    let mut len: usize = input.len();

    let mut offset: usize = 0;

    while len > 0 {
        let mut clen: usize; //Chunklength
        let mut u: usize;

        clen = 136 - dptr;
        if clen > len {
            clen = len
        }

        u = 0;
        while u < clen {
            let v: usize = u + dptr;
            sc.st[v >> 3] ^= (input[offset + u] as u64) << ((v & 7) << 3);
            u += 1;
        }

        dptr += clen;
        offset += clen;
        len -= clen;

        if dptr == 136 {
            process_block(&mut sc.st);
            dptr = 0;
        }
    }

    sc.dptr = dptr as u64;
}

pub fn i_shake256_inject_length(sc: &mut InnerShake256Context, input: &[u8], offset: usize, len: usize) -> () {
    let mut dptr: usize = sc.dptr as usize;

    let mut offset: usize = offset;
    let mut len: usize = len;

    while len > 0 {
        let mut clen: usize; //Chunklength
        let mut u: usize;

        clen = 136 - dptr;
        if clen > len {
            clen = len
        }

        u = 0;
        while u < clen {
            let v: usize = u + dptr;
            sc.st[v >> 3] ^= (input[offset + u] as u64) << ((v & 7) << 3);
            u += 1;
        }

        dptr += clen;
        offset += clen;
        len -= clen;

        if dptr == 136 {
            process_block(&mut sc.st);
            dptr = 0;
        }
    }

    sc.dptr = dptr as u64;
}

/*
 * Flip the SHAKE256 state to output mode. After this call, shake256_inject()
 * can no longer be called on the context, but shake256_extract() can be
 * called.
 *
 * Flipping is one-way; a given context can be converted back to input
 * mode only by initializing it again, which forgets all previously
 * injected data.
 */
pub fn i_shake256_flip(sc: &mut InnerShake256Context) -> () {
    /*
	 * We apply padding and pre-XOR the value into the state. We
	 * set dptr to the end of the buffer, so that first call to
	 * shake_extract() will process the block.
	 */
    let v: usize = sc.dptr as usize;

    sc.st[v >> 3] ^= (0x1F as u64) << ((v & 7) << 3);
    sc.st[16] ^= (0x80 as u64) << 56;

    sc.dptr = 136;
}

/*
 * Extract bytes from the SHAKE256 context ("squeeze" operation). The
 * context must have been flipped to output mode (with shake256_flip()).
 * Arbitrary amounts of data can be extracted, in one or several calls
 * to this function.
 */
pub fn i_shake256_extract(sc: &mut InnerShake256Context, out: &mut [u8]) {
    let mut dptr: usize = sc.dptr as usize;

    let mut len = out.len();
    let mut index: usize = 0;

    while len > 0 {
        let mut clen: usize;

        if dptr == 136 {
            process_block(&mut sc.st);
            dptr = 0;
        }

        clen = 136 - dptr;
        if clen > len {
            clen = len;
        }

        len -= clen;
        while clen > 0 {
            out[index] = (sc.st[dptr >> 3] >> ((dptr & 7) << 3)) as u8;
            index += 1;
            dptr += 1;
            clen -= 1;
        }
    }

    sc.dptr = dptr as u64;

    return;
}
