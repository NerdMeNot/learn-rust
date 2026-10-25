fn show(n: i32) -> String {
    format!("int {n}")
}

fn show(s: &str) -> String {
    format!("text {s}")
}

fn main() {
    println!("{}", show(1));
    println!("{}", show("a"));
}
