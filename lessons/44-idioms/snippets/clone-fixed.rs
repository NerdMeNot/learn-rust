struct Team {
    scores: Vec<u32>,
    bonus_per_member: u32,
}

impl Team {
    fn bonus(&self) -> u32 {
        self.bonus_per_member + self.scores.len() as u32
    }
}

fn award(team: &mut Team) {
    // Read the one number first (a `u32` is copied out, so no borrow is left), then change the list.
    let bonus = team.bonus();
    for score in team.scores.iter_mut() {
        *score += bonus;
    }
}

fn main() {
    let mut team = Team {
        scores: vec![10, 20, 30],
        bonus_per_member: 1,
    };
    award(&mut team);
    println!("{:?}", team.scores);
}
