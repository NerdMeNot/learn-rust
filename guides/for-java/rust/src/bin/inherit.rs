trait Describe {
    fn describe(&self) -> String;
}

struct User {
    name: String,
}

struct Admin {
    user: User,
}

impl Describe for User {
    fn describe(&self) -> String {
        format!("user {}", self.name)
    }
}

impl Describe for Admin {
    fn describe(&self) -> String {
        format!("admin {}", self.user.describe())
    }
}

fn show(thing: &dyn Describe) {
    println!("{}", thing.describe());
}

fn main() {
    let admin = Admin {
        user: User {
            name: String::from("ana"),
        },
    };
    show(&admin);
    println!("{}", admin.user.name);
}
