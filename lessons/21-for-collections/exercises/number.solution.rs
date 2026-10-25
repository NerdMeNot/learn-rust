// Put a number in front of every line, starting at 1, changing the list in place:
// ["a", "b"] becomes ["1: a", "2: b"].
fn number(lines: &mut Vec<String>) {
    for (i, line) in lines.iter_mut().enumerate() {
        *line = format!("{}: {line}", i + 1);
    }
}

fn main() {
    let mut lines = vec![String::from("a"), String::from("b"), String::from("c")];
    number(&mut lines);
    assert_eq!(lines, ["1: a", "2: b", "3: c"]);

    let mut empty: Vec<String> = Vec::new();
    number(&mut empty);
    assert!(empty.is_empty());

    let mut blank = vec![String::new(), String::from("x")];
    number(&mut blank);
    assert_eq!(blank, ["1: ", "2: x"]);

    let mut many: Vec<String> = (0..12).map(|_| String::from("z")).collect();
    number(&mut many);
    assert_eq!(many[9], "10: z");
    assert_eq!(many[11], "12: z");
    println!("all checks passed");
}
