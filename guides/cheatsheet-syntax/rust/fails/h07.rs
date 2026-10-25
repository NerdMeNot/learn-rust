fn show(n: i32) {
    println!("int {n}");
}

fn show(s: &str) {
    println!("text {s}");
}

fn main() {
    show(1);
    show("a");
}
