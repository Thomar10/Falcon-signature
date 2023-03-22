use bytemuck::cast_slice_mut;

use crate::prng::{NtruPrngChacha8Context, Rng};

pub fn gauss_sample_poly(logn: usize, f: &mut [i8], tab: &[u16], rng: Rng, ctx: NtruPrngChacha8Context) {
    let n = 1 << logn;
    let kmax: usize = tab[0] as usize;
    let mut pb = PrngBuffer {
        buf: [0; 512],
        ptr: 512,
        rng,
        ctx,
    };
    loop {
        let mut parity: u32 = 0;
        for j in 0..n {
            let mut v: u32 = (!(kmax as u32)).wrapping_add(1);
            let x: u32 = prng_buffer_next_u16(&mut pb) as u32;
            for k in 1usize..=(kmax << 1) {
                v = v.wrapping_add(((tab[k] as u32).wrapping_sub(x)  >> 31 ));
            }
            f[j] = v as i32 as i8;
            parity ^= v;
        }
        if (parity & 1) != 0 {
            return;
        }
    }
}

#[inline(always)]
pub fn prng_buffer_next_u16(pb: &mut PrngBuffer) -> u16 {
    if pb.ptr > 512 - 2 {
        (pb.rng)(&mut pb.ctx, &mut pb.buf, 512);
        pb.ptr = 0;
    }
    let mut x: u16 = pb.buf[pb.ptr] as u16;
    x |= (pb.buf[pb.ptr + 1] as u16) << 8;
    pb.ptr += 2;
    x
}


pub struct PrngBuffer {
    pub buf: [u8; 512],
    pub ptr: usize,
    pub rng: Rng,
    pub ctx: NtruPrngChacha8Context,
}