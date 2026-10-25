use std::collections::HashMap;

// Backs the "example -> result" cells of the method table on the page.
// `m` starts as {"a": 1}.
fn fresh() -> HashMap<&'static str, i32> {
    HashMap::from([("a", 1)])
}

fn main() {
    let mut m = fresh();
    assert_eq!(m.insert("b", 2), None);
    assert_eq!(m.len(), 2);

    let mut m = fresh();
    assert_eq!(m.insert("a", 5), Some(1));
    assert_eq!(m.get("a"), Some(&5));

    let mut m = fresh();
    assert_eq!(m.get("a"), Some(&1));
    assert_eq!(m.get("z"), None);
    assert_eq!(m.contains_key("a"), true);
    assert_eq!(m.contains_key("z"), false);
    assert_eq!(m["a"], 1);

    if let Some(v) = m.get_mut("a") {
        *v += 1;
    }
    assert_eq!(m["a"], 2);
    assert_eq!(m.get_mut("z"), None);

    assert_eq!(m.remove("a"), Some(2));
    assert_eq!(m.remove("a"), None);
    assert!(m.is_empty());

    let mut m = fresh();
    *m.entry("a").or_insert(0) += 1;
    *m.entry("z").or_insert(0) += 1;
    assert_eq!((m["a"], m["z"]), (2, 1));
    m.clear();
    assert_eq!(m.len(), 0);

    // the "default if missing" and bulk cells of the language tables
    let mut m = fresh();
    assert_eq!(m.get("a").copied().unwrap_or(0), 1);
    assert_eq!(m.get("z").copied().unwrap_or(0), 0);
    assert_eq!(m.get("z").copied().unwrap_or_default(), 0);
    m.extend([("a", 7), ("b", 8)]);
    assert_eq!((m["a"], m["b"], m.len()), (7, 8, 2));
}
