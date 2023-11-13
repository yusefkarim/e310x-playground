// riscv32-elf-as -c -mabi=ilp32 -march=rv32imac asm.S -o asm.o
// riscv32-elf-ar crs asm.a asm.o
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    fs::copy(
        format!("asm.a"),
        out_dir.join(format!("libasm.a")),
    ).unwrap();

    println!("cargo:rustc-link-lib=static=asm");
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=link.x");
}
