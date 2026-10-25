fn check(name: &str, answer: bool) -> bool {
    println!("  checked {name}");
    answer
}

// the right side indexes the slice, so it must only run when the slice is not empty
fn first_is_big(items: &[i32]) -> bool {
    !items.is_empty() && items[0] > 5
}

fn main() {
    println!("false && ...:");
    let a = check("left", false) && check("right", true);
    println!("  result: {a}");

    println!("true || ...:");
    let b = check("left", true) || check("right", false);
    println!("  result: {b}");

    println!("false & ... (no short-circuit):");
    let c = check("left", false) & check("right", true);
    println!("  result: {c}");

    println!("first_is_big(&[]): {}", first_is_big(&[]));
    println!("first_is_big(&[9, 1]): {}", first_is_big(&[9, 1]));
}
