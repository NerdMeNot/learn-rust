use std::mem::size_of;

fn main() {
    let mut v: Vec<i32> = Vec::with_capacity(4);
    v.extend([10, 20, 30]);
    println!("size_of::<Vec<i32>>(): {} bytes", size_of::<Vec<i32>>());
    println!(
        "three words: {}",
        size_of::<Vec<i32>>() == 3 * size_of::<usize>()
    );
    println!("len: {}", v.len());
    println!("capacity >= 4: {}", v.capacity() >= 4);
    // slot 3 is inside the buffer but holds no value yet: only `len` slots are initialised
    println!("get(3): {:?}", v.get(3));
}
