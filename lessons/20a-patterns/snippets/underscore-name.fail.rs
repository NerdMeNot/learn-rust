fn main() {
    let s = String::from("hello");
    let _unused = s; // a name starting with `_` is still a name: it binds
    println!("{s}");
}
