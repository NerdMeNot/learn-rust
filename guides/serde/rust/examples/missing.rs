use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    full_name: String,
    age: u32,
}

fn main() {
    let text = r#"{ "fullName": "Cy" }"#; // no "age"
    let result: Result<Person, serde_json::Error> = serde_json::from_str(text);
    match result {
        Ok(person) => println!("{} is {}", person.full_name, person.age),
        Err(e) => println!("error: {e}"),
    }
}
