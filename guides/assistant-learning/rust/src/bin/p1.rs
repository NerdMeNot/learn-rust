fn main() {
    let names = vec![String::from("ada"), String::from("grace")];
    for name in names {
        println!("hello, {name}");
    }
    println!("{} names", names.len());
}
