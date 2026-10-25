use std::mem::size_of;

/// How many bytes the elements of `v` take on the heap, counting spare room.
fn heap_bytes(v: &Vec<i32>) -> usize {
    todo!("capacity times the size of one element")
}

/// How many bytes the variable `v` itself takes (the header), however many elements it has.
fn header_bytes(v: &Vec<i32>) -> usize {
    todo!("the size of the Vec type itself, not of its elements")
}

/// How many more elements fit before the Vec has to ask for a bigger buffer.
fn spare(v: &Vec<i32>) -> usize {
    todo!("capacity minus length")
}

fn main() {
    let mut v: Vec<i32> = Vec::with_capacity(10);
    v.push(1);
    v.push(2);
    v.push(3);

    assert_eq!(header_bytes(&v), 3 * size_of::<usize>());
    assert_eq!(spare(&v), v.capacity() - 3);
    assert!(spare(&v) >= 7);
    assert_eq!(heap_bytes(&v), v.capacity() * 4);

    // The header stays the same size when the Vec grows.
    let before = header_bytes(&v);
    for i in 0..1000 {
        v.push(i);
    }
    assert_eq!(header_bytes(&v), before);
    assert!(heap_bytes(&v) >= 1003 * 4);
    println!("all checks passed");
}
