// Is the list the same read forwards and backwards?
// Use a slice pattern with `middle @ ..` and check the outer two items,
// then ask the same question about the middle.
fn is_palindrome(items: &[i32]) -> bool {
    match items {
        [first, middle @ .., last] => first == last && is_palindrome(middle),
        // Zero or one item: nothing left to compare.
        _ => true,
    }
}

fn main() {
    assert!(is_palindrome(&[]));
    assert!(is_palindrome(&[7]));
    assert!(is_palindrome(&[1, 2, 1]));
    assert!(is_palindrome(&[1, 2, 2, 1]));
    assert!(!is_palindrome(&[1, 2]));
    assert!(!is_palindrome(&[1, 2, 3]));
    // The outer pair matches but the inside does not.
    assert!(!is_palindrome(&[1, 2, 3, 1]));
    assert!(!is_palindrome(&[4, 5, 6, 5, 9]));
    println!("all checks passed");
}
