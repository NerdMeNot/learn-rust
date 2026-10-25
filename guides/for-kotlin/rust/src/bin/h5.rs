#[derive(Debug, Clone, PartialEq)]
struct User {
    name: String,
    tags: Vec<String>,
}

fn main() {
    let a = User {
        name: "ana".to_string(),
        tags: vec!["admin".to_string()],
    };
    let mut b = User {
        name: "bo".to_string(),
        ..a.clone()
    };
    b.tags.push("guest".to_string());
    println!("{a:?}");
    println!("{b:?}");
    let other = User {
        name: "ana".to_string(),
        tags: vec!["admin".to_string(), "guest".to_string()],
    };
    println!("{}", a == other);
}
