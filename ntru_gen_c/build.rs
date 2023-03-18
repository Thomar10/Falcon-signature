extern crate cmake;

use cmake::Config;

fn main() {
    let dst = Config::new("src/c_code").build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=ng_mp31");
    println!("cargo:rustc-link-lib=static=ng_poly");
    println!("cargo:rustc-link-lib=static=ng_zint31");
    //println!("cargo:rustc-link-lib=static=ng_falcon");
    //println!("cargo:rustc-link-lib=static=ng_ntru");
    //println!("cargo:rustc-link-lib=static=ng_fxp");
    //println!("cargo:rustc-link-lib=static=ng_gauss");
    //println!("cargo:rustc-link-lib=static=ng_prng");
    //println!("cargo:rustc-link-lib=static=sign");
    //println!("cargo:rustc-link-lib=static=vrfy");
}