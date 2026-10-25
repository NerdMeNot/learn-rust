fn counted() {
    for (int i = 0; i < 3; i++) {
        println!("{i}");
    }
}

fn run_once() {
    let mut n = 0;
    do {
        n += 1;
    } while (n < 3);
}

fn main() {}
