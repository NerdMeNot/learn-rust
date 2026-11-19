// The same program for base64 0.22: encoding goes through an `Engine`.
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

fn main() {
    let token = STANDARD.encode("user:secret");
    println!("{token}");
}
