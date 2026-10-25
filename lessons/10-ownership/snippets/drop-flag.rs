struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  drop {}", self.0);
    }
}

fn consume(n: Noisy) {
    println!("consumed {}", n.0);
}

fn run(give_away: bool) {
    let n = Noisy(if give_away { "given away" } else { "kept" });
    if give_away {
        consume(n); // maybe moved...
    }
    println!("end of run({give_away})");
} // ...so Rust drops n here only if it is still here

fn main() {
    run(true);
    run(false);
}
