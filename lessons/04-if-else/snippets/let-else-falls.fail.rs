fn main() {
    let text = "abc";
    let Ok(n) = text.parse::<i32>() else {
        println!("not a number");
    };
    println!("n: {n}");
}
