// extern crate cc;
// use std::env;

fn main() {
    println!("cargo:rustc-link-search=framework=/Library/Application Support/Tobii/");
    // cc::Build::new().file("dummy.c").warnings(false).flag("-Wl,-rpath,/Library/Application Support/Tobii/").compile("dummyc");
    // println!("cargo:root={}", env::var("OUT_DIR").unwrap());
}
