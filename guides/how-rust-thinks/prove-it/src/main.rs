fn main() {
    // true here, and true every time you run it. The compiler still does not use
    // that to decide whether `port` has a value on every path.
    let ready = true;

    let port: u16;
    if ready {
        port = 8080;
    }
    println!("listening on {port}");
}
