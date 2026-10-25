// Backs "without the `impl Drop` it compiles": the same program, minus the Drop impl.
struct Guard<'a>(&'a mut Vec<i32>);

fn main() {
    let mut v = vec![1];
    let guard = Guard(&mut v);
    println!("last use of guard: {}", guard.0.len());
    v.push(2);
    assert_eq!(v, [1, 2]);
}
