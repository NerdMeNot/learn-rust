struct Builder {
    retries: u32,
}

impl Builder {
    fn new() -> Self {
        Self { retries: 0 }
    }

    fn retries(&mut self, n: u32) -> &mut Self {
        self.retries = n;
        self
    }

    fn build(&self) -> u32 {
        self.retries
    }
}

fn main() {
    let b = Builder::new().retries(3);
    println!("built: {}", b.build());
}
