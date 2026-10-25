use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    full_name: String,
    age: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ana = Person { full_name: "Ana Silva".to_string(), age: 31 };

    // the same struct, three formats out
    println!("{}", serde_json::to_string(&ana)?);
    println!("{}", toml::to_string(&ana)?);
    let mut writer = csv::Writer::from_writer(vec![]);
    writer.serialize(&ana)?;
    println!("{}", String::from_utf8(writer.into_inner()?)?);

    // and back in, from TOML and from CSV
    let from_toml: Person = toml::from_str("fullName = \"Bo Chen\"\nage = 28\n")?;
    println!("{from_toml:?}");
    let mut reader = csv::Reader::from_reader("fullName,age\nCy Diaz,40\n".as_bytes());
    for row in reader.deserialize() {
        let p: Person = row?;
        println!("{p:?}");
    }
    Ok(())
}
