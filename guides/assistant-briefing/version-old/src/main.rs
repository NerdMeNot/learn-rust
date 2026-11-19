// Written against base64 0.13's API. Cargo.lock pins base64 0.22.
fn main() {
    let token = base64::encode("user:secret");
    println!("{token}");
}
