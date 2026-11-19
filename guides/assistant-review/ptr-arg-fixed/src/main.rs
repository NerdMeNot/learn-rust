fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn total(prices: &[u32]) -> u32 {
    prices.iter().sum()
}

fn main() {
    let note = String::from("buy milk and bread");
    let prices = vec![120, 250, 99];
    println!("words: {}", count_words(&note));
    println!("total: {}", total(&prices));
    // A literal, an array and part of a Vec now work as they are.
    println!("words: {}", count_words("two words"));
    println!("total: {}", total(&[5, 5]));
    println!("total: {}", total(&prices[1..]));
}
