struct Guard(u32);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("dropped {}", self.0);
    }
}

fn main() {
    for i in 1..=3 {
        let _ = Guard(i);
        println!("loop {i}");
    }
    println!("end of main");
}
