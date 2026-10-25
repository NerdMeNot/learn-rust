fn main() {
    println!("[{:>8}]", "ab");
    println!("[{:>8?}]", "ab");
    println!("[{:5?}]", Some(1));
    println!("[{:>10}]", format!("{:?}", Some(1)));
}
