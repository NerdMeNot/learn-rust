use std::collections::HashMap;

// A HashMap is Rust's map type (Maps is a later lesson). get returns an Option.
fn port_from(settings: &HashMap<&str, &str>) -> Result<u16, String> {
    let text = settings
        .get("port")
        .ok_or_else(|| "no 'port' setting".to_string())?; // Option -> Result
    let port: u16 = text.parse().map_err(|e| format!("port {text:?}: {e}"))?; // change the error
    if port < 1024 {
        return Err(format!("port {port} needs special rights"));
    }
    Ok(port)
}

// and_then: run a second step that can also fail, only if the first worked.
fn half(n: u32) -> Result<u32, String> {
    if n % 2 == 0 {
        Ok(n / 2)
    } else {
        Err(format!("{n} is odd"))
    }
}

fn main() {
    let mut settings = HashMap::new();
    println!("empty: {:?}", port_from(&settings));
    settings.insert("port", "http");
    println!("http: {:?}", port_from(&settings));
    settings.insert("port", "80");
    println!("80: {:?}", port_from(&settings));
    settings.insert("port", "8080");
    println!("8080: {:?}", port_from(&settings));

    println!("half(8): {:?}", half(8));
    println!("half(8) then half: {:?}", half(8).and_then(half));
    println!("half(6) then half: {:?}", half(6).and_then(half));
    println!("half(7) then half: {:?}", half(7).and_then(half));
}
