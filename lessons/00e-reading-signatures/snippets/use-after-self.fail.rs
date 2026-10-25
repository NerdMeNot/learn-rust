struct Counter {
    total: u32,
}

impl Counter {
    fn finish(self) -> u32 {
        self.total
    }
}

fn main() {
    let c = Counter { total: 7 };
    println!("finish: {}", c.finish());
    println!("again: {}", c.finish());
}
