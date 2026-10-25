fn main() {
    let line = "  name=Ana, age=30, city=Lisbon  ";

    let clean = line.trim(); // a &str view into line: nothing copied
    println!("[{clean}]");

    // split on ',', trim each piece, split once on '='
    for field in clean.split(',') {
        if let Some((key, value)) = field.trim().split_once('=') {
            println!("{key:>5} -> {value}");
        }
    }

    // text to number: parse returns Result
    let age: u32 = "30".parse().unwrap_or(0);
    println!("next year: {}", age + 1);

    // these build new Strings
    let shout = clean.to_uppercase().replace(", ", " | ");
    println!("{shout}");

    println!("starts with name: {}", clean.starts_with("name"));
    println!("position of age: {:?}", clean.find("age"));

    let words = ["build", "with", "rust"];
    println!("{}", words.join("-"));

    // grow a String, like strings.Builder
    let mut out = String::new();
    for i in 1..=3 {
        out.push_str(&i.to_string());
        out.push(';');
    }
    println!("{out}");
}
