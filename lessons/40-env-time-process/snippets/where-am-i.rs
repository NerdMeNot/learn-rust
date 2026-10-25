use std::env;
use std::process::{self, Command};

fn main() -> std::io::Result<()> {
    println!("OS {}, architecture {}", env::consts::OS, env::consts::ARCH);
    println!("process id is positive: {}", process::id() > 0);
    println!(
        "current_dir is absolute: {}",
        env::current_dir()?.is_absolute()
    );
    println!(
        "current_exe is absolute: {}",
        env::current_exe()?.is_absolute()
    );

    // the child's directory is its own setting; ours does not change
    let before = env::current_dir()?;
    let out = Command::new("pwd").current_dir("/").output()?;
    println!(
        "pwd in the child: {:?}",
        String::from_utf8_lossy(&out.stdout)
    );
    println!("ours unchanged: {}", env::current_dir()? == before);

    // set_current_dir moves the whole process, every thread, and every relative path with it
    env::set_current_dir("/")?;
    println!("after set_current_dir: {:?}", env::current_dir()?);
    Ok(())
}
