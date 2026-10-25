// `Copy` makes `let b = a` copy the bytes, so `a` stays usable.
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

/// Returns a point `dx` to the right of `p`. `p` is `Copy`, so the caller keeps its own.
fn shifted(p: Point, dx: i32) -> Point {
    todo!("make a copy with `let mut q = p;`, change q.x, and return q")
}

/// Takes the text (it is moved in) and hands it back with its length in bytes,
/// so the caller can keep using it.
fn with_len(text: String) -> (String, usize) {
    todo!("get the length first, then build the tuple: `(text, text.len())` would not compile")
}

fn main() {
    let p = Point { x: 1, y: 2 };
    let q = shifted(p, 10);
    assert_eq!((p.x, p.y), (1, 2)); // p is untouched, and still usable
    assert_eq!((q.x, q.y), (11, 2));

    let text = String::from("héllo");
    let (text, n) = with_len(text); // the old `text` was moved; this is a new one
    assert_eq!(text, "héllo");
    assert_eq!(n, 6); // "é" takes two bytes
    println!("all checks passed");
}
