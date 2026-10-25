fn main() {
    let n = dbg!(2 + 3) * 10; // prints, and hands the value back
    println!("n = {n}");

    let name = String::from("ada");
    dbg!(&name); // dbg!(name) would move it
    dbg!(name.len(), &name);

    let v = vec![1, 2];
    dbg!(&v);
}
