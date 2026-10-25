fn main() {
    let v = vec![1, 2, 3];
    let i = v.len() + 6;
    // SAFETY: none. This breaks the rule that `i` must be less than `v.len()`.
    let x = unsafe { *v.get_unchecked(i) };
    println!("{x}");
}
