fn main() {
    let inputs = ["4", "x", "9"];
    let mut total = 0;
    for text in inputs {
        let n: i32 = match text.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("skipping {text:?}");
                continue;
            }
        };
        total += n;
    }
    println!("total: {total}");
}
