mod port {
    pub struct Port(u16);

    impl Port {
        pub fn get(&self) -> u16 {
            self.0
        }
    }
}

fn main() {
    let p = port::Port(0);
    println!("{}", p.get());
}
