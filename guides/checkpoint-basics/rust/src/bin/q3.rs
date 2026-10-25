fn main() {
    let target = 5;
    for n in [3, 5, 8] {
        match n {
            target => println!("{n} is the target"),
        }
    }
}
