fn send(msg: String) -> usize {
    msg.len()
}

fn main() {
    let msg = String::from("hello world");
    let first = msg.split(' ').next().unwrap();
    let sent = send(msg);
    println!("sent {sent} bytes, first word {first}");
}
