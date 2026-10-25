struct Guard<'a>(&'a String);

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        println!("dropping a guard for {}", self.0);
    }
}

fn main() {
    let name = String::from("Ada");
    let guard = Guard(&name);

    let moved = name; // Guard::drop will still read the borrowed name
    println!("moved: {moved}");
}
