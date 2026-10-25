fn call_twice(f: impl Fn()) {
    f();
    f();
}

fn main() {
    let name = String::from("ferris");
    let greet = move || println!("hello, {name}");
    call_twice(&greet); // lend it: `greet` is still ours
    call_twice(&greet);
    greet();
}
