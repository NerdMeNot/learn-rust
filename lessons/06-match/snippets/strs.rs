fn main() {
    for cmd in ["go", "run", "stop", "quit", "fly"] {
        let action = match cmd {
            "go" | "run" => "moving",
            "stop" => "halting",
            "quit" | "exit" => "leaving",
            _ => "unknown command",
        };
        println!("{cmd}: {action}");
    }

    // A String is not a &str: borrow the text first.
    let typed = String::from("stop");
    match typed.as_str() {
        "stop" => println!("typed: halting"),
        _ => println!("typed: something else"),
    }
}
