use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    full_name: String,
    email: Option<String>, // missing or null -> None
    #[serde(default)]
    tags: Vec<String>, // missing -> Vec::default(), an empty list
}

fn show(p: &Person) {
    println!("{}: email={:?} tags={:?}", p.full_name, p.email, p.tags);
}

fn main() -> Result<(), serde_json::Error> {
    // "email" and "tags" are missing
    let a: Person = serde_json::from_str(r#"{ "fullName": "Ana" }"#)?;
    show(&a);
    // "email" is null
    let b: Person = serde_json::from_str(r#"{ "fullName": "Bo", "email": null }"#)?;
    show(&b);
    // a field the struct does not have
    let c: Person = serde_json::from_str(r#"{ "fullName": "Cy", "nickname": "C" }"#)?;
    show(&c);
    Ok(())
}
