// `Copy` makes `let b = a` copy the bytes, so `a` stays usable.
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

/// Returns a point `dx` to the right of `p`. `p` is `Copy`, so the caller keeps its own.
fn shifted(p: Point, dx: i32) -> Point {
    let mut q = p;
    q.x += dx;
    q
}

/// Takes the text (it is moved in) and hands it back with its length in bytes,
/// so the caller can keep using it.
fn with_len(text: String) -> (String, usize) {
    let n = text.len();
    (text, n)
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
