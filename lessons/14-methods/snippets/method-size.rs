use std::mem::size_of;

struct Counter {
    count: u32,
}

impl Counter {
    fn get(&self) -> u32 {
        self.count
    }

    fn bump(&mut self) {
        self.count += 1;
    }

    fn into_value(self) -> u32 {
        self.count
    }
}

fn main() {
    let mut c = Counter { count: 0 };
    c.bump();
    println!("count: {}", c.get());
    println!("size of Counter: {} bytes", size_of::<Counter>());
    println!("size of u32:     {} bytes", size_of::<u32>());
    println!("into_value: {}", c.into_value());
}
