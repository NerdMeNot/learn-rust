use std::collections::HashSet;

fn sorted<'a>(set: &HashSet<&'a str>) -> Vec<&'a str> {
    let mut v: Vec<&str> = set.iter().copied().collect();
    v.sort();
    v
}

fn main() {
    let rust: HashSet<&str> = HashSet::from(["ana", "bo", "cy", "dee"]);
    let go: HashSet<&str> = HashSet::from(["cy", "dee", "eli"]);

    // The order of a HashSet is not specified, so sort before printing.
    println!("union: {:?}", sorted(&rust.union(&go).copied().collect()));
    println!("intersection: {:?}", sorted(&(&rust & &go)));
    println!("difference: {:?}", sorted(&(&rust - &go)));
    println!("symmetric difference: {:?}", sorted(&(&rust ^ &go)));

    let team: HashSet<&str> = HashSet::from(["cy", "dee"]);
    println!("team is subset of rust: {}", team.is_subset(&rust));
    println!("rust is superset of team: {}", rust.is_superset(&team));
    println!("rust and go disjoint: {}", rust.is_disjoint(&go));

    // `insert` says whether the value was new; `take` removes and returns it.
    let mut seen = HashSet::new();
    println!("insert cy: {}", seen.insert("cy"));
    println!("insert cy again: {}", seen.insert("cy"));
    println!(
        "take cy: {:?}, take cy again: {:?}",
        seen.take("cy"),
        seen.take("cy")
    );
}
