// The gotcha's call used `"Ada": 36`. Can a rule be written to accept it?
macro_rules! pair {
    ($key:expr : $value:expr) => {
        ($key, $value)
    };
}

fn main() {
    let p = pair!("Ada": 36);
    println!("{p:?}");
}
