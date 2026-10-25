fn main() {
    // Generates the C++ side of the bridge from src/main.rs, then compiles it
    // together with our own C++ file.
    cxx_build::bridge("src/main.rs")
        .file("src/counter.cc")
        .std("c++20")
        .compile("counter");
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/counter.cc");
    println!("cargo:rerun-if-changed=src/counter.h");
}
