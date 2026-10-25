use std::mem::size_of;

#[derive(Clone, Copy, Debug)]
struct Id(usize);

struct Dept {
    name: String,
    parent: Option<Id>,
}

struct Org {
    depts: Vec<Dept>,
}

impl Org {
    fn add(&mut self, name: &str, parent: Option<Id>) -> Id {
        self.depts.push(Dept {
            name: name.to_string(),
            parent,
        });
        Id(self.depts.len() - 1)
    }

    fn rename(&mut self, id: Id, name: &str) {
        self.depts[id.0].name = name.to_string();
    }

    // Walk up the parents and join the names, root first.
    fn path(&self, id: Id) -> String {
        let mut names = vec![self.depts[id.0].name.as_str()];
        let mut at = self.depts[id.0].parent;
        while let Some(p) = at {
            names.push(&self.depts[p.0].name);
            at = self.depts[p.0].parent;
        }
        names.reverse();
        names.join(" / ")
    }
}

fn main() {
    let mut org = Org { depts: Vec::new() };
    let hq = org.add("HQ", None);
    let eng = org.add("Eng", Some(hq));
    let web = org.add("Web", Some(eng));
    for (i, d) in org.depts.iter().enumerate() {
        println!("depts[{i}] = {}, parent {:?}", d.name, d.parent);
    }
    println!("path: {}", org.path(web));

    // The parent can change while the children still point at it.
    org.rename(hq, "Acme");
    println!("after rename: {}", org.path(web));
    println!("size of Id: {} bytes", size_of::<Id>());

    // The price: an Id is only a number. Nothing says which Org it came from.
    let mut other = Org { depts: Vec::new() };
    other.add("Sales", None);
    other.add("Field", None);
    println!("eng's id in another org: {}", other.path(eng));
}
