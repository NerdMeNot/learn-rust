fn main() {
    let n = 7;

    let block = {
        let doubled = n * 2;
        doubled + 1
    };
    let branch = if n > 5 { "big" } else { "small" };
    let name = match n {
        1 => "one",
        2 => "two",
        _ => "many",
    };
    let mut count = 0;
    let found = loop {
        count += 3;
        if count % 7 == 0 {
            break count;
        }
    };

    println!("block: {block}");
    println!("branch: {branch}");
    println!("match: {name}");
    println!("loop: {found}");
}
