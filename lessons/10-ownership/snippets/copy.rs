// A small struct made only of Copy fields can be Copy too. Copy needs Clone.
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // Numbers, bool and char are copied. Both names stay usable.
    let n = 5;
    let m = n;
    let ok = true;
    let ok2 = ok;
    let c = 'é';
    let c2 = c;
    println!("n {n}, m {m}, ok {ok} {ok2}, c {c} {c2}");

    // Tuples and arrays are Copy when everything in them is.
    let pair = (1, 2.5);
    let pair2 = pair;
    let grid = [0u8; 4];
    let grid2 = grid;
    println!("pair {pair:?} {pair2:?}, grid {grid:?} {grid2:?}");

    // Size doesn't matter: 1000 u64s is 8000 bytes, and still Copy.
    let big = [7u64; 1000];
    let big2 = big;
    println!(
        "big[999] {}, big2[999] {}, size {} bytes",
        big[999],
        big2[999],
        size_of_val(&big)
    );

    // A shared reference is Copy: a second way to look, not a second value.
    let text = String::from("shared");
    let r1 = &text;
    let r2 = r1;
    println!("r1 {r1}, r2 {r2}");

    // Your own type, thanks to #[derive(Clone, Copy)].
    let p = Point { x: 1, y: 2 };
    let mut q = p; // a copy
    q.x = 10;
    println!("p ({}, {}), q ({}, {})", p.x, p.y, q.x, q.y);
}
