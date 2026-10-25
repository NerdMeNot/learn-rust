fn main() {
    println!("(1, 9) < (2, 0): {}", (1, 9) < (2, 0));
    println!("(2, 1) < (2, 3): {}", (2, 1) < (2, 3));
    println!("(2, 3) < (2, 3): {}", (2, 3) < (2, 3));
    println!("(1, 2) == (1, 2): {}", (1, 2) == (1, 2));
    println!("(\"b\", 1) > (\"a\", 9): {}", ("b", 1) > ("a", 9));

    // Because they compare, a list of tuples sorts: by the first slot, then the second.
    let mut results = [(2, "bo"), (1, "zed"), (2, "ana")];
    results.sort();
    println!("sorted: {results:?}");
}
