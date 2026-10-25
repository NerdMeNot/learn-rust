struct User {
    email: Option<&'static str>,
    age: Option<u32>,
}

fn find_user(id: u32) -> Option<User> {
    match id {
        1 => Some(User {
            email: Some("ana@example.com"),
            age: Some(31),
        }),
        2 => Some(User {
            email: None,
            age: Some(15),
        }),
        _ => None,
    }
}

fn main() {
    // filter: keep the value only if the test says yes.
    let adult_1 = find_user(1).and_then(|u| u.age).filter(|age| *age >= 18);
    let adult_2 = find_user(2).and_then(|u| u.age).filter(|age| *age >= 18);
    println!("adult age, user 1: {adult_1:?}");
    println!("adult age, user 2: {adult_2:?}");

    // zip: Some of both, or None if either is missing.
    println!("zip, 1 and 'a': {:?}", Some(1).zip(Some("a")));
    println!("zip, 1 and nothing: {:?}", Some(1).zip(None::<&str>));
    if let Some(user) = find_user(1) {
        println!("age and email, user 1: {:?}", user.age.zip(user.email));
    }
    if let Some(user) = find_user(2) {
        println!("age and email, user 2: {:?}", user.age.zip(user.email));
    }
}
