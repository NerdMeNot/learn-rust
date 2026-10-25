struct Counter {
    count: u32,
}

impl Counter {
    fn bump(&mut self) {
        self.count += 1;
    }
}

fn main() {
    let c = Counter { count: 0 };
    c.bump();
    println!("{}", c.count);
}
