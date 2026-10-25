use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Person {
    full_name: String,
    age: u32,
}

fn main() {
    let inputs = [
        r#"{ "fullName": "Ana", "age": 31, "nickname": "A" }"#, // unknown field
        r#"{ "fullName": "Bo", "age": "28" }"#,                  // a string, not a number
        r#"{ "fullName": "Cy", "age": -1 }"#,                    // negative for a u32
        r#"{ "fullName": "Di", "age": 1.5 }"#,                   // fraction for a u32
        r#"{ "fullName": "Ed", "age": 40"#,                      // cut off
    ];
    for text in inputs {
        match serde_json::from_str::<Person>(text) {
            Ok(p) => println!("ok: {} is {}", p.full_name, p.age),
            Err(e) => println!("error: {e}"),
        }
    }
}
