use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let current_dir = env::current_dir().unwrap();

    build_hankaku(&out_dir, &current_dir);
    build_asm(&out_dir, &current_dir);
}

fn build_hankaku(out_dir: &str, current_dir: &PathBuf) {
    let make_font = Path::new(&current_dir)
        .parent()
        .unwrap()
        .join("tools")
        .join("makefont.py");

    Command::new(make_font)
        .arg("-o")
        .arg(format!("{}/hankaku.bin", out_dir))
        .arg("hankaku.txt")
        .current_dir(&current_dir)
        .status()
        .unwrap();

    Command::new("objcopy")
        .args(&["-I", "binary", "-O", "elf64-x86-64", "-B", "i386:x86-64"])
        .arg("hankaku.bin")
        .arg("hankaku.o")
        .current_dir(&out_dir)
        .status()
        .unwrap();

    // ref: https://doc.rust-lang.org/cargo/reference/build-script-examples.html#building-a-native-library
    Command::new("ar")
        .args(&["crs", "libhankaku.a", "hankaku.o"])
        .current_dir(&out_dir)
        .status()
        .unwrap();

    // It allows Rust to include hankaku data in the elf.
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=hankaku");
    println!("cargo:rerun-if-changed=hankaku.tx");
}

fn build_asm(out_dir: &str, current_dir: &PathBuf) {

    Command::new("nasm")
        .current_dir(current_dir)
        .args(&["-f", "elf64"])
        .arg("-o")
        .arg(Path::new(out_dir).join("asmfunc.o"))
        .arg("asmfunc.asm")
        .status()
        .unwrap();

    Command::new("ar")
        .args(&["crs", "libasmfunc.a", "asmfunc.o"])
        .current_dir(out_dir)
        .status()
        .unwrap();

    println!("cargo:rustc-link-lib=static=asmfunc");
    println!("cargo:rerun-if-changed=asmfunc.asm");
}