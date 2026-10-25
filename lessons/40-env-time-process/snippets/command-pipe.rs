use std::io::Write;
use std::process::{self, Command, Stdio};

fn main() -> std::io::Result<()> {
    let mut child = Command::new("sort")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().expect("stdin was piped");
    stdin.write_all(b"pear\napple\nfig\n")?;

    // sort cannot start its answer until its input ends, and the pipe is still open
    println!("exited yet: {:?}", child.try_wait()?);

    drop(stdin); // closing our end of the pipe is how the child sees "end of input"
    let out = child.wait_with_output()?;
    println!("sorted: {:?}", String::from_utf8_lossy(&out.stdout));
    println!("status: {:?}", out.status.code());

    // the child is another process with its own id
    let mut other = Command::new("true").spawn()?;
    println!(
        "child id differs from ours: {}",
        other.id() != process::id()
    );
    other.wait()?; // collect the exit status, so no zombie is left behind
    Ok(())
}
