fn main() {
    // 1, 2, 3 (the end is not included)
    for i in 1..4 {
        println!("for {i}");
    }

    let mut n = 3;
    while n > 0 {
        n -= 1;
    }

    // loop runs until break. break can hand back a value.
    let mut tries = 0;
    let found = loop {
        tries += 1;
        if tries == 3 {
            break tries * 10;
        }
    };
    println!("n={n} found={found}");
}
