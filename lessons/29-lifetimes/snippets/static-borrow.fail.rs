fn consume<T: 'static>(_value: T) {}

fn main() {
    let name = String::from("local");
    consume(&name);
}
