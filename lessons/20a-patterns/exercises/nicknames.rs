struct Person {
    name: String,
    nick: Option<String>,
}

/// The nickname if there is one, otherwise the name. Borrow it from the person: do not clone.
fn display_name(person: &Person) -> &str {
    todo!("match &person.nick: matching person.nick itself would try to move the String out")
}

/// The longest nickname (counting characters, not bytes), borrowed from the people.
/// The first one wins a tie. None if nobody has a nickname.
fn longest_nick(people: &[Person]) -> Option<&str> {
    todo!("loop over the people and bind each nickname through a reference")
}

fn main() {
    let person = |name: &str, nick: Option<&str>| Person {
        name: String::from(name),
        nick: nick.map(String::from),
    };
    let people = vec![
        person("Ana", Some("annie")),
        person("Bo", None),
        person("Cy", Some("ñññ")),
        person("Di", Some("abcd")),
        person("Ed", Some("wxyz")),
    ];
    assert_eq!(display_name(&people[0]), "annie");
    assert_eq!(display_name(&people[1]), "Bo");

    // the answer points into the person: nothing was copied or moved out
    let shown = display_name(&people[0]);
    assert!(std::ptr::eq(
        shown.as_ptr(),
        people[0].nick.as_ref().unwrap().as_ptr()
    ));

    assert_eq!(longest_nick(&people), Some("annie"));
    // "ñññ" is 3 characters but 6 bytes, and "abcd" and "wxyz" tie at 4: the first wins
    assert_eq!(longest_nick(&people[2..]), Some("abcd"));
    assert_eq!(longest_nick(&people[1..2]), None);
    assert_eq!(longest_nick(&[]), None);

    // the people are still whole afterwards
    assert_eq!(people[0].nick.as_deref(), Some("annie"));
    assert_eq!(people[3].name, "Di");
    println!("all checks passed");
}
