use std::io;
use std::os::unix::process::ExitStatusExt;
use std::process::{Child, Command, ExitStatus};
use std::thread;
use std::time::Duration;

/// Asks the child up to `polls` times, 10 ms apart, whether it has finished.
fn wait_up_to(child: &mut Child, polls: u32) -> io::Result<Option<ExitStatus>> {
    for _ in 0..polls {
        if let Some(status) = child.try_wait()? {
            return Ok(Some(status));
        }
        thread::sleep(Duration::from_millis(10));
    }
    Ok(None)
}

fn main() -> io::Result<()> {
    let mut quick = Command::new("true").spawn()?;
    let status = wait_up_to(&mut quick, 300)?;
    println!("true finished in time: {}", status.is_some());

    let mut slow = Command::new("sleep").arg("60").spawn()?;
    if wait_up_to(&mut slow, 5)?.is_none() {
        println!("sleep 60: still running when the polls ran out");
        slow.kill()?;
        let status = slow.wait()?; // collect it, so no zombie is left behind
        println!(
            "after kill -> code {:?}, signal {:?}",
            status.code(),
            status.signal()
        );
    }
    Ok(())
}
