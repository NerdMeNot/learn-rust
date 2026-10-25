use std::future::Future;
fn make() -> impl Future<Output = usize> {
    let s = String::from("hello");
    async { s.len() }
}
fn main() {
    let _ = make();
}
