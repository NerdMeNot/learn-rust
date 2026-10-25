#[derive(Debug)]
enum Command {
    Quit,                    // no data
    Say(String),             // one value
    Paint(u8, u8, u8),       // three values, by position
    Move { x: i32, y: i32 }, // named fields, like a struct
}

fn run(cmd: Command) {
    match cmd {
        Command::Quit => println!("quitting"),
        Command::Say(text) => println!("saying {text:?}"),
        Command::Paint(r, g, b) => println!("painting rgb({r}, {g}, {b})"),
        Command::Move { x, y } => println!("moving by ({x}, {y})"),
    }
}

fn main() {
    let cmds = vec![
        Command::Say(String::from("hi")),
        Command::Paint(255, 128, 0),
        Command::Move { x: 3, y: -4 },
        Command::Quit,
    ];
    println!("debug: {:?}", cmds[1]);
    for cmd in cmds {
        run(cmd);
    }
}
