use std::fmt;

struct Account {
    id: u32,
    owner: String,
    password: String,
}

impl fmt::Debug for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Account")
            .field("id", &self.id)
            .field("owner", &self.owner)
            .finish_non_exhaustive()
    }
}

fn main() {
    let a = Account {
        id: 7,
        owner: "ada".to_string(),
        password: "hunter2".to_string(),
    };
    println!("{:?}", a);
    println!("{:#?}", a);
    println!(
        "password printed: {}",
        format!("{a:?}").contains(&a.password)
    );
}
