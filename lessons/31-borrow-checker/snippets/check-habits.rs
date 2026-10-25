// Backs the Go and Python/TypeScript rows of the habit tables.
struct Item {
    name: String,
    count: u32,
}

fn main() {
    // Go: `for _, it := range items { ptrs = append(ptrs, &it) }` becomes a loop over `&items`:
    // each reference points at the element itself, not at a copy.
    let items = vec![
        Item {
            name: "a".into(),
            count: 1,
        },
        Item {
            name: "b".into(),
            count: 2,
        },
    ];
    let mut refs: Vec<&Item> = Vec::new();
    for it in &items {
        refs.push(it);
    }
    assert!(std::ptr::eq(refs[0], &items[0]));
    assert!(std::ptr::eq(refs[1], &items[1]));
    assert_eq!(refs[1].name, "b");
    assert_eq!(refs[1].count, 2);

    // Go: `a[:2]` shares memory. In Rust `to_vec()` is the independent copy,
    // and `split_at_mut` gives two writable halves that cannot overlap.
    let mut a = vec![1, 2, 3, 4];
    let copy = a[..2].to_vec();
    a[2] = 99;
    assert_eq!(copy, [1, 2]);
    let (left, right) = a.split_at_mut(2);
    left[0] = right[0];
    assert_eq!(a, [99, 2, 99, 4]);
}
