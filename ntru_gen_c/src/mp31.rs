#[link(name = "ng_mp31", kind = "static")]
extern "C" {
    pub fn ntrugen_mp_div(x: u32, y: u32, p: u32) -> u32;
    /*   pub fn mp_mkgmigm(logn: u32, gm: *const u32, igm: *const u32, g: u32, ig: u32, p: u32, p0i: u32);
       pub fn mp_mkgm(logn: u32, gm: *const u32, g: u32, p: u32, p0i: u32);
       pub fn mp_mkgm7(gm: *const u32, g: u32, p: u32, p0i: u32);
       pub fn mp_mkigm(logn: u32, igm: *const u32, ig: u32, p: u32, p0i: u32);
       pub fn mp_ntt(logn: u32, a: *const u32, gm: *const u32, p: u32, p0i: u32);
       pub fn mp_intt(logn: u32, a: *const u32, igm: *const u32, p: u32, p0i: u32);*/
}
