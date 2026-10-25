struct Session {
    user: String,
}

// A type with its own Drop can't be taken apart field by field: its drop()
// would run on a value with a hole in it.
impl Drop for Session {
    fn drop(&mut self) {
        println!("closing the session for {}", self.user);
    }
}

fn main() {
    let session = Session {
        user: String::from("ana"),
    };
    let user = session.user;
    println!("{user}");
}
