struct Score(u32);

impl Score {
    fn new(points: u32) -> Self {
        Self(points)
    }

    fn add(&mut self, points: u32) {
        self.0 += points;
    }

    fn is_high(&self) -> bool {
        self.0 >= 100
    }

    fn into_inner(self) -> u32 {
        self.0
    }
}

fn main() {
    let mut s = Score::new(90);
    s.add(15);
    println!("high: {}", s.is_high());
    println!("inner: {}", s.into_inner());
}
