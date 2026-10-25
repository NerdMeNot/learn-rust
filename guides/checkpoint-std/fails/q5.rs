use std::env;

fn main() {
    env::set_var("MODE", "test");
    println!("{:?}", env::var("MODE"));
}
