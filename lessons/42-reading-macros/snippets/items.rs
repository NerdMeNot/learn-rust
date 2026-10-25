trait Describe {
    fn describe(&self) -> String;
}

// One impl block for each type in the list
macro_rules! describe_all {
    ($($t:ty),+ $(,)?) => {
        $(impl Describe for $t {
            fn describe(&self) -> String {
                format!("{} {}", stringify!($t), self)
            }
        })+
    };
}
describe_all!(i32, u8, f64, bool);

// A function whose name the caller chooses
macro_rules! make_adder {
    ($name:ident, $n:expr) => {
        fn $name(x: i32) -> i32 {
            x + $n
        }
    };
}
make_adder!(add_one, 1);
make_adder!(add_ten, 10);

fn main() {
    println!("{}", 7_i32.describe());
    println!("{}", 7_u8.describe());
    println!("{}", 2.5_f64.describe());
    println!("{}", true.describe());
    println!("add_one(5) = {}, add_ten(5) = {}", add_one(5), add_ten(5));
}
