struct Player {
    name: String,
    scores: Vec<u32>,
}

/// The highest score, or 0 when there are none. Only reads the player.
fn best(player: &Player) -> u32 {
    todo!("loop over &player.scores and keep the largest")
}

/// A line like "ana: best 95 in 3 games". Builds a new String and only reads the player.
fn report(player: &Player) -> String {
    todo!("call best(player), then format! the name, the best score and scores.len()")
}

fn main() {
    let ana = Player {
        name: String::from("ana"),
        scores: vec![30, 95, 60],
    };

    // Reading the same player twice only works if report borrows it.
    let first = report(&ana);
    let second = report(&ana);
    assert_eq!(first, "ana: best 95 in 3 games");
    assert_eq!(first, second);

    // And we can still use the player.
    assert_eq!(ana.name, "ana");
    assert_eq!(best(&ana), 95);

    // A player with no scores.
    let new = Player {
        name: String::from("zoë"),
        scores: Vec::new(),
    };
    assert_eq!(best(&new), 0);
    assert_eq!(report(&new), "zoë: best 0 in 0 games");

    println!("all checks passed");
}
