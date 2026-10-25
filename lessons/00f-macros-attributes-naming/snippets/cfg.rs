#[cfg(target_os = "linux")]
fn os() -> &'static str {
    "linux"
}

#[cfg(not(target_os = "linux"))]
fn os() -> &'static str {
    "something else"
}

fn main() {
    println!("os: {}", os());
    println!("debug build: {}", cfg!(debug_assertions));
}
