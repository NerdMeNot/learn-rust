struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  drop {}", self.0);
    }
}

struct Order {
    item: Noisy,
}

// Takes ownership. The value is dropped when this function ends,
// unless the function hands it on.
fn consume(n: Noisy) {
    println!("consume got {}", n.0);
}

// Takes ownership and gives it back to the caller.
fn pass_back(n: Noisy) -> Noisy {
    println!("pass_back got {}", n.0);
    n
}

fn make() -> Noisy {
    Noisy("made") // moves out to the caller; nothing is dropped
}

fn main() {
    let a = Noisy("a");
    consume(a); // a moves into the function...
    println!("back in main"); // ...and was dropped before this line

    let b = Noisy("b");
    let b2 = pass_back(b); // b moved in, and back out as b2
    println!("main owns {} again", b2.0);

    let m = make();
    println!("main owns {}", m.0);

    let mut list = Vec::new();
    list.push(Noisy("in vec")); // the Vec owns it now

    let order = Order {
        item: Noisy("in struct"),
    }; // the struct owns it
    println!("order holds {}", order.item.0);

    println!("end of main");
}
