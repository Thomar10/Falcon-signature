#[cfg(test)]
mod tests {
    use rand::Rng;
    use crate::codec::{comp_decode, comp_encode, max_fg_bits, max_FG_bits, modq_decode, modq_encode, trim_i16_decode, trim_i16_encode, trim_i8_decode, trim_i8_encode};
    use crate::falcon_c::codec_c::{falcon_inner_comp_decode, falcon_inner_comp_encode, falcon_inner_modq_decode, falcon_inner_modq_encode, falcon_inner_trim_i16_decode, falcon_inner_trim_i16_encode, falcon_inner_trim_i8_decode, falcon_inner_trim_i8_encode};

    #[test]
    fn test_trim_i8_encode() {
        let mut sk: [u8; 1281] = [0; 1281];
        let max_out = sk.len();
        let mut sk_c: [u8; 1281] = [0; 1281];
        let mut rng = rand::thread_rng();
        // Values must not be above / below (1 << bits) - 1. Below ensures this.
        let f: [i8; 512] = core::array::from_fn(|_| rng.gen::<i8>());
        let mut vec = f.iter().map(|x| x & ((1 << 4) - 1)).collect::<Vec<_>>();
        let mut f_input = vec.as_mut_slice();

        let g: [i8; 512] = core::array::from_fn(|_| rng.gen::<i8>());
        let mut vec = g.iter().map(|x| x & ((1 << 4) - 1)).collect::<Vec<_>>();
        let mut g_input = vec.as_mut_slice();

        let ff: [i8; 512] = core::array::from_fn(|_| rng.gen::<i8>());
        let mut vec = ff.iter().map(|x| x & ((1 << 4) - 1)).collect::<Vec<_>>();
        let mut ff_input = vec.as_mut_slice();
        sk[0] = 0x50 + 9;
        sk_c[0] = 0x50 + 9;
        let mut u_r = 1;
        let mut u_c = 1;
        let mut size_r = trim_i8_encode(&mut sk, u_r, max_out - u_r, &mut f_input, 9, max_fg_bits[9] as u32);
        let mut size_c = unsafe {
            let x: *mut u8 = sk_c.as_mut_ptr().add(u_c);
            falcon_inner_trim_i8_encode(x.cast(), max_out - u_c, f_input.as_ptr(), 9, max_fg_bits[9] as u32)
        };
        u_r += size_r;
        u_c += size_c;
        assert_eq!(size_c, size_r);
        assert_eq!(sk, sk_c);
        size_r = trim_i8_encode(&mut sk, u_r, max_out - u_r, &mut g_input, 9, max_fg_bits[9] as u32);
        size_c = unsafe {
            let x: *mut u8 = sk_c.as_mut_ptr().add(u_c);
            falcon_inner_trim_i8_encode(x.cast(), max_out - u_c, g_input.as_ptr(), 9, max_fg_bits[9] as u32)
        };
        u_r += size_r;
        u_c += size_c;
        assert_eq!(size_c, size_r);
        assert_eq!(sk, sk_c);
        size_r = trim_i8_encode(&mut sk, u_r, max_out - u_r, &mut ff_input, 9, max_FG_bits[9] as u32);
        size_c = unsafe {
            let x: *mut u8 = sk_c.as_mut_ptr().add(u_c);
            falcon_inner_trim_i8_encode(x.cast(), max_out - u_c, ff_input.as_ptr(), 9, max_FG_bits[9] as u32)
        };
        u_r += size_r;
        u_c += size_c;
        assert_eq!(sk, sk_c);
        assert_eq!(size_c, size_r);
        assert_eq!(u_c, u_r);
        assert_eq!(1281, u_r);
    }

