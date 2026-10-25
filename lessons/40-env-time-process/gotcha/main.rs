use std::env;

fn main() {
    let port = env::var("RBC_PORT");
    println!("listening on port {}", port);
}
