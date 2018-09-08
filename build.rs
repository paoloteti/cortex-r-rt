extern crate cc;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=link.ld");

    let target = env::var("TARGET").unwrap();

    cc::Build::new()
        .file("src/cpustack.s")
        .compile("cortex-r");

    if target.contains("eabihf") {
        println!("cargo:rustc-cfg=vfp");
    }
}
