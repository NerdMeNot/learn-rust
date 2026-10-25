mod percent {
    pub struct Percent(u8); // the field has no `pub`

    impl Percent {
        pub fn new(n: u8) -> Option<Percent> {
            if n <= 100 { Some(Percent(n)) } else { None }
        }

        pub fn get(&self) -> u8 {
            self.0
        }
    }
}

fn main() {
    let ok = percent::Percent::new(50).unwrap();
    let bad = percent::Percent(250);
    println!("{} {}", ok.get(), bad.get());
}
