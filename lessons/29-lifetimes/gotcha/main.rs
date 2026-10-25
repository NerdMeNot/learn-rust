fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let s1 = String::from("a long first string");
    let result;
    {
        let s2 = String::from("short");
        result = longest(&s1, &s2);
    }
    println!("longest: {result}");
}
