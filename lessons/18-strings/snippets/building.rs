fn main() {
    // Two ways to make an owned String from text. They do the same thing.
    let a = String::from("one");
    let b = "two".to_string();
    println!("{a} {b}");

    // Grow one in place.
    let mut s = String::new();
    s.push_str("Hello");
    s.push(','); // push takes a single char
    s.push_str(" world");
    println!("{s}");

    // format! builds a new String from any mix of values, like println! without printing.
    let user = "ana";
    let visits = 3;
    let line = format!("{user} has visited {visits} times");
    println!("{line}");

    // Joining a list.
    let words = ["red", "green", "blue"];
    println!("join: {}", words.join(", "));
    println!("concat: {}", words.concat());

    // Building in a loop: reserve the space once if you know roughly how much.
    let mut csv = String::with_capacity(64);
    for n in 1..=5 {
        if !csv.is_empty() {
            csv.push(',');
        }
        csv.push_str(&n.to_string());
    }
    println!("{csv}");
}
