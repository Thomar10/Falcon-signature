#[link(name = "ng_mp31", kind = "static")]
extern "C" {
    pub fn ntrugen_mp_div(x: u32, y: u32, p: u32) -> u32;
    pub fn ntrugen_mp_mkgmigm(logn: u32, gm: *const u32, igm: *const u32, g: u32, ig: u32, p: u32, p0i: u32);
    pub fn ntrugen_mp_mkgm(logn: u32, gm: *const u32, g: u32, p: u32, p0i: u32);
    pub fn ntrugen_mp_mkgm7(gm: *const u32, g: u32, p: u32, p0i: u32);
    pub fn ntrugen_mp_mkigm(logn: u32, igm: *const u32, ig: u32, p: u32, p0i: u32);
    pub fn ntrugen_mp_NTT(logn: u32, a: *const u32, gm: *const u32, p: u32, p0i: u32);
    pub fn ntrugen_mp_iNTT(logn: u32, a: *const u32, igm: *const u32, p: u32, p0i: u32);
}
