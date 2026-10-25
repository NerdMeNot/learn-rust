// Prints each format string next to what it produced, between |bars|.
macro_rules! row {
    ($fmt:literal $(, $arg:expr)*) => {
        println!("{:<12} |{}|", $fmt, format!($fmt $(, $arg)*))
    };
}

#[derive(Debug)]
#[allow(dead_code)]
struct P {
    x: i32,
    y: i32,
}

fn main() {
    let name = "Ana";
    let pi = 3.14159;
    let p = P { x: 1, y: 2 };
    row!("{}", name);
    row!("{:?}", name);
    row!("{:?}", p);
    row!("{:>8}", name);
    row!("{:<8}", name);
    row!("{:^8}", name);
    row!("{:*^9}", name);
    row!("{:08.3}", pi);
    row!("{:.2}", pi);
    row!("{:+}", 5);
    row!("{:5}", 42);
    row!("{:#x}", 255);
    row!("{:#010b}", 5);
    row!("{name}");
    row!("{0}-{0}-{1}", "a", "b");
    row!("{{}}");
    row!("{:>1$}", name, 6);
    println!("{:#?}", p);
}
