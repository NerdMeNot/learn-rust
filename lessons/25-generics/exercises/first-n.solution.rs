// The first N items of a slice as an array; if the slice is shorter, the rest is
// the default value. The caller picks N, usually with a type: `let a: [i32; 3] = ...`.
fn first_n<T: Clone + Default, const N: usize>(items: &[T]) -> [T; N] {
    std::array::from_fn(|i| items.get(i).cloned().unwrap_or_default())
}

// A type with very few traits: no Copy, no Debug, no PartialEq.
#[derive(Clone, Default)]
struct Tally(u32);

fn main() {
    let a: [i32; 3] = first_n(&[1, 2, 3, 4, 5]);
    assert_eq!(a, [1, 2, 3]);
    let b: [i32; 4] = first_n(&[7]);
    assert_eq!(b, [7, 0, 0, 0]);
    let c: [char; 3] = first_n(&['x', 'y', 'z']);
    assert_eq!(c, ['x', 'y', 'z']);
    let none: [char; 0] = first_n(&['x']);
    assert_eq!(none, []);
    // Strings are Clone but not Copy; the default is the empty string.
    let d: [String; 2] = first_n(&[String::from("ana")]);
    assert_eq!(d, ["ana", ""]);
    let e: [Tally; 2] = first_n(&[Tally(5)]);
    assert_eq!((e[0].0, e[1].0), (5, 0));
    // N can also be named with the turbofish.
    assert_eq!(first_n::<f64, 2>(&[1.5, 2.5, 3.5]), [1.5, 2.5]);
    println!("all checks passed");
}
