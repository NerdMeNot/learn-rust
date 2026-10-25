use std::env;

fn main() {
    env::set_var("RBC_MODE", "fast");
    println!("mode: {:?}", env::var("RBC_MODE"));
}
