use std::any::type_name_of_val;

fn main() {
    let mut bytes = Vec::new(); // no type yet...
    bytes.push(1u8); // ...this line makes it a Vec<u8>

    let whole = 7; // nothing decides: an integer defaults to i32
    let fraction = 7.0; // and a float to f64
    let small = 7;
    let total: u64 = small + 1; // `small` becomes u64, because it is added to a u64

    println!("bytes[0]: {}", type_name_of_val(&bytes[0]));
    println!("whole: {}", type_name_of_val(&whole));
    println!("fraction: {}", type_name_of_val(&fraction));
    println!("small: {}", type_name_of_val(&small));
    println!("total: {total}");
}
