struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  drop {}", self.0);
    }
}

struct Pair {
    _first: Noisy,
    _second: Noisy,
}

fn main() {
    let _a = Noisy("a");
    let _b = Noisy("b");
    let c = Noisy("c");

    drop(c); // ends c now, not at the end of main
    println!("after drop(c)");

    let mut d = Noisy("d (old)");
    println!("d is {}, reassigning", d.0);
    d = Noisy("d (new)"); // the old value is dropped right here
    println!("d is now {}", d.0);

    let _ = Noisy("unnamed"); // a new value that `_` doesn't bind: dropped at once
    let _kept = Noisy("_kept"); // a name starting with _ does bind, and lives on
    println!("after the two lets");

    let _pair = Pair {
        _first: Noisy("pair.first"),
        _second: Noisy("pair.second"),
    };
    let _list = vec![Noisy("list[0]"), Noisy("list[1]")];

    println!("end of main");
}
