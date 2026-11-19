// Written against base64 0.22's API. Cargo.lock pins base64 0.13.
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

fn main() {
    let token = STANDARD.encode("user:secret");
    println!("{token}");
}
