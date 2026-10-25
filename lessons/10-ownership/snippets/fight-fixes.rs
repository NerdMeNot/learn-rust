#[derive(Debug)]
struct Player {
    name: String,
    score: u32,
}

// Return an index, not a reference, when the caller will change the Vec next.
fn best_index(players: &[Player]) -> Option<usize> {
    (0..players.len()).max_by_key(|&i| players[i].score)
}

// Return an owned value when you're building something new.
fn label(p: &Player) -> String {
    format!("{} ({})", p.name, p.score)
}

fn main() {
    let mut players = vec![
        Player {
            name: String::from("ana"),
            score: 30,
        },
        Player {
            name: String::from("bo"),
            score: 50,
        },
    ];

    let best = best_index(&players).unwrap(); // a plain number: no borrow is held
    players.push(Player {
        name: String::from("cy"),
        score: 10,
    });
    players[best].score += 5; // still the right player
    println!("best: {}", label(&players[best]));

    // Copy out the small thing you need, then change the original freely.
    let top_score = players[best].score;
    players.retain(|p| p.score * 2 >= top_score);
    println!("kept: {players:?}");
}
