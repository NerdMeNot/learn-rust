use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=c/stats.h");
    println!("cargo:rerun-if-changed=c/stats.c");

    // Compile the C library and link it into the Rust program.
    cc::Build::new().file("c/stats.c").compile("stats");

    // Read the header with libclang and write Rust declarations for it.
    let bindings = bindgen::Builder::default()
        .header("c/stats.h")
        .allowlist_function("stats_.*")
        .allowlist_type("stats_t")
        .generate()
        .expect("bindgen could not read c/stats.h");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings.write_to_file(out).expect("could not write bindings.rs");
}
