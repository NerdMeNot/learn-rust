fn main() {
    let name = String::from("ferris");
    let greet = move || println!("hello, {name}");
    greet();
    greet();
}
