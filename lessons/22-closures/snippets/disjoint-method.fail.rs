struct Player {
    name: String,
    score: u32,
}

impl Player {
    fn label(&self) -> String {
        format!("player {}", self.name)
    }
}

fn main() {
    let mut p = Player {
        name: String::from("Ana"),
        score: 0,
    };

    let show = || println!("{}", p.label());
    p.score += 5;
    show();
}
