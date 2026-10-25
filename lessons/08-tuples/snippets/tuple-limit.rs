fn main() {
    let twelve = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("twelve: {twelve:?}");

    // A 13-slot tuple is a fine type: you can build it, read a slot and copy it...
    let thirteen = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    let copy = thirteen;
    println!("thirteen.12: {}, copy.12: {}", thirteen.12, copy.12);
    // ...but `{:?}` on it is refused (see below).
}
