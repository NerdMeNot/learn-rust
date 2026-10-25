struct Team {
    names: Vec<String>,
}

impl Team {
    fn names(&self) -> &[String] {
        &self.names
    }
}

fn main() {
    let team = Team {
        names: vec![String::from("ana"), String::from("bo")],
    };
    let names = team.names();
    names[0] = String::from("changed");
}
