fn main() {
    let short = String::from("hello");
    let long = "x".repeat(1000);

    println!(
        "short: {} bytes of text, variable takes {} bytes",
        short.len(),
        size_of_val(&short)
    );
    println!(
        "long:  {} bytes of text, variable takes {} bytes",
        long.len(),
        size_of_val(&long)
    );
    println!("capacity of short: {}", short.capacity());

    // An array keeps its elements inside the variable, so its size grows with them.
    let small_array = [1u8; 5];
    let big_array = [1u8; 1000];
    println!("array of 5 bytes takes {} bytes", size_of_val(&small_array));
    println!(
        "array of 1000 bytes takes {} bytes",
        size_of_val(&big_array)
    );
}
