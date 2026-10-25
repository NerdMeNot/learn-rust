use serde_json::{Value, json};

fn main() -> Result<(), serde_json::Error> {
    // no struct: parse into serde_json::Value and look things up by key
    let v: Value = serde_json::from_str(r#"{ "fullName": "Ana", "tags": ["admin", "dev"] }"#)?;
    println!("{}", v["fullName"]);
    println!("{}", v["tags"][1]);
    println!("{}", v["nope"]); // a missing key is Value::Null, not an error
    println!("{:?}", v["fullName"].as_str());
    println!("{:?}", v["fullName"].as_u64());

    // build JSON with the json! macro
    let out = json!({ "ok": true, "count": 2, "names": ["Ana", "Bo"] });
    println!("{out}");
    Ok(())
}
