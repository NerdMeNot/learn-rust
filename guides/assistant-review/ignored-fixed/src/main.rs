use std::fs;
use std::io;

fn save(report: &str) -> io::Result<()> {
    fs::write("/no/such/dir/report.txt", report)?;
    println!("report saved");
    Ok(())
}

fn main() {
    if let Err(e) = save("3 orders") {
        println!("could not save the report: {e}");
    }
}
