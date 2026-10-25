use std::os::unix::process::ExitStatusExt;
use std::process::Command;

fn main() -> std::io::Result<()> {
    for script in ["exit 0", "exit 5", "kill -9 $$"] {
        let status = Command::new("sh").args(["-c", script]).status()?;
        println!(
            "{script:>10} -> code {:?}, signal {:?}, raw {}, shown as \"{status}\"",
            status.code(),
            status.signal(),
            status.into_raw()
        );
    }

    // stable std can send one signal to a child: kill, which is SIGKILL on Unix
    let mut child = Command::new("sleep").arg("60").spawn()?;
    child.kill()?;
    let status = child.wait()?;
    println!(
        "after kill -> code {:?}, signal {:?}",
        status.code(),
        status.signal()
    );
    Ok(())
}
