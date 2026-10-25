fn longest(words: &[String]) -> Option<&str> {
    let mut best: Option<&str> = None;
    for w in words {
        match best {
            Some(b) if b.len() >= w.len() => {}
            _ => best = Some(w),
        }
    }
    best
}

fn main() {
    let words = vec![
        String::from("fig"),
        String::from("banana"),
        String::from("kiwi"),
    ];
    println!("longest: {:?}", longest(&words));
    println!("empty: {:?}", longest(&[]));
    let owned: Option<String> = longest(&words).map(str::to_owned);
    println!("owned copy: {owned:?}");
}
