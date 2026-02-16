use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let vendor = root.join("vendor");
    let tamalib = vendor.join("tamalib");

    println!("cargo:rerun-if-changed={}", tamalib.join("cpu.c").display());
    println!("cargo:rerun-if-changed={}", tamalib.join("hw.c").display());
    println!(
        "cargo:rerun-if-changed={}",
        tamalib.join("tamalib.c").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        root.join("src/sys/hal_bridge.c").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        vendor.join("hal_types.h").display()
    );

    cc::Build::new()
        .include(&vendor)
        .include(&tamalib)
        .file(tamalib.join("cpu.c"))
        .file(tamalib.join("hw.c"))
        .file(tamalib.join("tamalib.c"))
        .file(root.join("src/sys/hal_bridge.c"))
        .compile("tamalib_bridge");
}
