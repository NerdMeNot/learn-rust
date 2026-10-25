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
    // "Fixes" the borrow error by working on a copy. The copy is thrown away.
    let mut scores = team.scores.clone();
    for score in scores.iter_mut() {
        *score += team.bonus();
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
