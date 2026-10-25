fn main() {
    let ready = true;

    // every path gives `port` a value
    let port = if ready { 8080 } else { 3000 };
    println!("listening on {port}");
}
