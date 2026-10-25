fn call_twice(f: impl Fn()) {
    f();
    f();
}

fn main() {
    let name = String::from("ferris");
    call_twice(|| drop(name));
}
