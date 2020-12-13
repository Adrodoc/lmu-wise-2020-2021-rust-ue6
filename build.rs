use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("gcc")
        .args(&["alloc.c", "-c", "-fPIC", "-o"])
        .arg(&format!("{}/alloc.o", out_dir))
        .status()
        .expect("GCC failed");
    Command::new("ar")
        .args(&["crus", "liballoc.a", "alloc.o"])
        .current_dir(&Path::new(&out_dir))
        .status()
        .expect("Linker ar failed");

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=alloc");
}
