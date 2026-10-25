use std::thread;

// A pointer to a number that lives for the whole program.
struct Table(*const i32);

impl Table {
    fn new(value: &'static i32) -> Table {
        Table(value)
    }

    fn get(&self) -> i32 {
        unsafe { *self.0 }
    }
}

static ANSWER: i32 = 42;

fn main() {
    let table = Table::new(&ANSWER);
    let handle = thread::spawn(move || table.get());
    println!("answer: {}", handle.join().unwrap());
}
