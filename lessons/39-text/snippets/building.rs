use std::fmt::{self, Write};

// A Display impl may fail, and write! passes that on.
struct Broken;
impl fmt::Display for Broken {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Err(fmt::Error)
    }
}

fn main() {
    // repeat, and collecting chars or pieces into a String.
    println!("repeat:  {}", "=-".repeat(4));
    let shout: String = "quiet".chars().map(|c| c.to_ascii_uppercase()).collect();
    println!("collect chars:  {shout}");
    let glued: String = ["ab", "cd", "ef"].into_iter().collect();
    println!("collect pieces: {glued}");

    // join works on a Vec<String> as well as on &str items.
    let owned: Vec<String> = vec!["x".to_string(), "y".to_string()];
    println!("join:    {}", owned.join(" + "));

    // extend appends many pieces to one String.
    let mut line = String::from("ids:");
    line.extend([" 1", " 2", " 3"]);
    println!("extend:  {line}");

    // write! formats into an existing String, with no temporary String.
    let mut report = String::new();
    for (name, qty) in [("tea", 2), ("jam", 10)] {
        writeln!(report, "{name:<5}{qty:>3}").unwrap();
    }
    print!("{report}");

    // Writing into a String is not what fails; a Display impl can.
    println!("write! of a number: {:?}", write!(report, "{}", 7));
    println!("write! of Broken:   {:?}", write!(report, "{}", Broken));
}
