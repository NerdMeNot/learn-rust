// Backs the "what you write today" tables: each Rust cell is compiled and run here.
use std::collections::HashMap;

struct User {
    name: String,
    email: Option<String>,
}

fn main() {
    // The ?. / ?: rows. `map` takes the Option by value, so `user` is used up.
    let user = Some(User {
        name: String::from("ana"),
        email: None,
    });
    let shown = user.map(|u| u.name).unwrap_or_else(|| "anon".to_string());
    assert_eq!(shown, "ana");

    // Borrow first and `user` stays usable.
    let user = Some(User {
        name: String::from("ana"),
        email: Some(String::from("a@b.c")),
    });
    let shown = user.as_ref().map(|u| u.name.as_str()).unwrap_or("anon");
    assert_eq!(shown, "ana");
    assert_eq!(
        user.as_ref().and_then(|u| u.email.as_deref()),
        Some("a@b.c")
    );
    assert!(user.is_some());

    // if let on a reference does not move.
    if let Some(u) = &user {
        assert_eq!(u.name, "ana");
    }
    assert!(user.is_some());

    // Maps: Go's m[k] gives the zero value on a miss, and so does unwrap_or_default.
    let mut ages: HashMap<&str, i32> = HashMap::new();
    ages.insert("ana", 31);
    assert_eq!(ages.get("bo").copied().unwrap_or_default(), 0);
    assert_eq!(ages.get("bo").copied().unwrap_or(10), 10);
    assert_eq!(ages.get("ana").copied().unwrap_or(10), 31);

    // Lists: get is the checked index.
    let list = vec![5, 6];
    assert_eq!(list.get(1).copied().unwrap_or(0), 6);
    assert_eq!(list.get(9).copied().unwrap_or(0), 0);
    assert_eq!(list.first(), Some(&5));

    // parse(...).ok() drops the reason, like int.TryParse, toIntOrNull or Atoi with _.
    assert_eq!("12".parse::<i32>().ok(), Some(12));
    assert_eq!("x".parse::<i32>().ok(), None);

    // `x or 5` in Python replaces 0; unwrap_or only replaces None.
    assert_eq!(Some(0).unwrap_or(5), 0);

    // Option<i32> is Copy, so it can be used again after a method call that takes it by value.
    let n = Some(4);
    let doubled = n.map(|v| v * 2);
    let kept = n.filter(|v| *v > 3);
    assert_eq!((doubled, kept), (Some(8), Some(4)));

    // is_some_and takes self by value: borrow first for a String.
    let name = Some(String::from("anabelle"));
    assert!(name.as_ref().is_some_and(|n| n.len() > 5));
    assert!(name.is_some());

    // Lazy default, like Kotlin's ?: and C#'s ??.
    let mut calls = 0;
    let _ = Some(1).unwrap_or_else(|| {
        calls += 1;
        0
    });
    assert_eq!(calls, 0);

    // The "early return" rows.
    fn len_of(x: Option<&str>) -> Option<usize> {
        let s = x?;
        Some(s.len())
    }
    assert_eq!(len_of(Some("abc")), Some(3));
    assert_eq!(len_of(None), None);
    fn first_or(x: Option<&str>) -> &str {
        let Some(s) = x else {
            return "none";
        };
        s
    }
    assert_eq!(first_or(None), "none");
    println!("all table cells hold");
}
