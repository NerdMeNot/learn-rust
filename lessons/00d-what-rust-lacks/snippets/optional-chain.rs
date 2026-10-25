struct User {
    name: String,
}

fn name_of(user: Option<User>) -> Option<String> {
    let name = user?.name;
    Some(name)
}

fn main() {
    println!("none: {:?}", name_of(None));
    let ada = User {
        name: "Ada".to_string(),
    };
    println!("some: {:?}", name_of(Some(ada)));
}
