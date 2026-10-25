// Remove the longest name from the list and return it (the first one, if two
// are equally long). Return None, and change nothing, if the list is empty.
// The returned String must be the one that was in the list: no cloning.
fn take_longest(names: &mut Vec<String>) -> Option<String> {
    let mut longest: Option<usize> = None;
    for (i, name) in names.iter().enumerate() {
        match longest {
            Some(best) if names[best].len() >= name.len() => {}
            _ => longest = Some(i),
        }
    }
    Some(names.remove(longest?))
}

fn main() {
    let mut names = vec![
        String::from("ana"),
        String::from("beatrice"),
        String::from("bo"),
        String::from("cordelia"),
    ];
    assert_eq!(take_longest(&mut names), Some(String::from("beatrice")));
    assert_eq!(names, ["ana", "bo", "cordelia"]); // gone from the list, not copied
    assert_eq!(take_longest(&mut names), Some(String::from("cordelia")));
    assert_eq!(names, ["ana", "bo"]);
    assert_eq!(take_longest(&mut names), Some(String::from("ana")));
    assert_eq!(take_longest(&mut names), Some(String::from("bo")));
    assert!(names.is_empty());
    assert_eq!(take_longest(&mut names), None);
    println!("all checks passed");
}