    #[test]
    fn test_trim_i16_encode() {
        let mut sk: [u8; 1281] = [0; 1281];
        let max_out = sk.len();
        let mut sk_c: [u8; 1281] = [0; 1281];
        let mut rng = rand::thread_rng();
        // Values must not be above / below (1 << bits) - 1. Below ensures this.
        let f: [i16; 512] = core::array::from_fn(|_| rng.gen::<i16>());
        let mut vec = f.iter().map(|x| x & ((1 << 4) - 1)).collect::<Vec<_>>();
        let mut f_input = vec.as_mut_slice();
        sk[0] = 0x50 + 9;
        sk_c[0] = 0x50 + 9;
        let u_r = 1;
        let u_c = 1;
        let size_r = trim_i16_encode(&mut sk, u_r, max_out - u_r, &mut f_input, 9, max_fg_bits[9] as u32);
        let size_c = unsafe {
            let x: *mut u8 = sk_c.as_mut_ptr().add(u_c);
            falcon_inner_trim_i16_encode(x.cast(), max_out - u_c, f_input.as_ptr(), 9, max_fg_bits[9] as u32)
        };
        assert_eq!(size_c, size_r);
        assert_eq!(sk, sk_c);
    }

    #[test]
    fn test_trim_i16_decode() {
        let mut sk: [u8; 1281] = [0; 1281];
        let max_out = sk.len();
        let mut sk_c: [u8; 1281] = [0; 1281];
        let mut rng = rand::thread_rng();
        let f: [i16; 512] = core::array::from_fn(|_| rng.gen::<i16>());
        let mut vec = f.iter().map(|x| x & ((1 << 4) - 1)).collect::<Vec<_>>();
        let mut f_input = vec.as_mut_slice();
        sk[0] = 0x50 + 9;
        sk_c[0] = 0x50 + 9;
        let u_r = 1;
        let u_c = 1;
        trim_i16_encode(&mut sk, u_r, max_out - u_r, &mut f_input, 9, max_fg_bits[9] as u32);
        unsafe {
            let x: *mut u8 = sk_c.as_mut_ptr().add(u_c);
            falcon_inner_trim_i16_encode(x.cast(), max_out - u_c, f_input.as_ptr(), 9, max_fg_bits[9] as u32)
        };
        let mut x: [i16; 512] = [0; 512];
        let x_c: [i16; 512] = [0; 512];
        let in_sizer = trim_i16_decode(&mut x, 9, max_fg_bits[9] as u32, &mut sk, 0, 512 - 1);
        let in_sizec = unsafe { falcon_inner_trim_i16_decode(x_c.as_ptr(), 9, max_fg_bits[9] as u32, sk_c.as_ptr().cast(), 512 - 1) };
        assert_eq!(x, x_c);
        assert_eq!(in_sizer, in_sizec);
    }

    #[test]
    fn test_trim_i8_decode() {
        let mut sk: [u8; 1281] = [0; 1281];
        let max_out = sk.len();
        let mut sk_c: [u8; 1281] = [0; 1281];
        let mut rng = rand::thread_rng();
        let f: [i8; 512] = core::array::from_fn(|_| rng.gen::<i8>());
        let mut vec = f.iter().map(|x| x & ((1 << 4) - 1)).collect::<Vec<_>>();
        let mut f_input = vec.as_mut_slice();
        sk[0] = 0x50 + 9;
        sk_c[0] = 0x50 + 9;
        let u_r = 1;
        let u_c = 1;
        trim_i8_encode(&mut sk, u_r, max_out - u_r, &mut f_input, 9, max_fg_bits[9] as u32);
        unsafe {
            let x: *mut u8 = sk_c.as_mut_ptr().add(u_c);
            falcon_inner_trim_i8_encode(x.cast(), max_out - u_c, f_input.as_ptr(), 9, max_fg_bits[9] as u32)
        };
        let mut x: [i8; 512] = [0; 512];
        let x_c: [i8; 512] = [0; 512];
        let in_sizer = trim_i8_decode(&mut x, 9, max_fg_bits[9] as u32, &mut sk, 0, 512 - 1);
        let in_sizec = unsafe { falcon_inner_trim_i8_decode(x_c.as_ptr(), 9, max_fg_bits[9] as u32, sk_c.as_ptr().cast(), 512 - 1) };
        assert_eq!(x, x_c);
        assert_eq!(in_sizer, in_sizec);
    }

