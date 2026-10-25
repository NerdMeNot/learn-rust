fn default_port() -> u16 {
    println!("  (default_port ran)");
    8080
}

fn main() {
    let configured: Option<u16> = Some(443);

    println!("unwrap_or:");
    let a = configured.unwrap_or(default_port());
    println!("  -> {a}");

    println!("unwrap_or_else:");
    let b = configured.unwrap_or_else(default_port);
    println!("  -> {b}");

    println!("unwrap_or_else with nothing configured:");
    let missing: Option<u16> = None;
    let c = missing.unwrap_or_else(|| default_port() + 1);
    println!("  -> {c}");
}
