use std::collections::HashMap;

struct Address {
    city: Option<String>,
}

struct User {
    address: Option<Address>,
}

// Kotlin: u?.address?.city?.length
fn city_len(u: Option<&User>) -> Option<usize> {
    let city = u?.address.as_ref()?.city.as_ref()?;
    Some(city.len())
}

fn main() {
    let ana = User {
        address: Some(Address {
            city: Some("Lyon".to_string()),
        }),
    };
    let bo = User {
        address: Some(Address { city: None }),
    };
    println!("{:?}", city_len(Some(&ana)));
    println!("{:?}", city_len(Some(&bo)));
    println!("{:?}", city_len(None));
    println!("{}", city_len(None).map_or(-1, |n| n as i32));
    let m: HashMap<&str, Option<&str>> = HashMap::from([("k", None)]);
    println!("{:?} {:?}", m.get("k"), m.get("z"));
    println!("{} {}", m.contains_key("k"), m.contains_key("z"));
}
