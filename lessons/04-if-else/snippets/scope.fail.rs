fn main() {
    if let Ok(v) = "5".parse::<i32>() {
        println!("inside: {v}");
    }
    println!("outside: {v}");
}
