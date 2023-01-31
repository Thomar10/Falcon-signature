mod main_test;
mod falcon_c {
    pub mod codec_c;
    pub mod nist_c;
}



fn main() {
    let res = 4;
    println!("{} tissemand", res);
    println!("Hello, world falcon!");
}

pub fn addd(a: i32, b: i32) -> i32  {
    a + b
}