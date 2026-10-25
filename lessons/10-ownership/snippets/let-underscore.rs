struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  drop {}", self.0);
    }
}

fn main() {
    println!("let _ = Noisy(…): a new value nobody holds");
    let _ = Noisy("temporary"); // dropped right here, at the end of the statement

    println!("let _ = x: an existing variable");
    let x = Noisy("x");
    let _ = x; // not a move and not a drop: x is untouched
    println!("x is still usable: {}", x.0);

    println!("let _kept = …");
    let _kept = Noisy("kept"); // a real variable: lives to the end of main
    println!("end of main");
}
