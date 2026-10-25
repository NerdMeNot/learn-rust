struct Player {
    name: String,
    score: u32,
}

fn main() {
    let mut p = Player {
        name: String::from("Ana"),
        score: 0,
    };

    // The closure uses only `p.name`, so it borrows only that field.
    let show = || println!("player {}", p.name);
    p.score += 5; // a different field: allowed while `show` is alive
    show();
    println!("score: {}", p.score);
}
