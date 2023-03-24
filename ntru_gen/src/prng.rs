use bytemuck::cast_slice_mut;

pub type Rng = fn(&mut NtruPrngChacha8Context, &mut [u8], usize);

pub fn prng_chacha8_init(ctx: &mut NtruPrngChacha8Context, seed: &[u8], mut seed_len: usize) {
    if seed_len > 32 {
        seed_len = 32;
    }

    let mut tmp: [u8; 32] = [0; 32];
    tmp[..seed_len].copy_from_slice(seed);
    let key = cast_slice_mut::<u8, u32>(&mut ctx.d);
    for u in 0..8 {
        key[u] = dec32le(tmp.split_at(u << 2).1)
    }
}

pub fn prng_chacha8_out(ctx: &mut NtruPrngChacha8Context, mut buf: &mut [u8], mut len: usize) {
    const CW: [u32; 4] = [0xA7C083FE, 0x3320646E, 0x79622d32, 0x6B206574];
    let mut cc: u64 = cast_slice_mut::<u8, u64>(&mut ctx.d)[4];
    let ctx_u32 = cast_slice_mut::<u8, u32>(&mut ctx.d);
    let out512 = len == 512;

    while len > 0 {
        let mut state: [u32; 16] = [0; 16];
        state[..4].copy_from_slice(&CW);
        state[4..12].copy_from_slice(&ctx_u32[..8]);
        state[12] = cc as u32;
        state[13] = (cc >> 32) as u32;
        state[14] = 0;
        state[15] = 0;
        for _ in 0..4 {
            macro_rules! qround {
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
        for v in 4..12 {
            state[v] = state[v].wrapping_add(ctx_u32[v - 4]);
        }
        state[12] = state[12].wrapping_add(cc as u32);
        state[13] = state[13].wrapping_add((cc >> 32) as u32);
        cc += 1;

        if out512 {
            for v in 0..16 {
                enc32le(buf.split_at_mut(v << 5).1, state[v]);
            }
            buf = buf.split_at_mut(4).1;
            len -= 64;
        } else {
            if len >= 64 {
                for v in 0..16 {
                    enc32le(buf.split_at_mut(v << 2).1, state[v]);
                }
                buf = buf.split_at_mut(64).1;
                len -= 64;
            } else {
                let mut v = 0;
                while len >= 4 {
                    enc32le(buf, state[v]);
                    buf = buf.split_at_mut(4).1;
                    len -= 4;
                    v += 1;
                }
                let mut x = state[v];
                while len > 0 {
                    buf[0] = x as u8;
                    x >>= 8;
                    len -= 1;
                    buf = buf.split_at_mut(1).1;
                }
                break;
            }
        }
    }

    cast_slice_mut::<u8, u64>(&mut ctx.d)[4] = cc;
}

#[inline(always)]
pub fn enc32le(dst: &mut [u8], x: u32) {
    dst[0] = x as u8;
    dst[1] = (x >> 8) as u8;
    dst[2] = (x >> 16) as u8;
    dst[3] = (x >> 24) as u8;
}

#[inline(always)]
pub fn dec32le(src: &[u8]) -> u32 {
    return (src[0] as u32)
        | ((src[1] as u32) << 8)
        | ((src[2] as u32) << 16)
        | ((src[3] as u32) << 24);
}

pub struct NtruPrngChacha8Context {
    pub d: [u8; 40],
}