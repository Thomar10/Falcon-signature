use alloc::vec::Vec;
use crate::shake::{i_shake256_extract, InnerShake256Context};

//TODO maybe we don't need a union and can avoid using unsafe?
//Supposedly, the dummy union is there to ensure proper alignment for 64 bit direct access
//That is probably not needed in rust and should be removed

pub union State {
    pub d: [u8; 256],
    d32: [u32; 64],
    d64: [u64; 32],
}

pub struct Prng {
    pub buf: [u8; 512],
    pub ptr: usize,
    pub state: State,
    pub typ: i32
}

pub fn prng_init(p: &mut Prng, src: &mut InnerShake256Context) -> () {
    let mut tmp: [u8; 56] = [0; 56];
    let tl: u32;
    let mut i: usize = 0;

    i_shake256_extract(src, &mut tmp);

    while i < 14 {
        //let mut w: u32;

        // w = tmp[(i << 2) + 0] as u32
        // | ((tmp[(i << 2) + 1] as u32) << 8)
        // | ((tmp[(i << 2) + 2] as u32) << 16)
        // | ((tmp[(i << 2) + 3] as u32) << 24);
        //Enforce little endian to ensure reproducibility for a given seed
        unsafe {
            p.state.d[(i << 2) + 0] = tmp[(i << 2) + 0];
            p.state.d[(i << 2) + 1] = tmp[(i << 2) + 1];
            p.state.d[(i << 2) + 2] = tmp[(i << 2) + 2];
            p.state.d[(i << 2) + 3] = tmp[(i << 2) + 3];
        }

        // Or maybe use u32::to_le_bytes

        i += 1;
    }

    unsafe {
        tl = p.state.d32[6];
        let th = p.state.d32[7];
        p.state.d32[6] = tl + ((th as u64) << 32) as u32;
    }
    prng_refill(p);
}

//TODO maybe change the union to actually hold a [u64] so we don't have to call from_ne_bytes all the time
pub fn prng_refill(p: &mut Prng) -> () {
    static CW: [u32; 4] = [
        0x61707865, 0x3320646e, 0x79622d32, 0x6b206574
    ];


    let mut cc: u64;
    unsafe {
        cc = p.state.d64[6];
    }

    for u in 0..8 {
        let mut state: [u32; 16] = [0; 16];
        //Grab 8 bytes from p.state.d and convert these to u64
        //cc = u64::from_ne_bytes(p.state.d[48 + u*8..56 + u*8].try_into().unwrap());
        // unsafe {
        //     cc = p.state.d64[6 + u];
        // }

        state[0..4].copy_from_slice(&CW);
        /* Squish bytes from p.state into state array. Done by collecting 4 bytes from p.state.d
         * at a time and converting this to a u32 and copying this into state array.
         */
        /*state[4..16].copy_from_slice(p.state.d[0..48]
            .chunks(4)
            .map(|x:[u8; 4]| u32::from_ne_bytes(*x)).collect());*/
        unsafe {
            state[4..16].copy_from_slice(&p.state.d32[0..12]);
        }
        state[14] ^= cc as u32;
        state[15] ^= (cc >> 32) as u32;

        for _ in 0..10 {
            macro_rules! qround{
                ($a:expr, $b:expr, $c:expr, $d:expr)=>{
                    state[$a] = state[$a].wrapping_add(state[$b]);
                    state[$d] ^= state[$a];
                    state[$d] = (state[$d] << 16) | (state[$d] >> 16);
                    state[$c] = state[$c].wrapping_add(state[$d]);
                    state[$b] ^= state[$c];
                    state[$b] = (state[$b] << 12) | (state[$b] >> 20);
                    state[$a] = state[$a].wrapping_add(state[$b]);
                    state[$d] ^= state[$a];
                    state[$d] = (state[$d] <<  8) | (state[$d] >> 24);
                    state[$c] = state[$c].wrapping_add(state[$d]);
                    state[$b] ^= state[$c];
                    state[$b] = (state[$b] <<  7) | (state[$b] >> 25);
                }
            }

            qround!( 0,  4,  8, 12);
            qround!( 1,  5,  9, 13);
            qround!( 2,  6, 10, 14);
            qround!( 3,  7, 11, 15);
            qround!( 0,  5, 10, 15);
            qround!( 1,  6, 11, 12);
            qround!( 2,  7,  8, 13);
            qround!( 3,  4,  9, 14);
        }

        for v in 0..4 {
           state[v] = state[v].wrapping_add(CW[v]);
        }

        for v in 4..14 {
           //state[v] += u32::from_ne_bytes(p.state.d[(v - 4)*4..v*4].try_into().unwrap());
            unsafe {
                state[v] = state[v].wrapping_add(p.state.d32[v - 4]);
            }
        }

        //state[14] += u32::from_ne_bytes(p.state.d[40..44].try_into().unwrap()) ^ (cc as u32);
        //state[15] += u32::from_ne_bytes(p.state.d[44..48].try_into().unwrap()) ^ ((cc >> 32) as u32);
        unsafe {
            state[14] = state[14].wrapping_add(p.state.d32[10] ^ (cc as u32));
            state[15] = state[15].wrapping_add(p.state.d32[11] ^ ((cc >> 32) as u32));
        }

        cc += 1;

        for v in 0..16 {
            p.buf[(u << 2) + (v << 5) + 0] = state[v] as u8;
            p.buf[(u << 2) + (v << 5) + 1] = (state[v] >> 8) as u8;
            p.buf[(u << 2) + (v << 5) + 2] = (state[v] >> 16) as u8;
            p.buf[(u << 2) + (v << 5) + 3] = (state[v] >> 24) as u8;
        }
    }

    //p.state.d[192..200] = u64::to_ne_bytes(cc).try_into().unwrap();
    unsafe {
        p.state.d64[6] = cc;
    }
    p.ptr = 0;
}

pub fn prng_get_bytes(p: &mut Prng, mut len: usize) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::with_capacity(len);

    while len > 0 {

        let mut clen: usize;

        clen = p.buf.len() - p.ptr;

        if clen > len {
            clen = len;
        }

        //output.append(&mut p.buf.d[p.ptr..p.ptr + clen].to_vec());
        //This seems kinda weird as they just keep extracting the same part of the array
        //without refill if multiple calls with a small length
        output.append(&mut p.buf[0..clen].to_vec());

        len -= clen;
        p.ptr += clen;

        if p.ptr == p.buf.len() {
            prng_refill(p);
        }
    }

    return output;
}

#[inline(always)]
pub fn prng_get_u64(p: &mut Prng) -> u64 {
    let mut u = p.ptr;
    if u >= p.buf.len()  - 9 {
        prng_refill(p);
        u = 0;
    }
    p.ptr = u + 8;
    return (p.buf[u] as u64)
        | ((p.buf[u + 1] as u64) << 8)
        | ((p.buf[u + 2] as u64) << 16)
        | ((p.buf[u + 3] as u64) << 24)
        | ((p.buf[u + 4] as u64) << 32)
        | ((p.buf[u + 5] as u64) << 40)
        | ((p.buf[u + 6] as u64) << 48)
        | ((p.buf[u + 7] as u64) << 56);
}

#[inline(always)]
pub fn prng_get_u8(p: &mut Prng) -> u8 {
    let v = p.buf[p.ptr];
    p.ptr += 1;
    if p.ptr == p.buf.len() {
        prng_refill(p);
    }
    v
}