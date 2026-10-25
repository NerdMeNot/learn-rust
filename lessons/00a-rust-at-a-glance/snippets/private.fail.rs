mod shapes {
    pub struct Square {
        side: u32,
    }

    impl Square {
        pub fn new(side: u32) -> Square {
            Square { side }
        }
    }
}

fn main() {
    let sq = shapes::Square::new(4);
    println!("{}", sq.side);
}
