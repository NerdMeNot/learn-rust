use std::io::{self, Write};

fn main() -> io::Result<()> {
    // print! has no newline, so the text may sit in stdout's buffer: flush before reading
    print!("your name: ");
    io::stdout().flush()?;

    let mut line = String::new();
    let n = io::stdin().read_line(&mut line)?;
    println!();
    println!("read_line returned {n}, line is {line:?}");
    // read_line keeps the newline, so trim before using it
    println!("trimmed: {:?}", line.trim_end());

    // the rest of the input, line by line (Stdin has its own lines(), no BufRead import needed)
    let rest = io::stdin().lines().count();
    println!("lines left: {rest}");
    Ok(())
}
