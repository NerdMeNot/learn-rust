fn main() {
    println!("size_of::<char>(): {}", size_of::<char>());
    println!("size_of::<u8>(): {}", size_of::<u8>());
    println!("'e'.len_utf8(): {}", 'e'.len_utf8());
    println!("'é'.len_utf8(): {}", 'é'.len_utf8());
    println!("'🦀'.len_utf8(): {}", '🦀'.len_utf8());
    println!("\"é\".len(): {}", "é".len());
}
