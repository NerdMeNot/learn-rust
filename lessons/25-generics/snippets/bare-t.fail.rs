// Nothing is asked of T, so a body may not print it or compare it.
fn bracket<T>(item: T) -> String {
    format!("[{item}]")
}

fn same<T>(a: T, b: T) -> bool {
    a == b
}

fn main() {
    println!("{} {}", bracket(5), same(1, 2));
}
