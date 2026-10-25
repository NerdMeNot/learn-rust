use std::thread::Builder;

fn main() {
    let handle = Builder::new()
        .name("indexer".to_string())
        .spawn(|| panic!("disk full"))
        .unwrap();
    // join's Err is a value; unwrap turns it into a panic in main
    handle.join().unwrap();
}
