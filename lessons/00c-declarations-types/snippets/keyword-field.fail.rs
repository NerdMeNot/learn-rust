struct Event {
    type: String,
}

fn main() {
    let e = Event {
        type: String::from("click"),
    };
    println!("{}", e.type);
}
