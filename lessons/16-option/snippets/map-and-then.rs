struct User {
    name: &'static str,
    email: Option<&'static str>,
}

fn find_user(id: u32) -> Option<User> {
    match id {
        1 => Some(User {
            name: "ana",
            email: Some("ana@example.com"),
        }),
        2 => Some(User {
            name: "bo",
            email: None,
        }),
        _ => None,
    }
}

fn main() {
    // map: change the value inside Some. None stays None.
    println!(
        "name length, user 1: {:?}",
        find_user(1).map(|u| u.name.len())
    );
    println!(
        "name length, user 3: {:?}",
        find_user(3).map(|u| u.name.len())
    );

    // map with a function that itself returns an Option nests them.
    println!("map to email, user 1: {:?}", find_user(1).map(|u| u.email));
    println!("map to email, user 2: {:?}", find_user(2).map(|u| u.email));

    // and_then flattens: one Option out, not two.
    println!(
        "and_then email, user 1: {:?}",
        find_user(1).and_then(|u| u.email)
    );
    println!(
        "and_then email, user 2: {:?}",
        find_user(2).and_then(|u| u.email)
    );

    // A chain of lookups: any step that gives None ends it with None.
    for id in [1, 2, 3] {
        let domain = find_user(id)
            .and_then(|u| u.email)
            .and_then(|email| email.split_once('@'))
            .map(|(_, domain)| domain);
        println!("domain, user {id}: {domain:?}");
    }
}
