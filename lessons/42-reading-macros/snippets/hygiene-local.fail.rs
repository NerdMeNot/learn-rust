macro_rules! make {
    () => {
        let local = 5;
    };
}

fn main() {
    make!();
    println!("local = {}", local);
}
