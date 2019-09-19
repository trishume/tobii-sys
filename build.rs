// extern crate cc;
// use std::env;

fn main() {
    #[cfg(target_os="linux")]
    println!("cargo:rustc-link-lib=tobii_stream_engine");

    #[cfg(target_os="macos")]
    println!("cargo:rustc-link-search=framework=/Library/Application Support/Tobii/");

    // cc::Build::new().file("dummy.c").warnings(false).flag("-Wl,-rpath,/Library/Application Support/Tobii/").compile("dummyc");
    // println!("cargo:root={}", env::var("OUT_DIR").unwrap());
}
