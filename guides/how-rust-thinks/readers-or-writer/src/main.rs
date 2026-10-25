fn main() {
    let mut list = vec![1, 2, 3];

    let total: i32 = list.iter().sum(); // read...
    list.push(total); // ...then write. Fine: the read is over.

    let first = &list[0]; // a reader that is still in use below
    list.clear(); // a writer at the same time
    println!("{first}");
}
