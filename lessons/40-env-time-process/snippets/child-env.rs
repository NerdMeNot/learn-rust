use std::env;
use std::process::Command;

fn main() {
    if env::args().nth(1).as_deref() == Some("--child") {
        println!("RBC_A: {:?}", env::var("RBC_A"));
        println!("RBC_GONE: {:?}", env::var("RBC_GONE"));
        println!("PATH set: {}", env::var_os("PATH").is_some());
        let inherited = env::vars().any(|(name, _)| !name.starts_with("RBC_"));
        println!("other variables, inherited: {inherited}");
        return;
    }

    let me = env::current_exe().expect("the path of this program");

    // the child inherits our environment, plus what we add or remove for it alone
    let out = Command::new(&me)
        .arg("--child")
        .env("RBC_A", "1")
        .env_remove("PATH")
        .output()
        .expect("the child should start");
    println!("-- env, env_remove:");
    print!("{}", String::from_utf8_lossy(&out.stdout));

    // env_clear starts the child with nothing but what you add next
    let out = Command::new(&me)
        .arg("--child")
        .env_clear()
        .env("RBC_A", "2")
        .output()
        .expect("the child should start");
    println!("-- env_clear:");
    print!("{}", String::from_utf8_lossy(&out.stdout));

    // our own environment did not change
    println!("parent RBC_A: {:?}", env::var("RBC_A"));
}
