enum Command {
    Quit,
    Say(String),
    Move { x: i32, y: i32 },
}

fn shout(cmd: &Command) {
    // `if let` for the one variant you care about
    if let Command::Say(text) = cmd {
        println!("SAY: {}", text.to_uppercase());
    } else {
        println!("not a Say");
    }
}

fn distance(cmd: &Command) -> i32 {
    // `let ... else` keeps the happy path unindented
    let Command::Move { x, y } = cmd else {
        return 0;
    };
    x.abs() + y.abs()
}

fn main() {
    let a = Command::Say(String::from("hello"));
    let b = Command::Move { x: 3, y: -4 };
    let c = Command::Quit;
    shout(&a);
    shout(&b);
    println!("distance of a: {}", distance(&a));
    println!("distance of b: {}", distance(&b));
    // `matches!` gives a bool, without binding anything
    println!("c is Quit: {}", matches!(c, Command::Quit));
    println!("b is Move: {}", matches!(b, Command::Move { .. }));
}
