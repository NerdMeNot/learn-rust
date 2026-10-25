use std::collections::HashMap;

// Backs the "you write -> in Rust" cells of the language tables. Never shown.
fn truthy_i32(n: i32) -> bool {
    n != 0
}

fn main() {
    // if n: / if (n)
    assert!(!truthy_i32(0) && truthy_i32(-3));
    // if s: / if items:
    let s = "";
    let items: Vec<i32> = Vec::new();
    assert!(s.is_empty() && items.is_empty());
    assert!(!"x".is_empty() && !vec![1].is_empty());
    // x if c else y, c ? a : b
    let n = 7;
    let size = if n > 5 { "big" } else { "small" };
    assert_eq!(size, "big");
    // a < b < c
    let x = 3;
    assert!(1 < x && x < 5);
    // x or d (Python), x || d (TypeScript) for numbers
    let zero = 0;
    let picked = if zero != 0 { zero } else { 5 };
    assert_eq!(picked, 5);
    // if p / p != NULL
    let p: Option<&i32> = None;
    assert!(p.is_none());
    let q = Some(&4);
    assert!(q.is_some());
    // C++17 find with init, Go v, ok := m[k]
    let mut ages: HashMap<&str, i32> = HashMap::new();
    ages.insert("ana", 31);
    let mut found = 0;
    if let Some(age) = ages.get("ana") {
        found = *age;
    }
    assert_eq!(found, 31);
    assert!(ages.get("bo").is_none());
    // Go: if v, err := Atoi(s); err != nil
    let mut got = -1;
    if let Ok(v) = "42".parse::<i32>() {
        got = v;
    }
    assert_eq!(got, 42);
    // Kotlin x != null, smart cast; Option unpacked
    let name: Option<String> = Some("Ada".to_string());
    if let Some(name) = &name {
        assert_eq!(name.len(), 3);
    }
    // Java equals(), == on text compares content
    assert!(String::from("rust") == "rust");
    // bool then_some
    assert_eq!((n > 5).then_some(1), Some(1));
    assert_eq!((n > 50).then_some(1), None);
    // Python `not`, `and`, `or`
    assert!(!(1 > 2) && (1 < 2 || 2 < 1));
    // == on text, tuples and arrays; ! on integers is bitwise
    assert!("ab" == String::from("ab"));
    assert!((1, "a") == (1, "a"));
    assert!([1, 2] == [1, 2]);
    assert_eq!(!0_i32, -1);
    // arms-differ fix: 1.0 and 2.5 are both f64
    let limit = if n > 5 { 1.0 } else { 2.5 };
    assert_eq!(limit, 1.0);
    // x ?? d, x if x is not None else d
    assert_eq!(Some(0).unwrap_or(9), 0);
    assert_eq!(None.unwrap_or(9), 9);
    // == compares contents of arrays and Vecs
    assert!(vec![1, 2] == vec![1, 2]);
    assert!(vec![1, 2] == [1, 2]);
    // parse rejects an overflowing number
    assert!("99999999999".parse::<i32>().is_err());
    println!("rows hold");
}
