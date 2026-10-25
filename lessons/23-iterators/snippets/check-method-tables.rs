// Not shown on the page: this runs every "Example -> result" in the method tables, so the
// extract fails if one of them stops being true. Each assert_eq! is one table cell.
fn main() {
    // Adaptors table
    let squares: Vec<i32> = [1, 2, 3].iter().map(|n| n * n).collect();
    assert_eq!(squares, [1, 4, 9]);

    let parsed: Vec<i32> = ["7", "x", "12"]
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    assert_eq!(parsed, [7, 12]);

    let letters: Vec<char> = ["hi", "yo"].iter().flat_map(|w| w.chars()).collect();
    assert_eq!(letters, ['h', 'i', 'y', 'o']);

    let flat: Vec<i32> = [Some(1), None, Some(3)].into_iter().flatten().collect();
    assert_eq!(flat, [1, 3]);

    let readings = [3, 8, 1, 9, 4, 7, 2];
    let first_three: Vec<i32> = readings.iter().copied().take(3).collect();
    assert_eq!(first_three, [3, 8, 1]);
    let after_five: Vec<i32> = readings.iter().copied().skip(5).collect();
    assert_eq!(after_five, [7, 2]);

    let before_nine: Vec<i32> = readings.iter().copied().take_while(|&n| n < 9).collect();
    assert_eq!(before_nine, [3, 8, 1]);
    let from_nine: Vec<i32> = readings.iter().copied().skip_while(|&n| n < 9).collect();
    assert_eq!(from_nine, [9, 4, 7, 2]);

    let every_third: Vec<i32> = (0..10).step_by(3).collect();
    assert_eq!(every_third, [0, 3, 6, 9]);

    let numbered: Vec<(usize, &str)> = ["ana", "bo"].into_iter().enumerate().collect();
    assert_eq!(numbered, [(0, "ana"), (1, "bo")]);

    let zipped: Vec<(&str, i32)> = ["ana", "bo"].into_iter().zip([90, 75, 82]).collect();
    assert_eq!(zipped, [("ana", 90), ("bo", 75)]);

    let chained: Vec<i32> = (1..=3).chain(7..=8).collect();
    assert_eq!(chained, [1, 2, 3, 7, 8]);

    // Searching and folding table
    let list = [3, 8, 12, 5, 20];
    assert!(list.iter().any(|&n| n > 10));
    assert!(list.iter().all(|&n| n > 0));
    assert_eq!(list.iter().find(|&&n| n > 10), Some(&12));
    assert_eq!(list.iter().position(|&n| n > 10), Some(2));

    let lines = ["host=example.org", "port=8080"];
    let port = lines.iter().find_map(|l| l.strip_prefix("port="));
    assert_eq!(port, Some("8080"));

    assert_eq!([3, 8, 1, 9].iter().nth(2), Some(&1));

    let sum_of_squares = (1..=6).fold(0, |total, n| total + n * n);
    assert_eq!(sum_of_squares, 91);
    let empty: Vec<i32> = Vec::new();
    assert_eq!(empty.into_iter().reduce(|a, b| a + b), None);

    let (even, odd): (Vec<i32>, Vec<i32>) = (1..=6).partition(|n| n % 2 == 0);
    assert_eq!(even, [2, 4, 6]);
    assert_eq!(odd, [1, 3, 5]);

    println!("every table claim holds");
}
