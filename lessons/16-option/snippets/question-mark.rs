struct User {
    email: Option<&'static str>,
}

fn find_user(id: u32) -> Option<User> {
    match id {
        1 => Some(User {
            email: Some("ana@example.com"),
        }),
        2 => Some(User { email: None }),
        _ => None,
    }
}

// Each ? means: if this is None, return None from the whole function now.
fn email_domain(id: u32) -> Option<&'static str> {
    let user = find_user(id)?;
    let email = user.email?;
    let (_, domain) = email.split_once('@')?;
    Some(domain)
}

fn main() {
    for id in [1, 2, 3] {
        println!("domain, user {id}: {:?}", email_domain(id));
    }
}
