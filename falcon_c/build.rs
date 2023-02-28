extern crate cmake;

use cmake::Config;

fn main() {
    let dst = Config::new("src/c_code").build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=common");
    println!("cargo:rustc-link-lib=static=codec");
    println!("cargo:rustc-link-lib=static=fft");
    println!("cargo:rustc-link-lib=static=fpr");
    println!("cargo:rustc-link-lib=static=keygen");
    println!("cargo:rustc-link-lib=static=nist");
    println!("cargo:rustc-link-lib=static=rng");
    println!("cargo:rustc-link-lib=static=shake");
    println!("cargo:rustc-link-lib=static=sign");
    println!("cargo:rustc-link-lib=static=vrfy");
}