struct Stats {
    wins: u32,
    losses: u32,
}

fn reset(s: &mut Stats) {
    s.wins = 0;
    s.losses = 0;
}

fn main() {
    let mut s = Stats { wins: 3, losses: 1 };

    let w = &mut s.wins;
    reset(&mut s); // `reset` could touch any field, including `wins`
    *w += 1;

    println!("wins: {}, losses: {}", s.wins, s.losses);
}
