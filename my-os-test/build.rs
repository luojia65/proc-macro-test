use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Put the linker script somewhere the linker can find it
    fs::File::create(out_dir.join("link64.ld"))
        .unwrap()
        .write_all(include_bytes!("link64.ld"))
        .unwrap();
    fs::File::create(out_dir.join("link32.ld"))
        .unwrap()
        .write_all(include_bytes!("link32.ld"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=link32.ld");
    println!("cargo:rerun-if-changed=link64.ld");
}
