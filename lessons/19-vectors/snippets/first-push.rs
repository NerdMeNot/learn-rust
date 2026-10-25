fn main() {
    let mut words: Vec<u32> = Vec::new();
    words.push(1);
    println!("first push, Vec<u32>:        capacity {}", words.capacity());

    let mut bytes: Vec<u8> = Vec::new();
    bytes.push(1);
    println!("first push, Vec<u8>:         capacity {}", bytes.capacity());

    let mut blocks: Vec<[u8; 2000]> = Vec::new();
    blocks.push([0; 2000]);
    println!(
        "first push, Vec<[u8; 2000]>: capacity {}",
        blocks.capacity()
    );
}
