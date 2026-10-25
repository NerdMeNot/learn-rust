//! Uses the library the way another crate would: only its public API.

fn main() -> Result<(), wordstat::Error> {
    let stats = wordstat::count_words("to be or not to be")?;
    println!("{} words, as JSON: {}", stats.total(), stats.to_json());
    Ok(())
}
