fn deferred() {
    defer println!("done");
}

fn tagless(x: i32) {
    switch {
        case x > 5:
            println!("big");
    }
}

fn swap() {
    let (mut a, mut b) = (1, 2);
    a, b = b, a;
}

fn jump() {
    goto done;
}

fn main() {}
