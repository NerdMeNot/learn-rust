struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn leave_early(early: bool) -> i32 {
    let p = Box::new(Noisy("p"));
    if early {
        return 1; // p is dropped here, on this path too
    }
    println!("end of function, p holds {}", p.0);
    0
}

fn main() {
    println!("early: {}", leave_early(true));
    println!("late: {}", leave_early(false));
    let a = Box::new(Noisy("a"));
    let b = a; // a move: one owner, so one drop
    println!("moved, b holds {}", b.0);
}
