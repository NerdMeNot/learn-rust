fn sum(items: &[i32]) -> i32 {
    match items {
        [] => 0,
        [head, rest @ ..] => head + sum(rest),
    }
}

fn run(words: &[&str]) {
    match words {
        ["add", a, b] => println!("{words:?}: add {a} and {b}"),
        ["neg", a] => println!("{words:?}: negate {a}"),
        [cmd, ..] => println!("{words:?}: unknown command {cmd}"),
        [] => println!("{words:?}: nothing to do"),
    }
}

fn main() {
    println!("sum: {}", sum(&[1, 2, 3, 4]));
    run(&["add", "2", "3"]);
    run(&["neg", "7"]);
    run(&["add", "2"]);
    run(&[]);
}
