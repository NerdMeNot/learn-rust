struct Stats {
    wins: u32,
    losses: u32,
}

fn main() {
    let mut s = Stats { wins: 3, losses: 1 };

    let w = &mut s.wins;
    let l = &mut s.losses; // a different field: no overlap
    *w += 1;
    *l += 1;

    println!("wins: {}, losses: {}", s.wins, s.losses);
}
