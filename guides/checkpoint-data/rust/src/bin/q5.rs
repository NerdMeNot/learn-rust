fn describe(id: Option<u32>, admin: bool) -> &'static str {
    match (id, admin) {
        (Some(_), true) => "admin",
        (Some(_), false) => "user",
        (None, true) => "root",
    }
}

fn main() {
    println!("{}", describe(Some(7), false));
}