    #[test]
    fn test_modq_encode() {
        let mut pk: [u8; 1281] = [0; 1281];
        let mut pk_c: [u8; 1281] = [0; 1281];
        let mut rng = rand::thread_rng();

        // Using u8 because values must be below 12289, then cast it to u16
        let h: [u8; 512] = core::array::from_fn(|_| rng.gen::<u8>());
        let mut binding = h.iter().map(|x| *x as u16).collect::<Vec<u16>>();
        let mut h_input = binding.as_mut_slice();
        pk[0] = 0x00 + 9;
        pk_c[0] = 0x00 + 9;
        let v_c = unsafe {
            let x: *const u8 = pk_c.as_ptr().add(1);
            falcon_inner_modq_encode(x.cast(), 897 - 1, h_input.as_ptr(), 9)
        };
        let v_r = modq_encode(&mut pk, 1, 897 - 1, &mut h_input, 9);
        assert_eq!(pk, pk_c);
        assert_eq!(v_c, v_r);
        assert_eq!(v_r, 896);
    }

    #[test]
    fn test_modq_decode() {
        let mut pk: [u8; 897] = [0; 897];
        let mut pk_c: [u8; 897] = [0; 897];
        let mut rng = rand::thread_rng();

        // Using u8 because values must be below 12289, then cast it to u16
        let h: [u8; 512] = core::array::from_fn(|_| rng.gen::<u8>());
        let mut binding = h.iter().map(|x| *x as u16).collect::<Vec<u16>>();
        let mut h_input = binding.as_mut_slice();
        pk[0] = 0x00 + 9;
        pk_c[0] = 0x00 + 9;
        let v_c = unsafe {
            let x: *const u8 = pk_c.as_ptr().add(1);
            falcon_inner_modq_encode(x.cast(), 897 - 1, h_input.as_ptr(), 9)
        };
        let v_r = modq_encode(&mut pk, 1, 897 - 1, &mut h_input, 9);
        assert_eq!(pk, pk_c);
        assert_eq!(v_c, v_r);
        assert_eq!(v_r, 896);

        let mut h_ret: [u16; 512] = [0; 512];
        let h_ret_c: [u16; 512] = [0; 512];
        let dr = modq_decode(&mut h_ret, 9, &mut pk, 1, 897 - 1);
        let dc = unsafe {
            let inn = pk_c.as_ptr().add(1);
            falcon_inner_modq_decode(h_ret_c.as_ptr(), 9, inn.cast(), 897 - 1)
        };
        assert_eq!(h_ret, h_ret_c);
        assert_eq!(dr, dc);
        assert_eq!(dr, 896);
    }

    #[test]
    fn test_comp_encode() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let f: [i16; 512] = core::array::from_fn(|_| rng.gen::<i16>());
                let mut vec = f.iter().map(|x| {
                    if *x < -2047 || *x > 2047 { 2 } else { *x }
                }).collect::<Vec<i16>>();
                let f_input = vec.as_mut_slice();
                let mut out: [u8; 2000] = [0; 2000];
                let out_c: [u8; 2000] = [0; 2000];
                let resr_size = comp_encode(&mut out, 0, 2000, f_input, logn);
                let resc_size = unsafe { falcon_inner_comp_encode(out_c.as_ptr().cast(), 2000, f_input.as_ptr(), logn as u32) };
                assert_eq!(resr_size, resc_size);
                assert_eq!(out, out_c);
            }
        }
    }

    #[test]
    fn test_comp_decode() {
        for _ in 0..100 {
            for logn in 1..10 {
                let mut rng = rand::thread_rng();
                let mut inn: [u8; 2000] = core::array::from_fn(|_| rng.gen::<u8>());
                let mut f: [i16; 512] = [0; 512];
                let f_c: [i16; 512] = [0; 512];

                let res_size = comp_decode(&mut f, logn, &mut inn, 0, 512 - 1);
                let resc_size = unsafe { falcon_inner_comp_decode(f_c.as_ptr(), logn, inn.as_ptr().cast(), 512 - 1) };
                println!("{}", res_size);
                println!("{:?}", f);
                println!("logn {}", logn);
                assert_eq!(f, f_c);
                assert_eq!(res_size, resc_size);
            }
        }
    }
}