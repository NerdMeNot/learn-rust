use std::env;

fn main() {
    // SAFETY: no other thread exists yet, so nothing else can read or write the environment.
    unsafe {
        env::set_var("RBC_MODE", "fast");
    }
    println!("after set_var: {:?}", env::var("RBC_MODE"));

    unsafe {
        env::remove_var("RBC_MODE");
    }
    println!("after remove_var: {:?}", env::var("RBC_MODE"));
}
