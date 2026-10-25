// Trim the spaces around each name and make it lowercase, changing the list in place.
fn tidy(names: &mut Vec<String>) {
    for n in names.iter_mut() {
        *n = n.trim().to_lowercase();
    }
}

fn main() {
    let mut names = vec![
        String::from("  Ana "),
        String::from("BO"),
        String::from(" ÉLAN"),
    ];
    tidy(&mut names);
    assert_eq!(names, ["ana", "bo", "élan"]);

    let mut empty: Vec<String> = Vec::new();
    tidy(&mut empty);
    assert!(empty.is_empty());

    let mut blanks = vec![String::from("   "), String::from("x")];
    tidy(&mut blanks);
    assert_eq!(blanks, ["", "x"]);
    println!("all checks passed");
}
