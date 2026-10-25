/// Stringly typed: any text compiles, and a typo falls through to the default arm.
fn open_flags(mode: &str) -> &'static str {
    match mode {
        "read" => "O_RDONLY",
        "write" => "O_WRONLY",
        _ => "O_RDONLY",
    }
}

fn main() {
    println!("read  -> {}", open_flags("read"));
    println!("write -> {}", open_flags("write"));
    println!("wrtie -> {}", open_flags("wrtie"));
}
