fn main() {
    // peekable lets you look at the next item without taking it.
    // Here: split "aaabccdd" into runs of the same letter.
    let mut chars = "aaabccdd".chars().peekable();
    let mut runs = Vec::new();
    while let Some(c) = chars.next() {
        let mut len = 1;
        while chars.peek() == Some(&c) {
            chars.next();
            len += 1;
        }
        runs.push(format!("{c}{len}"));
    }
    println!("runs: {}", runs.join(" "));

    // next_if takes the next item only if it passes the test.
    let mut it = "123abc".chars().peekable();
    let mut digits = String::new();
    while let Some(d) = it.next_if(|c| c.is_ascii_digit()) {
        digits.push(d);
    }
    println!("digits: {digits}, left over: {}", it.collect::<String>());

    // windows and chunks are methods on slices (and so on Vec and arrays),
    // not on iterators. They hand out sub-slices without copying.
    let temps = [12, 14, 13, 17, 21];
    let rises: Vec<i32> = temps.windows(2).map(|w| w[1] - w[0]).collect();
    println!("windows(2) differences: {rises:?}");

    for chunk in temps.chunks(2) {
        println!("chunks(2): {chunk:?}"); // the last chunk can be shorter
    }
    println!(
        "chunks_exact(2) remainder: {:?}",
        temps.chunks_exact(2).remainder()
    );
}
