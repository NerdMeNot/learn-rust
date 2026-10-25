#[derive(Clone)]
struct Team {
    name: String,
    members: Vec<String>,
}

fn main() {
    let a = Team {
        name: "red".to_string(),
        members: vec!["ana".to_string()],
    };
    let mut b = a.clone();
    b.name = "blue".to_string();
    b.members.push("bo".to_string());
    println!("{} {}", a.name, a.members.len());
    println!("{} {}", b.name, b.members.len());
}
