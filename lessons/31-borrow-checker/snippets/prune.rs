use std::collections::HashMap;

fn stock() -> HashMap<&'static str, u32> {
    HashMap::from([("bolt", 0), ("nut", 12), ("washer", 0)])
}

fn sorted(mut names: Vec<&str>) -> Vec<&str> {
    names.sort();
    names
}

fn main() {
    // Just drop them: retain keeps what the closure accepts.
    let mut a = stock();
    a.retain(|_, count| *count > 0);
    println!("retain: {:?}", sorted(a.keys().copied().collect()));

    // Need the removed ones too: extract_if hands them to you.
    let mut b = stock();
    let gone: Vec<&str> = b
        .extract_if(|_, count| *count == 0)
        .map(|(name, _)| name)
        .collect();
    println!(
        "extract_if removed {:?}, kept {:?}",
        sorted(gone),
        sorted(b.keys().copied().collect())
    );

    // extract_if is lazy: an iterator that is never consumed removes nothing.
    let mut d = stock();
    let _ = d.extract_if(|_, count| *count == 0);
    println!("unconsumed extract_if: {} entries left", d.len());

    // Decide in one pass, change in the next. The keys are copied (they are &str).
    let mut c = stock();
    let empty: Vec<&str> = c
        .iter()
        .filter(|(_, count)| **count == 0)
        .map(|(name, _)| *name)
        .collect();
    for name in &empty {
        c.remove(name);
    }
    println!("two passes: {:?}", sorted(c.keys().copied().collect()));
}
