fn main() {
    // loop is the only loop that gives back a value: break carries it out
    let mut guess = 1;
    let first_over_100 = loop {
        guess *= 3;
        if guess > 100 {
            break guess;
        }
    };
    println!("first power of 3 over 100: {first_over_100}");

    // a do-while: run the body once, then decide
    let mut n = 0;
    loop {
        n += 5;
        if n >= 3 {
            break;
        }
    }
    println!("n after the body ran once: {n}");
}
