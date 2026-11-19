fn count_words(text: &String) -> usize {
    text.split_whitespace().count()
}

fn total(prices: &Vec<u32>) -> u32 {
    prices.iter().sum()
}

fn main() {
    let note = String::from("buy milk and bread");
    let prices = vec![120, 250, 99];
    println!("words: {}", count_words(&note));
    println!("total: {}", total(&prices));
    // A caller with a string literal has to build a String first.
    println!("words: {}", count_words(&"two words".to_string()));
}
