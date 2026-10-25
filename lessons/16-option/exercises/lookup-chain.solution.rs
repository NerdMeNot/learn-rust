/// The owner of a pet, if we know one.
fn owner(pet: &str) -> Option<&'static str> {
    match pet {
        "rex" => Some("ana"),
        "tom" => Some("bo"),
        "fido" => Some("cy"),
        _ => None,
    }
}

/// The city a person lives in, if we know it.
fn city(person: &str) -> Option<&'static str> {
    match person {
        "ana" => Some("Lisbon"),
        "bo" => Some("Oslo"),
        _ => None,
    }
}

/// "rex lives in Lisbon" when the pet's owner and the owner's city are both known,
/// and "rex: unknown" when either is missing.
fn pet_location(pet: &str) -> String {
    owner(pet)
        .and_then(city)
        .map(|city| format!("{pet} lives in {city}"))
        .unwrap_or_else(|| format!("{pet}: unknown"))
}

fn main() {
    assert_eq!(pet_location("rex"), "rex lives in Lisbon");
    assert_eq!(pet_location("tom"), "tom lives in Oslo");
    // fido has an owner, but the owner has no known city.
    assert_eq!(pet_location("fido"), "fido: unknown");
    // ghost has no owner at all.
    assert_eq!(pet_location("ghost"), "ghost: unknown");
    assert_eq!(pet_location(""), ": unknown");

    println!("all checks passed");
}
