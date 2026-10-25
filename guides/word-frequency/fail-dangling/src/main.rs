mod words;

fn main() {
    let found;
    {
        let text = String::from("the cat sat");
        found = words::words(&text);
    }
    println!("{found:?}");
}
