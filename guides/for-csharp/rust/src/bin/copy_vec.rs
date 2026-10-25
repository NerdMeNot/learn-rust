#[derive(Clone, Copy)]
struct Team {
    name: &'static str,
    members: Vec<&'static str>,
}

fn main() {
    let a = Team {
        name: "red",
        members: vec!["ana"],
    };
    let b = a;
    println!("{} {}", a.name, b.members.len());
}
