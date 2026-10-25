fn log(msg: &str) {
    println!("log: {msg}");
}

fn main() {
    let c: i32 = (log("first"), 2);
    println!("comma: {c}");
}
