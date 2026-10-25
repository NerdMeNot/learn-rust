struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {          // no self: called as Counter::new()
        Counter { count: 0 }
    }

    fn get(&self) -> u32 {         // &self: only reads
        self.count
    }

    fn bump(&mut self) {           // &mut self: changes the struct
        self.count += 1;
    }
}

fn main() {
    let mut c = Counter::new();
    c.bump();
    c.bump();
    println!("{}", c.get());
}
