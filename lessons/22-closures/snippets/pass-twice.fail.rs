fn call_twice(f: impl Fn()) {
    f();
    f();
}

fn main() {
    let name = String::from("ferris");
    let greet = move || println!("hello, {name}");
    call_twice(greet);
    call_twice(greet);
}
