fn main() {
    let (a, b) = std::future::join!(async { 1 }, async { 2 });
    let _ = (a, b);
}
