#[derive(Debug, Clone)]
struct Team {
    name: String,
    members: Vec<String>,
}

fn main() {
    let a = Team {
        name: String::from("red"),
        members: vec![String::from("ana")],
    };
    let mut b = Team {
        name: String::from("blue"),
        ..a.clone()
    };
    b.members.push(String::from("bob"));
    println!("a: {} {:?}", a.name, a.members);
    println!("b: {} {:?}", b.name, b.members);
}
