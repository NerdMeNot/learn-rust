#[derive(Clone, Copy)]
struct Player {
    name: String,
    score: u32,
}

fn main() {
    let p = Player {
        name: String::from("ana"),
        score: 1,
    };
    println!("{}", p.score);
}
