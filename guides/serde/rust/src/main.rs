use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] // full_name <-> "fullName"
struct Person {
    full_name: String,
    age: u32,
    email: Option<String>, // may be missing or null
    #[serde(default)]
    tags: Vec<String>, // missing -> empty list
}

fn main() -> Result<(), serde_json::Error> {
    // JSON text -> struct
    let text = r#"{ "fullName": "Ana Silva", "age": 31, "tags": ["admin"] }"#;
    let ana: Person = serde_json::from_str(text)?;
    println!("{ana:?}");

    // struct -> JSON text
    let bo = Person {
        full_name: "Bo Chen".to_string(),
        age: 28,
        email: Some("bo@example.com".to_string()),
        tags: vec![],
    };
    println!("{}", serde_json::to_string_pretty(&bo)?);
    Ok(())
}
