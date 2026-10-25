use std::num::ParseIntError;

// Parse every piece, or stop at the first bad one. A for loop with ? does it.
fn parse_all(text: &str) -> Result<Vec<i32>, ParseIntError> {
    let mut numbers = Vec::new();
    for piece in text.split(',') {
        numbers.push(piece.trim().parse()?);
    }
    Ok(numbers)
}

// The same, as one expression. collect can build a Result<Vec<_>, _> from
// an iterator of Results: the Vec if all are Ok, or the first Err.
fn parse_all_collect(text: &str) -> Result<Vec<i32>, ParseIntError> {
    text.split(',').map(|piece| piece.trim().parse()).collect()
}

fn main() {
    for text in ["1, 2, 3", "1, two, 3", ""] {
        println!("loop    {text:?}: {:?}", parse_all(text));
        println!("collect {text:?}: {:?}", parse_all_collect(text));
    }
}
