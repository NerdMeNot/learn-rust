use std::env;

fn main() {
    let port = env::var("RBC_PORT").unwrap_or_else(|_| "8080".to_string());
    println!("listening on port {port}");
}
