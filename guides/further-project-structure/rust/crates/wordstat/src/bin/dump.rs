//! Prints the counts as JSON. Built only with `--features json`.

fn main() {
    let text = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    match wordstat::count_words(&text) {
        Ok(stats) => println!("{}", stats.to_json()),
        Err(e) => {
            eprintln!("dump: {e}");
            std::process::exit(1);
        }
    }
}
