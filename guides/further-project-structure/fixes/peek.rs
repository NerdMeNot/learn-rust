//! Reaches for things the library keeps to itself.

fn main() {
    let word = wordstat::count::normalize("Hello!");
    let stats = wordstat::count_words("a b").unwrap();
    println!("{word} {:?}", stats.counts());
}
