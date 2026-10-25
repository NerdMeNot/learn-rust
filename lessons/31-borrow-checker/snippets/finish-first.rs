use std::mem;

fn send(msg: String) -> usize {
    msg.len()
}

fn main() {
    // E0505: use the borrowed word first, or copy it out of the message.
    let msg = String::from("hello world");
    let first = msg.split(' ').next().unwrap().to_string();
    let sent = send(msg);
    println!("sent {sent} bytes, first word {first}");

    // E0506: take the old value out as you put the new one in.
    let mut position = String::from("start");
    let before = mem::replace(&mut position, String::from("middle"));
    println!("moved from {before} to {position}");
}
