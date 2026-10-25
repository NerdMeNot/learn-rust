fn main() {
    let word = "naïve";
    println!("{} bytes, {} chars", word.len(), word.chars().count());

    // chars: Unicode scalar values. bytes: the raw UTF-8.
    let chars: Vec<char> = word.chars().collect();
    let bytes: Vec<u8> = word.bytes().collect();
    println!("chars: {chars:?}");
    println!("bytes: {bytes:?}");

    // char_indices: each char with the byte position it starts at.
    let indices: Vec<(usize, char)> = word.char_indices().collect();
    println!("char_indices: {indices:?}");

    // The nth character is a walk, not a jump: O(n).
    println!("chars().nth(2): {:?}", word.chars().nth(2));

    // Slicing uses byte positions. get() returns None instead of panicking
    // when the range would cut a character in half.
    println!("get(0..2): {:?}", word.get(0..2));
    println!("get(0..3): {:?}", word.get(0..3));
    println!("get(0..4): {:?}", word.get(0..4));

    // A char is always 4 bytes in memory, whatever it encodes to in a string.
    println!("size of a char: {}", size_of::<char>());
    // One thing a person sees can be several chars: this flag is two, in 8 bytes.
    let flag = "🇵🇹";
    println!("flag: {} chars, {} bytes", flag.chars().count(), flag.len());
}
