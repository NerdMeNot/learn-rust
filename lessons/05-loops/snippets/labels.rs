fn main() {
    // continue 'outer skips to the next n as soon as one divisor is found
    'outer: for n in 2..20 {
        for d in 2..n {
            if n % d == 0 {
                continue 'outer;
            }
        }
        println!("prime: {n}");
    }

    // break 'label value leaves a labelled loop and gives back a value
    let pair = 'search: loop {
        for a in 1..10 {
            for b in a..10 {
                if a * b == 24 && a + b == 10 {
                    break 'search (a, b);
                }
            }
        }
        break (0, 0);
    };
    println!("pair: {pair:?}");
}
