use serde::{Deserialize, Serialize};

// the default: the variant name wraps the data
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Wrapped {
    Circle { radius: f64 },
    Square { side: f64 },
}

// tag = "type": the variant name becomes a field inside the object
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum Tagged {
    Circle { radius: f64 },
    Square { side: f64 },
}

fn main() -> Result<(), serde_json::Error> {
    println!("{}", serde_json::to_string(&Wrapped::Circle { radius: 1.5 })?);
    println!("{}", serde_json::to_string(&Tagged::Circle { radius: 1.5 })?);

    let shapes: Vec<Tagged> =
        serde_json::from_str(r#"[{"type":"circle","radius":2.0},{"type":"square","side":3.0}]"#)?;
    println!("{shapes:?}");

    // an unknown tag is an error, not a guess
    let err = serde_json::from_str::<Tagged>(r#"{"type":"triangle","side":3.0}"#).unwrap_err();
    println!("error: {err}");
    Ok(())
}
