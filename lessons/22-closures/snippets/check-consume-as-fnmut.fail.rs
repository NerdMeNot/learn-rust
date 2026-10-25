fn call_twice(mut f: impl FnMut()) {
    f();
    f();
}

fn main() {
    let name = String::from("ferris");
    call_twice(|| drop(name));
}
