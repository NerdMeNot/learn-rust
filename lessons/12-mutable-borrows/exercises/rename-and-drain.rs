struct Profile {
    name: String,
    tags: Vec<String>,
}

/// Give the profile a new name and return the old one.
fn rename(p: &mut Profile, new_name: String) -> String {
    todo!("std::mem::replace puts a new value in and gives the old one back")
}

/// Remove every tag and return them. Afterwards `p.tags` is an empty Vec.
fn take_tags(p: &mut Profile) -> Vec<String> {
    todo!("std::mem::take leaves an empty value behind")
}

fn main() {
    let mut p = Profile {
        name: String::from("Ada"),
        tags: vec![String::from("admin"), String::from("beta")],
    };
    let ada_text = p.name.as_ptr();
    let new_name = String::from("Grace");
    let grace_text = new_name.as_ptr();

    let old = rename(&mut p, new_name);
    assert_eq!(old, "Ada");
    assert_eq!(
        old.as_ptr(),
        ada_text,
        "return the old String, do not copy it"
    );
    assert_eq!(p.name, "Grace");
    assert_eq!(
        p.name.as_ptr(),
        grace_text,
        "store the new String, do not copy it"
    );

    let admin_text = p.tags[0].as_ptr();
    let tags = take_tags(&mut p);
    assert_eq!(tags, ["admin", "beta"]);
    assert_eq!(
        tags[0].as_ptr(),
        admin_text,
        "hand over the tags, do not copy them"
    );
    assert!(p.tags.is_empty());

    // Taking from an empty list is fine, and the profile still works.
    assert!(take_tags(&mut p).is_empty());
    p.tags.push(String::from("new"));
    assert_eq!(p.tags, ["new"]);

    println!("{}: all checks passed", p.name);
}
