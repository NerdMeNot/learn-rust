use std::mem::size_of;

fn main() {
    println!("size_of::<[i32; 4]>():   {}", size_of::<[i32; 4]>());
    println!("size_of::<[i32; 100]>(): {}", size_of::<[i32; 100]>());
    println!("size_of::<Vec<i32>>():   {}", size_of::<Vec<i32>>());
    println!("size_of::<&[i32]>():     {}", size_of::<&[i32]>());

    let a = [1, 2, 3];
    let b = a; // an array of Copy items is Copy
    println!("a: {a:?}, b: {b:?}");

    let zeros = [0u8; 6];
    println!("[0u8; 6]: {zeros:?}, len {}", zeros.len());

    let v: Vec<i32> = a.to_vec();
    println!("a.to_vec(): {v:?}");
}
