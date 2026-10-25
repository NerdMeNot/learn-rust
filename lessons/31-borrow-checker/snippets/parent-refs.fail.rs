struct Dept<'a> {
    name: String,
    parent: Option<&'a Dept<'a>>,
}

fn main() {
    let mut hq = Dept {
        name: String::from("HQ"),
        parent: None,
    };
    let eng = Dept {
        name: String::from("Eng"),
        parent: Some(&hq),
    };
    hq.name = String::from("Acme");
    println!("{} reports to {}", eng.name, eng.parent.unwrap().name);
}
