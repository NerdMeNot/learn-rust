fn main() {
    // a..b stops before b; a..=b includes b
    for i in 1..4 {
        println!("1..4: {i}");
    }
    for i in 1..=4 {
        println!("1..=4: {i}");
    }

    // ranges of char work too
    for c in 'a'..='c' {
        println!("'a'..='c': {c}");
    }

    // counting down: reverse a range
    for i in (1..=3).rev() {
        println!("rev: {i}");
    }

    // every third number
    for i in (0..10).step_by(3) {
        println!("step_by(3): {i}");
    }

    // a range that starts above its end is empty: no error, no countdown
    let mut runs = 0;
    for _ in 4..0 {
        runs += 1;
    }
    println!("runs of 4..0: {runs}");

    // the end is read once, when the loop starts
    let mut n = 3;
    let mut passes = 0;
    for _ in 0..n {
        n = 5;
        passes += 1;
    }
    println!("n is now {n}, passes: {passes}");
}
