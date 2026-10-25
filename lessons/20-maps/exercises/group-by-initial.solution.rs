use std::collections::HashMap;

/// Group names by their first character. Names keep their order inside a group,
/// and an empty name belongs to no group.
fn group_by_initial(names: &[&str]) -> HashMap<char, Vec<String>> {
    let mut groups: HashMap<char, Vec<String>> = HashMap::new();
    for name in names {
        let Some(first) = name.chars().next() else {
            continue;
        };
        groups.entry(first).or_default().push(name.to_string());
    }
    groups
}

fn main() {
    assert!(group_by_initial(&[]).is_empty());

    let groups = group_by_initial(&["ana", "al", "bo", "", "élan", "ed"]);
    assert_eq!(groups.len(), 4);
    assert_eq!(groups[&'a'], vec!["ana", "al"]);
    assert_eq!(groups[&'b'], vec!["bo"]);
    assert_eq!(groups[&'é'], vec!["élan"]);
    assert_eq!(groups[&'e'], vec!["ed"]);
    assert_eq!(groups.get(&'x'), None);

    println!("all checks passed");
}
