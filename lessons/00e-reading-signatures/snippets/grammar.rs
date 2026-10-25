use std::error::Error;
use std::fmt::Display;

trait Describe {
    fn describe(&self) -> String;
}

struct Square(u32);

impl Describe for Square {
    fn describe(&self) -> String {
        format!("square of side {}", self.0)
    }
}

// `T` is any type that can be printed and cloned
fn label<T: Display + Clone>(item: &T) -> String {
    format!("<{}>", item.clone())
}

// two type parameters, with their bounds moved into a `where` clause
fn pair<A, B>(a: &A, b: &B) -> String
where
    A: Display,
    B: Display,
{
    format!("{a}/{b}")
}

// `'a` ties the result to both inputs: it may borrow from either
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if y.len() > x.len() { y } else { x }
}

// an array of exactly 3, and a plain function as a parameter
fn total(values: [i32; 3], tweak: fn(i32) -> i32) -> i32 {
    let mut sum = 0;
    for v in values {
        sum += tweak(v);
    }
    sum
}

fn double(n: i32) -> i32 {
    n * 2
}

// returns some closure; the caller only knows it implements `Fn(i32) -> i32`
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

// accepts anything that implements `Describe`, chosen while the program runs
fn show(item: &dyn Describe) -> String {
    item.describe()
}

// `Box<dyn Error>` can hold any error type
fn load(text: &str) -> Result<i32, Box<dyn Error>> {
    let n = text.trim().parse::<i32>()?;
    Ok(n)
}

// `-> !` never returns normally
fn stop(message: &str) -> ! {
    panic!("{message}")
}

fn checked(n: i32) -> i32 {
    if n < 0 {
        stop("negative");
    }
    n
}

fn main() {
    println!("label: {}", label(&"hi"));
    println!("pair: {}", pair(&1, &"two"));
    println!("longest: {}", longest("ab", "abc"));
    println!("total: {}", total([1, 2, 3], double));
    println!("adder: {}", make_adder(10)(5));
    println!("show: {}", show(&Square(4)));
    println!("load: {:?}", load(" 42 ").ok());
    println!("load bad: {}", load("x").is_err());
    println!("checked: {}", checked(7));
}
