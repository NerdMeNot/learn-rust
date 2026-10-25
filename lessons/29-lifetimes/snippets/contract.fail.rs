fn pick<'a, 'b>(x: &'a str, y: &'b str, first: bool) -> &'a str {
    if first { x } else { y }
}

fn main() {
    println!("pick: {}", pick("x", "y", false));
}
