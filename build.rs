use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("gcc")
        .args(&["clib.c", "-c", "-fPIC", "-o"])
        .arg(&format!("{}/clib.o", out_dir))
        .status()
        .expect("GCC failed");
    Command::new("ar")
        .args(&["crus", "libclib.a", "clib.o"])
        .current_dir(&Path::new(&out_dir))
        .status()
        .expect("Linker ar failed");

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=clib");
}
