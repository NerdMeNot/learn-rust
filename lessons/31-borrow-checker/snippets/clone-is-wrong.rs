struct Session {
    token: Option<String>,
}

impl Session {
    // The compiler's `help:` suggestion for E0507: it compiles, and it is not a logout.
    fn logout(&mut self) -> Option<String> {
        self.token.clone()
    }
}

fn main() {
    let mut session = Session {
        token: Some(String::from("abc123")),
    };
    println!("logout: {:?}", session.logout());
    println!("token afterwards: {:?}", session.token);
}
