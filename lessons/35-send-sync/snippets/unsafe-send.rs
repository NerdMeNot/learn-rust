use std::thread;

mod table {
    /// A read-only view of a number that lives for the whole program.
    pub struct Table(*const i32); // the field is private

    impl Table {
        pub fn new(value: &'static i32) -> Table {
            Table(value) // the only way to make a Table
        }

        pub fn get(&self) -> i32 {
            unsafe { *self.0 }
        }
    }

    // SAFETY: every Table holds a pointer made from a `&'static i32`: shared, read-only
    // and never freed. A `&'static i32` is already Send and Sync, so this is too.
    unsafe impl Send for Table {}
    unsafe impl Sync for Table {}
}

static ANSWER: i32 = 42;

fn main() {
    let table = table::Table::new(&ANSWER);
    thread::scope(|s| {
        let a = s.spawn(|| table.get()); // shares &Table: needs Sync
        let b = s.spawn(|| table.get());
        println!("answers: {} and {}", a.join().unwrap(), b.join().unwrap());
    });
    let moved = thread::spawn(move || table.get()); // moves Table: needs Send
    println!("moved: {}", moved.join().unwrap());
}
