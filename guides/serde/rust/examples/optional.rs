use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    full_name: String,
    age: Option<u32>, // say it out loud: age may be missing
}

fn main() {
    let cy: Person = serde_json::from_str(r#"{ "fullName": "Cy" }"#).unwrap();
    match cy.age {
        Some(age) => println!("{} is {age}", cy.full_name),
        None => println!("{} didn't tell us their age", cy.full_name),
    }
}
