use std::collections::HashMap;
use std::fs;
use std::path::Path;

// impl AsRef<Path> means "anything that can be looked at as a path"
fn size_of(path: impl AsRef<Path>) -> std::io::Result<u64> {
    Ok(fs::metadata(path)?.len())
}

fn main() -> std::io::Result<()> {
    let file = std::env::temp_dir().join("rbc-tour.txt");
    fs::write(&file, "twelve bytes")?;

    // one function, three kinds of path: &PathBuf, &String, &str
    let as_text: String = file.to_string_lossy().into_owned();
    println!(
        "{} {} {}",
        size_of(&file)?,
        size_of(&as_text)?,
        size_of(as_text.as_str())?
    );

    // trim(&self) -> &str: the result borrows from the input, nothing is copied
    let padded = String::from("  hi  ");
    let trimmed: &str = padded.trim();
    println!("[{trimmed}]");

    // HashMap<String, _>::get accepts a &str and returns Option<&V>
    let mut ages: HashMap<String, u32> = HashMap::new();
    ages.insert("ana".to_string(), 30);
    println!("{:?} {:?}", ages.get("ana"), ages.get("zed"));

    fs::remove_file(&file)?;
    Ok(())
}
