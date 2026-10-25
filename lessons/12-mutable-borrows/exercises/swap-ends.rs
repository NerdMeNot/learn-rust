/// Swap the first and the last element. A list with fewer than two
/// elements is left as it is.
fn swap_ends(words: &mut Vec<String>) {
    todo!("words.swap(i, j) swaps two elements; check the length first so an empty list is safe")
}

fn main() {
    let mut words = vec![
        String::from("first"),
        String::from("second"),
        String::from("third"),
        String::from("last"),
    ];
    let first_text = words[0].as_ptr();
    let last_text = words[3].as_ptr();

    swap_ends(&mut words);
    assert_eq!(words, ["last", "second", "third", "first"]);
    assert_eq!(
        words[0].as_ptr(),
        last_text,
        "swap the Strings, do not copy them"
    );
    assert_eq!(
        words[3].as_ptr(),
        first_text,
        "swap the Strings, do not copy them"
    );

    let mut odd = vec![String::from("a"), String::from("b"), String::from("c")];
    swap_ends(&mut odd);
    assert_eq!(odd, ["c", "b", "a"]);

    let mut two = vec![String::from("x"), String::from("y")];
    swap_ends(&mut two);
    assert_eq!(two, ["y", "x"]);

    let mut one = vec![String::from("only")];
    swap_ends(&mut one);
    assert_eq!(one, ["only"]);

    let mut none: Vec<String> = Vec::new();
    swap_ends(&mut none);
    assert!(none.is_empty());

    println!("all checks passed");
}
