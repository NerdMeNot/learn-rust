struct Dept {
    name: String,
    parent: Option<&Dept>,
}

fn main() {
    let hq = Dept {
        name: String::from("HQ"),
        parent: None,
    };
    println!("{}", hq.name);
}
