fn main() {
    let mut tries = 0;
    let result = loop {
        tries += 1;
        if tries < 3 {
            continue;
        }
        break tries * 10;
    };
    println!("tries: {tries}, result: {result}");
}
