use std::fmt::Display;

trait Speaker {
    fn speak(&self) -> String;
}

struct Dog;
struct Robot {
    id: u32,
}

impl Speaker for Dog {
    fn speak(&self) -> String {
        String::from("woof")
    }
}

impl Speaker for Robot {
    fn speak(&self) -> String {
        format!("beep {}", self.id)
    }
}

impl Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "robot #{}", self.id)
    }
}

// 1. a type parameter with a bound
fn intro_a<T: Speaker>(s: &T) -> String {
    format!("a: {}", s.speak())
}

// 2. the same, written as `impl Trait` in the parameter
fn intro_b(s: &impl Speaker) -> String {
    format!("b: {}", s.speak())
}

// 3. the same again, with the bound moved into a where clause
fn intro_c<T>(s: &T) -> String
where
    T: Speaker,
{
    format!("c: {}", s.speak())
}

// several bounds are joined with +
fn label<T: Speaker + Display>(s: &T) -> String {
    format!("{} says {}", s, s.speak())
}

fn main() {
    println!("{}", intro_a(&Dog));
    println!("{}", intro_b(&Dog));
    println!("{}", intro_c(&Robot { id: 7 }));
    println!("{}", label(&Robot { id: 7 }));
}
