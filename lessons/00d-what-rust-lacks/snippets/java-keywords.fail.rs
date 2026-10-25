struct Counter {
    n: u32,
}

impl Counter {
    public void bump(&mut self) {
        self.n += 1;
    }
}

impl Counter {
    static count: u32 = 0;
}

fn main() {}
