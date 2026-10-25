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
    // ok_or: turn None into an error value, so the caller gets a reason.
    let found: Result<User, &str> = find_user(1).ok_or("no such user");
    let lost: Result<User, &str> = find_user(3).ok_or("no such user");
    println!("ok_or, user 1: {:?}", found.map(|u| u.name));
    println!("ok_or, user 3: {:?}", lost.map(|u| u.name));

    // ok_or_else builds the error only if it is needed.
    let id = 9;
    let err = find_user(id)
        .map(|u| u.name)
        .ok_or_else(|| format!("no user with id {id}"));
    println!("ok_or_else, user 9: {err:?}");

    // or: use this Option, else that one. or_else only builds the other when needed.
    let fallback = Some("noreply@example.com");
    let own = |id: u32| find_user(id).and_then(|u| u.email);
    println!("or, user 1: {:?}", own(1).or(fallback));
    println!("or, user 2: {:?}", own(2).or(fallback));
    println!(
        "or_else, user 2: {:?}",
        own(2).or_else(|| Some("noreply@example.com"))
    );
}
